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

#[tokio::main]
async fn main() {

    let mut file = File::open("signing_key.bin").unwrap();
    let mut key_data = Vec::new();
    file.read_to_end(&mut key_data).unwrap();

    let state = Arc::new(Mutex::new(sign_hsm::connect()));
    
    let app = Router::new()
        // .route("/keygen", post(sign::_keygen))
        .route("/signh", post(sign_hsm::sign))
        .route("/checkh", post(sign_hsm::check))
        .route("/sign", post({
            let key_data = key_data.clone(); // Clone la clé pour la route
            move |payload| sign::sign(payload, key_data.clone()) // Capturer key_data dans la closure
        }))
        .route("/check", post({
            let key_data = key_data.clone(); // Clone la clé pour la route
            move |payload| sign::check(payload, key_data.clone()) // Capturer key_data dans la closure
        }))
        .layer(Extension(state));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}