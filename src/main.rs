use tonic::{ transport::Server, Request, Response, Status };

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
        println!("Got a req {:?}", req);

        let re = req.into_inner();
        let rply = BtcPaymentResponse {
            successful: true,
            message: format!("Sent {}BTC to {}.", re.amount, re.to_addr).into(),
        };

        Ok(Response::new(rply))
    }
}

mod basics;
mod mini_projects;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    // basics::main();
    // mini_projects::main();

    let addr = "[::1]:50051".parse()?;

    let btc_service: BitcoinService = BitcoinService::default();

    Server::builder().add_service(BitcoinServer::new(btc_service)).serve(addr).await?;

    Ok(())
}
