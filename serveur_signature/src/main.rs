use axum::{
    routing::post,
    Router, Json
};
use hex::{encode, decode};

mod sign;
mod structures;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/keygen", post(sign::keygen))
        .route("/sign", post(sign::sign))
        .route("/check", post(sign::check));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}