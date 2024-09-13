use ed25519_dalek::SigningKey;
use crate::{
    Write,
    io,
};

pub fn keygen() {
    let mut rng = rand::rngs::OsRng;
    let signing_key: SigningKey = SigningKey::generate(&mut rng);
    io::stdout().write_all(signing_key.as_bytes()).unwrap();
}