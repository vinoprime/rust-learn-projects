use actix_web::{ web, App, HttpResponse, HttpServer, Responder };


async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    println!("grpc");


    let addr = "0.0.0.0:8080";

    println!("Server running at http://{}", addr);

    let _ = HttpServer::new(move || {
        App::new().route("/", web::get().to(index)).route("/hey", web::get().to(manual_hello))
    })
        .bind(addr)?
        .run().await;


    Ok(())
}
