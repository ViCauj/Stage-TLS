use p256::ecdsa::SigningKey;
use pkcs8::EncodePrivateKey;
use crate::{
    Write,
    io,
};

pub fn keygen() {
    let mut rng = rand::rngs::OsRng;
    let signing_key= SigningKey::random(&mut rng);
    let pem = signing_key.to_pkcs8_pem(pkcs8::LineEnding::LF).unwrap();
    io::stdout().write_all((*pem).as_bytes()).unwrap();
}

pub fn _keygen() {
    let mut rng = rand::rngs::OsRng;
    let signing_key = SigningKey::random(&mut rng);
    io::stdout().write_all(&signing_key.to_bytes()).unwrap();
}