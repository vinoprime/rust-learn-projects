use axum::routing::get;
use axum::{Router, ServiceExt};
use axum_server::tls_rustls::RustlsConfig;
use rcgen::generate_simple_self_signed;
use std::fs::File;
use std::io::Write;
use std::net::SocketAddr;
use std::path::PathBuf;

fn generate_self_signed_ssl_certificate() {
    let names = vec![String::from("localhost")];
    let x = String::from("localhost");

    let certificate_key = generate_simple_self_signed(x).unwrap();

    let certificate = certificate_key.cert;
    let key = certificate_key.signing_key;

    let pem = certificate.pem();
    let key = key.serialize_pem();

    let pem_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("cert.pem");

    let key_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("key.pem");

    let mut pem_file = File::create(pem_path).unwrap();
    let mut key_file = File::create(key_path).unwrap();

    pem_file.write_all(pem.as_bytes()).unwrap();
    key_file.write_all(key.as_bytes()).unwrap();
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    // generate_self_signed_ssl_certificate();


    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("failed to install");

    let cert = PathBuf::from((env!("CARGO_MANIFEST_DIR"))).join("cert.pem");
    let key = PathBuf::from((env!("CARGO_MANIFEST_DIR"))).join("key.pem");

    let config = RustlsConfig::from_pem_file(cert, key).await.unwrap();

    let handle = axum_server::Handle::new();

    let app = Router::new().route("/", get(hello_world_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    println!("Listening on {}", addr);

    axum_server::bind_rustls(addr, config)
        .handle(handle)
        .serve(app.into_make_service())
        .await
        .unwrap();

}

async fn hello_world_handler() -> &'static str {
    "Hello... I am SSL"
}
