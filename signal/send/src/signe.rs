use std::str::FromStr;

use ed25519_dalek::{VerifyingKey, Signature, Verifier};
use pkcs8::DecodePublicKey;

pub fn check(message: String, signature: String, pem_verifying_key: String) -> bool {
    let verifying_key = VerifyingKey::from_public_key_pem(&pem_verifying_key).unwrap();
    let signature = Signature::from_str(&signature).unwrap();
    verifying_key.verify(message.as_bytes(), &signature).is_ok()
}