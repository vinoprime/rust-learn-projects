use actix_web::{get, web, App, HttpResponse, HttpServer, Responder,Result};
use serde::{Deserialize, Serialize};

use rand::distributions::{Distribution, Uniform};
use rand::Rng;
// Define a struct to deserialize the incoming JSON data
#[derive(Debug, Deserialize)]
struct PostData {
    title: String,
    body: String,
}

// Define a struct to serialize the response data
#[derive(Debug, Serialize, Deserialize)]
struct PostResponse {
    id: usize,
    title: String,
    body: String,
}

async fn create_post(data: web::Json<PostData>) -> Result<HttpResponse> {

    println!("jhgfh");

    let mut rng = rand::thread_rng();
    let range = Uniform::new_inclusive(1, 100);
    let random_number = rng.sample(range);
    println!("{:?}",random_number);

    // Process the incoming JSON data and create a new post
    let post = PostResponse {
        id: random_number,
        title: data.title.clone(),
        body: data.body.clone(),
    };

    // Serialize the response data and return it as JSON
    Ok(HttpResponse::Ok().json(post))
}


// #[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}


async fn get_post_by_id(info: web::Path<usize>) -> Result<HttpResponse> {
    // Serialize the post and return it as JSON
    Ok(HttpResponse::Ok().json({"df".to_owned()}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
        .service(web::resource("/api")
        .route(web::post().to(create_post))
        .route(web::get().to(index)))
        .service(web::resource("/api/{id}")
        .route(web::get().to(get_post_by_id)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}