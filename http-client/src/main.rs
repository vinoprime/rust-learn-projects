use actix_web::{ get, web, App, HttpServer, HttpResponse, Responder, middleware::Logger };
use chrono::{ Duration };
use reqwest;

mod auth;
mod exception;
mod electronics;
mod mathematics;

use exception::log::AppLogging;
use exception::CustomError;

#[get("/")]
async fn index() -> Result<HttpResponse, CustomError> {
    let client = reqwest::Client::new();
    let response = client.get("http://localhost:8080/hello/rust").send().await?;
    let body = response.text().await?;
    let resistors = [10.0, 20.0, 30.0]; // example array
    let t = electronics::calculate_parallel_resistance(&resistors);
    println!("{:?}", t);



    let f = |x: f64| x.powi(2); // f(x) = x^2

    let x = 1.0; // Point to estimate derivative
    let h = 2.0; // Step size

    let dydx = mathematics::derivative(f, x, h); 

    println!("dy/dx at x={} is {}", x, dydx);


    Ok(HttpResponse::Created().body(body))
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    let name_string = name.to_string();
    if name_string == "xyz" {
        return format!("Hello");
    }
    format!("Hello {}!", name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    // logging();
    auth::my_function();

    let two_hours = Duration::hours(2);
    let secret_key = b"secret";
    let response = auth::generate_access_token(secret_key, "1", two_hours);
    match response {
        Ok(token) => {
            // token is the extracted string from the Ok variant
            println!("Token received: {}", token);
            let decoded = auth::decode_token::<()>(&token, secret_key);
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
