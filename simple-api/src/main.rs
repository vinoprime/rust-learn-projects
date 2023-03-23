use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Ok")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    println!("Hello Rust");

    HttpServer::new(|| App::new().service(health_check))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
