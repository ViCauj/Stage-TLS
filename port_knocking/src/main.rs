use std::{
    fs,
    process::Command,
    sync::Arc
};
use axum::{
    routing::post, Extension, Json, Router
};
use structures::Input;
use tokio::sync::Mutex;

mod structures;

#[tokio::main]
async fn main() {
    let allowed_pub_file = fs::read_to_string(&"./allowed_pub.json").unwrap();
    let allowed_pub: structures::AllowedPub = serde_json::from_str(&allowed_pub_file).unwrap();
    let state = Arc::new(Mutex::new(allowed_pub.allowed_pub));

    let app = Router::new()
        .route("/add", post(add))
        .layer(Extension(state));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

}

async fn add(state: Extension<Arc<Mutex<Vec<String>>>>, Json(payload): Json<Input>) {
    let allowed_pub = state.lock().await;
    if allowed_pub.contains(&payload.key_pub) {
        Command::new("sudo")
        .args(["wg", "set", "wg0", "peer", &payload.key_pub, "allowed-ips", &payload.allowed_ip])
        .output().unwrap();    
    };
}

// faire serveur udp

// use std::net::UdpSocket;
// use std::str;

// fn main() -> std::io::Result<()> {
//     // Créer un socket UDP lié à l'adresse "127.0.0.1:8080"
//     let socket = UdpSocket::bind("127.0.0.1:8080")?;
//     println!("Serveur UDP démarré sur 127.0.0.1:8080");

//     // Buffer pour stocker les données reçues
//     let mut buf = [0; 1024];

//     // Boucle principale du serveur
//     loop {
//         match socket.recv_from(&mut buf) {
//             Ok((nb_bytes, src_addr)) => {
//                 // Convertir les bytes reçus en string
//                 if let Ok(message) = str::from_utf8(&buf[..nb_bytes]) {
//                     println!("Message reçu de {}: {}", src_addr, message);

//                     // Répondre au client
//                     let response = format!("Reçu: {}", message);
//                     socket.send_to(response.as_bytes(), src_addr)?;
//                 }

//                 // Réinitialiser le buffer
//                 buf = [0; 1024];
//             }
//             Err(e) => {
//                 eprintln!("Erreur lors de la réception: {}", e);
//             }
//         }
//     }
// }


// echo "Hello" | nc -u 127.0.0.1 8080