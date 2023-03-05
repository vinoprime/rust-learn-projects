use dotenv::dotenv;
use hyper::body::Buf;
use hyper::{header,Body,Client,Request};
use hyper_tls::HttpsConnector;
use serde_derive::{Serialize, Deserialize};
use spinners::{Spinner,Spinners};
use std::env;
use std::io::{stdin,stdout};





fn main() {
    println!("Hello, world!");
}
