use actix_web::{ web, App, HttpResponse, HttpServer, Responder };
use tonic::{ transport::Server, Request, Response, Status };
use tokio::runtime::Runtime;
use std::thread;
use std::time::Duration;
use tonic::transport::Channel;

// For gRPC server
use payments::bitcoin_server::{ Bitcoin, BitcoinServer };
use payments::{ BtcPaymentResponse, BtcPaymentRequest };

pub mod payments {
    tonic::include_proto!("payments");
}

#[derive(Debug, Default)]
pub struct BitcoinService {}

#[tonic::async_trait]
impl Bitcoin for BitcoinService {
    async fn send_payment(
        &self,
        req: Request<BtcPaymentRequest>
    ) -> Result<Response<BtcPaymentResponse>, Status> {
        // println!("Got a req {:?}", req);
        println!("Got request");

        let re = req.into_inner();
        let rply = BtcPaymentResponse {
            successful: true,
            message: format!("Sent {}BTC to {}.", re.amount, re.to_addr).into(),
        };

        Ok(Response::new(rply))
    }
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("grpc");

    let rt = Runtime::new().unwrap();
    rt.spawn(async move {
        // everything in here runs in a separate thread
        let grpc_addr = "[::1]:50051".parse().unwrap();
        println!("gRPC Server running at http://{}", grpc_addr);
        let btc_service: BitcoinService = BitcoinService::default();

        if
            let Err(e) = Server::builder()
                .add_service(BitcoinServer::new(btc_service))
                .serve(grpc_addr).await
        {
            eprintln!("gRPC Server error: {}", e);
        }
    });

    let client_rt = Runtime::new().unwrap();
    client_rt.spawn(async move {
        // For gRPC client
        // everything in here runs in a separate tokio runtime
        use payments::bitcoin_client::BitcoinClient;

        let mut client = BitcoinClient::connect("http://[::1]:50051").await.unwrap();

        for i in 1..=5 {
            let req = tonic::Request::new(BtcPaymentRequest {
                from_addr: "123456".to_owned(),
                to_addr: "1789".to_owned(),
                amount: i,
            });

            println!("{:?}", req);
            let res = client.send_payment(req).await.unwrap();
            println!("Response : {:?}", res);
        }
    });

    // create a thread
    // thread::spawn(|| {
    //     println!("Thread called");
    //     // everything in here runs in a separate thread
    //     // for i in 1..=5 {
    //     //     println!("hi number {} from the spawned thread!", i);
    //     //     thread::sleep(Duration::from_millis(1));
    //     // }
    // });

    // let mut client = BitcoinClient::connect("http://[::1]:50051").await?;
    // let req = tonic::Request::new(BtcPaymentRequest {
    //     from_addr: "123456".to_owned(),
    //     to_addr: "1789".to_owned(),
    //     amount: 100,
    // });

    // println!("{:?}", req);
    // let res = client.send_payment(req).await?;
    // println!("Response : {:?}", res);

    let addr = "0.0.0.0:8080";
    println!("Server running at http://{}", addr);
    let _ = HttpServer::new(move || {
        App::new().route("/", web::get().to(index)).route("/hey", web::get().to(manual_hello))
    })
        .bind(addr)?
        .run().await?;

    Ok(())
}
