use axum::{
    routing::post, Extension, Json, Router
};
use pkcs8::DecodePrivateKey;
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
    let config = load_config();
    let state_config = Arc::new(Mutex::new(config.clone()));

    if config.hsm {
        eprintln!("HSM init");
        let state_session = Arc::new(Mutex::new(sign_hsm::connect()));

        let app = Router::new()
            .route("/s", post(sign_hsm::_show))
            .route("/sign", post(sign_hsm::sign))
            .route("/check", post(sign_hsm::check))
            .layer(Extension(state_session))
            .layer(Extension(state_config));

        let listener = tokio::net::TcpListener::bind(config.addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    } else {
        eprintln!("no HSM init");
        let mut file = File::open(config.key_file).unwrap();
        let mut key_data = String::from("");
        file.read_to_string(&mut key_data).unwrap();
        let siging_key = ed25519_dalek::SigningKey::from_pkcs8_pem(&key_data).unwrap();
        let state: Arc<Mutex<[u8; 32]>> = Arc::new(Mutex::new(siging_key.to_bytes()));

        let app = Router::new()
            // .route("/kg", post(sign::_keygen))
            .route("/sign", post(sign::sign))
            .route("/check", post(sign::check))
            .route("/reload", post(reload_key))
            .layer(Extension(state));

        let listener = tokio::net::TcpListener::bind(config.addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }
}

fn load_config() -> structures::Config {
    let mut file = File::open("config.yaml").unwrap();
    let mut config = String::new();
    file.read_to_string(&mut config).unwrap();
    serde_yaml::from_str(&config).unwrap()
}

async fn reload_key(state: Extension<Arc<Mutex<[u8; 32]>>>) {
    let config = load_config();
    let mut file = File::open(config.key_file).unwrap();
    let mut key_data = String::from("");
    file.read_to_string(&mut key_data).unwrap();

    let siging_key = ed25519_dalek::SigningKey::from_pkcs8_pem(&key_data).unwrap();

    let mut new_state = state.lock().await;
    *new_state = siging_key.to_bytes();
}  