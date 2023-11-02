use actix_web::{ ResponseError };

use reqwest;
use std::fmt;

pub mod log;

#[derive(Debug)]
pub enum CustomError {
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
