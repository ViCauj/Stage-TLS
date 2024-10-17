use axum::{
    routing::post, Json, Router
};
use std::collections::HashMap;

mod structures;
mod base;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/check_user", post(base::check_user))
        .route("/register", post(base::register))
        .route("/suppr_user", post(base::suppr_user))
        .route("/check_session", post(base::check_session))
        .route("/init_session", post(base::init_session))
        .route("/premier_message", post(base::premier_message))
        .route("/fetch_messages_recus", post(base::fetch_messages_recus))
        .route("/send", post(base::send));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}