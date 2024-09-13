use ed25519_dalek::{SigningKey, VerifyingKey};
use crate::{
    Write,
    io,
};

pub fn keyexp(data: Vec<u8>) {
    let signing_key = SigningKey::from_bytes(&data.try_into().unwrap());
    let verifying_key = VerifyingKey::from(&signing_key);
    io::stdout().write_all(verifying_key.as_bytes()).unwrap();
}