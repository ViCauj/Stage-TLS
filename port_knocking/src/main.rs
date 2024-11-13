#![allow(deprecated)]

use std::{
    net::UdpSocket,
    process::Command,
    collections::HashMap,
    sync::{Mutex, Arc},
    thread,
    time::Duration,
    fs
};
use time::Instant;

mod structures;

fn main() {
    let allowed_pub_file = fs::read_to_string(&"./allowed_pub.json").unwrap();
    let allowed_pub: structures::AllowedPub = serde_json::from_str(&allowed_pub_file).unwrap();
    println!("{:?}", allowed_pub.allowed_pub);

    let socket = UdpSocket::bind("0.0.0.0:3000").unwrap();
    socket.set_nonblocking(true).unwrap();
    let mut buffer = [0; 1024];

    let hashmap_instants: Arc<Mutex<HashMap<String, Instant>>> = Arc::new(Mutex::new(HashMap::new()));
    let hashmap_instants_clone = Arc::clone(&hashmap_instants);

    thread::spawn(move || {
        loop {
            if let Ok(mut hashmap_instants) = hashmap_instants_clone.try_lock() {
                match cherche_socket(&allowed_pub, &socket, &mut buffer) {
                    Ok((pub_key, instant)) => hashmap_instants.insert(pub_key, instant),
                    Err(e) => {
                        if e != String::from("Erreur de réception UDP: Resource temporarily unavailable (os error 11)") {
                            println!("{}", e);
                        }
                        None
                    },
                };
            }
            thread::sleep(Duration::from_millis(1000));
        }
    });

    loop {
        if let Ok(mut hashmap_instants) = hashmap_instants.try_lock() {
            for (pub_key, instant) in hashmap_instants.iter() {
                println!("{}: {}", pub_key, instant.elapsed());
            }
            hashmap_instants.retain(|_, instant| instant.elapsed() <= Duration::from_secs(10));
        }
        thread::sleep(Duration::from_millis(500));
    }
}

fn cherche_socket(allowed_pub: &structures::AllowedPub, socket: &UdpSocket, buffer: &mut [u8; 1024]) -> Result<(String, Instant), String> {
    match socket.recv_from(buffer) {
        Ok((taille, _)) => {
            let message = String::from_utf8_lossy(&buffer[0..taille]);
            match serde_json::from_str(&message) {
                Ok(input) => {
                    return add(allowed_pub.allowed_pub.clone(), input);
                }
                Err(e) => {
                    return Err(format!("Erreur de sérialisation JSON: {}", e).to_string());
                }
            };
        }
        Err(e) => {
            return Err(format!("Erreur de réception UDP: {}", e).to_string());
        }
    };
}

fn add(allowed_pub: Vec<String>, input: structures::Input) -> Result<(String, Instant), String> {
    if allowed_pub.contains(&input.key_pub) {
        Command::new("sudo")
        .args(["wg", "set", "wg0", "peer", &input.key_pub, "allowed-ips", &input.allowed_ip])
        .output().unwrap();   
        return Ok((input.key_pub.clone(), Instant::now()));
    } else {
        return Err("Erreur: la clef publique n'est pas dans la base de données".to_string());
    };
}


// REMARQUE : J'AI SURCOMPLIQUE CE CODE CAR JE NE CONNAISSAIS PAS L'OPTION "set_nonblocking", CA NE SERT A RIEN DE FAIRE DU MULTITHREAD