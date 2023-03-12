use actix_web::{web, HttpResponse, HttpRequest, Result};


pub async fn auth(req: HttpRequest, payload: web::Payload) -> Result<web::HttpResponse> {
    println!("Middleware: Request received for URL '{}'", req.uri());

    // Perform some action on the request or payload here, if needed

    let response = web::HttpResponse::Ok()
        .content_type("text/plain")
        .body("Hello from middleware!");

    Ok(response)
}
