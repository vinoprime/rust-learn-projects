use chrono::{ Duration, Utc };
use jsonwebtoken::{
    decode,
    encode,
    errors::Result as JwtResult,
    DecodingKey,
    EncodingKey,
    Header,
    Validation,
};
use serde_derive::{ Deserialize, Serialize };

// Define the payload struct
#[derive(Debug, Serialize, Deserialize)]
struct Payload {
    sub: String,
    exp: i64,
    iat: i64,
}

// Define a function to generate an access token
pub fn generate_access_token(
    secret_key: &[u8],
    user_id: &str,
    expire_time: Duration
) -> JwtResult<String> {
    let now = Utc::now().timestamp();
    let payload = Payload {
        sub: user_id.to_owned(),
        iat: now,
        exp: now + expire_time.num_seconds(),
    };
    let header = Header::default();
    encode(&header, &payload, &EncodingKey::from_secret(secret_key))
}

// Define a function to generate a refresh token
fn generate_refresh_token(
    secret_key: &[u8],
    user_id: &str,
    expire_time: Duration
) -> JwtResult<String> {
    let now = Utc::now().timestamp();
    let payload = Payload {
        sub: user_id.to_owned(),
        iat: now,
        exp: now + expire_time.num_seconds(),
    };
    let header = Header::default();
    encode(&header, &payload, &EncodingKey::from_secret(secret_key))
}

// Define a function to decode a token
pub fn decode_token<T: serde::de::DeserializeOwned>(token: &str, secret_key: &[u8]) -> JwtResult<T> {
    decode::<T>(token, &DecodingKey::from_secret(secret_key), &Validation::default()).map(
        |token_data| token_data.claims
    )
}

pub fn my_function() {
    println!("Hello, world! from module");
}
