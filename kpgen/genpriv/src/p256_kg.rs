use p256::ecdsa::SigningKey;
use crate::{
    Write,
    io,
};

pub fn keygen() {
    let mut rng = rand::rngs::OsRng;
    let signing_key = SigningKey::random(&mut rng);
    io::stdout().write_all(&signing_key.to_bytes()).unwrap();
}