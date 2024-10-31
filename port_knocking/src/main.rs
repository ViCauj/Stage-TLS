use std::{
    net::UdpSocket,
    process::Command,
    fs
};

mod structures;

fn main() -> std::io::Result<()> {
    let allowed_pub_file = fs::read_to_string(&"./allowed_pub.json").unwrap();
    let allowed_pub: structures::AllowedPub = serde_json::from_str(&allowed_pub_file).unwrap();
    println!("{:?}", allowed_pub.allowed_pub);

    let socket = UdpSocket::bind("0.0.0.0:3000")?;
    let mut buffer = [0; 1024];

    loop {
        match socket.recv_from(&mut buffer) {
            Ok((taille, _)) => {
                let message = String::from_utf8_lossy(&buffer[0..taille]);
                match serde_json::from_str(&message) {
                    Ok(input) => {
                        add(allowed_pub.allowed_pub.clone(), input);
                    }
                    Err(e) => {
                        eprintln!("Erreur de sérialisation JSON: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Erreur de réception UDP: {}", e);
            }
        }
    }
}

fn add(allowed_pub: Vec<String>, input: structures::Input) {
    if allowed_pub.contains(&input.key_pub) {
        Command::new("sudo")
        .args(["wg", "set", "wg0", "peer", &input.key_pub, "allowed-ips", &input.allowed_ip])
        .output().unwrap();   
        println!("ok"); 
    } else {
        println!("pas ok");
    };
}