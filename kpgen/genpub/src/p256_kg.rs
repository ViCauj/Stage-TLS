use p256::ecdsa::{SigningKey, VerifyingKey};
use crate::{
    Write,
    io,
};

pub fn keyexp(data: Vec<u8>) {
    let bytes: [u8; 32] = data.try_into().unwrap();
    let signing_key = SigningKey::from_bytes(&bytes.into()).unwrap();
    let verifying_key = VerifyingKey::from(&signing_key);
    io::stdout().write_all(verifying_key.to_encoded_point(true).as_bytes()).unwrap();
}