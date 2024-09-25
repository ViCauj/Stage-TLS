use axum::{
    routing::post, 
    Json, Router, Extension
};
use std::{
    fs::File,
    io::Read,
    sync::Arc
};
use tokio::sync::Mutex;
use hex::{encode, decode};

mod sign;
mod sign_hsm;
mod structures;

const HSM: bool = false; 
const _KEY_ID: u8 = 1;
const ADDR: &str = "0.0.0.0:3000";

#[tokio::main]
async fn main() {
    if HSM {
        let state = Arc::new(Mutex::new(sign_hsm::connect()));

        let app = Router::new()
            .route("/sign", post(sign_hsm::sign))
            .route("/check", post(sign_hsm::check))
            .layer(Extension(state));

        let listener = tokio::net::TcpListener::bind(ADDR).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    } else {
        let mut file = File::open("signing_key.bin").unwrap();
        let mut key_data = Vec::new();
        file.read_to_end(&mut key_data).unwrap();
        let state = Arc::new(Mutex::new(<Vec<u8> as TryInto<[u8; 32]>>::try_into(key_data).unwrap()));

        let app = Router::new()
            .route("/sign", post(sign::sign))
            .route("/check", post(sign::check))
            .layer(Extension(state));

        let listener = tokio::net::TcpListener::bind(ADDR).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }
}