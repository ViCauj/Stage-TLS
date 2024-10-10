use ed25519_dalek::{SigningKey, Signature, Signer};
use pkcs8::DecodePrivateKey;

pub fn signe(message: String, pem_signing_key: String) -> Signature {
    let signing_key = SigningKey::from_pkcs8_pem(&pem_signing_key).unwrap();    
    let signature: Signature = signing_key.sign(message.as_bytes());
    
    signature
}