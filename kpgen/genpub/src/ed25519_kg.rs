use ed25519_dalek::{SigningKey, VerifyingKey};
use pkcs8::{EncodePublicKey, DecodePrivateKey};
use crate::{
    Write,
    io,
};

pub fn keyexp(data: Vec<u8>) {
    let signing_key = SigningKey::from_pkcs8_pem(&String::from_utf8(data).unwrap()).unwrap();
    let verifying_key = VerifyingKey::from(&signing_key);
    let pem = verifying_key.to_public_key_pem(pkcs8::LineEnding::LF).unwrap();
    io::stdout().write_all((*pem).as_bytes()).unwrap();
}