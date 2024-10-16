use x25519_dalek::{StaticSecret, PublicKey};

use crate::encode;

pub fn kpgen() -> (String, String) {
    let static_sec = StaticSecret::random_from_rng(&mut rand::rngs::OsRng);
    let pub_key = PublicKey::from(&static_sec);
    (encode(static_sec.to_bytes()), encode(pub_key.to_bytes()))
}

// pub fn kpgen() -> (String, String) {
//     let mut rng = rand::rngs::OsRng;
//     let signing_key: SigningKey = SigningKey::generate(&mut rng);
//     let verifying_key = VerifyingKey::from(&signing_key);
//     let pem_priv = signing_key.to_pkcs8_pem(pkcs8::LineEnding::LF).unwrap();
//     let pem_pub = verifying_key.to_public_key_pem(pkcs8::LineEnding::LF).unwrap();
//     (String::from(pem_priv.as_str()), pem_pub)
// }