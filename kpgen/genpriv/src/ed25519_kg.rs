use ed25519_dalek::SigningKey;
use pkcs8::EncodePrivateKey;
use crate::{
    Write,
    io,
};

pub fn keygen() {
    let mut rng = rand::rngs::OsRng;
    let signing_key: SigningKey = SigningKey::generate(&mut rng);
    let pem = signing_key.to_pkcs8_pem(pkcs8::LineEnding::LF).unwrap();
    io::stdout().write_all((*pem).as_bytes()).unwrap();
}