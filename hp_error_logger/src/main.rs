use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use futures::FutureExt;
use serde::Serialize;
use std::env;

#[path = "./db/mongo_db.rs"]
mod mongo_db;

#[path = "./middlewares/auth_middleware.rs"]
mod my_middleware;

#[derive(Serialize)]
struct ResponseData {
    list: Vec<String>,
}

// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn post_log_data(req_body: String) -> impl Responder {
    HttpResponse::Ok().json(req_body)
}

async fn manual_hello() -> impl Responder {
    dotenv().ok();
    let MONGO_DB_URL = env::var("MONGO_DB_URL").unwrap();
    let DB_NAME = env::var("DB_NAME").unwrap();

    let client = mongo_db::connect(MONGO_DB_URL)
        .await
        .expect("Some error message");
    let dbList = mongo_db::gell_all_db(&client)
        .await
        .expect("Some error message");

    let response_data = ResponseData { list: dbList };

    HttpResponse::Ok().json(response_data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // dotenv().ok();
    // let MONGO_DB_URL = env::var("MONGO_DB_URL").unwrap();
    // let DB_NAME = env::var("DB_NAME").unwrap();

    // let client = mongo_db::connect(MONGO_DB_URL).await.expect("Some error message");
    // let dbList = mongo_db::gell_all_db(&client).await.expect("Some error message");

    HttpServer::new(|| {
        App::new()
            // .wrap_fn(my_middleware::auth())
            // .service(hello)
            // .service(echo)
            .route("/", web::get().to(manual_hello))
            .route("/", web::post().to(post_log_data))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
