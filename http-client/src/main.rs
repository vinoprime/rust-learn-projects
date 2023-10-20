use actix_web::{
    get,
    web,
    App,
    HttpServer,
    HttpResponse,
    ResponseError,
    Responder,
    middleware::Logger,
};
use std::fmt;
use chrono::{ Duration };
use reqwest;

mod Auth;

#[derive(Debug)]
enum CustomError {
    ReqwestError(reqwest::Error),
}

impl ResponseError for CustomError {}

impl From<reqwest::Error> for CustomError {
    fn from(error: reqwest::Error) -> Self {
        CustomError::ReqwestError(error)
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomError::ReqwestError(e) => write!(f, "Reqwest error: {}", e),
        }
    }
}

#[get("/")]
async fn index() -> Result<HttpResponse, CustomError> {
    let client = reqwest::Client::new();
    let response = client.get("http://localhost:8080/hello/rust").send().await?;
    let body = response.text().await?;

    // println!("{:?}", &body);
    Ok(HttpResponse::Created().body(body))
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    Auth::my_function();

    let two_hours = Duration::hours(2);
    let secret_key = b"secret";
    let response = Auth::generate_access_token(secret_key, "1", two_hours);
    match response {
        Ok(token) => {
            // token is the extracted string from the Ok variant
            println!("Token received: {}", token);
            let decoded = Auth::decode_token::<()>(&token, secret_key);
            match decoded {
                Ok(d) => println!("{:?}", d),
                Err(er) => println!("{:?}", er),
            }
            // println!("Hello, world! {:?}", decoded);
        }
        Err(err) => {
            // handle the error
            println!("Error: {}", err);
        }
    }

    let _ = HttpServer::new(|| { App::new().wrap(Logger::default()).service(greet).service(index) })
        .bind(("0.0.0.0", 8080))?
        .run().await;

    Ok(())
}
