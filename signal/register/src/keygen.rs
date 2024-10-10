use ed25519_dalek::{SigningKey, VerifyingKey};
use pkcs8::{
    EncodePrivateKey, EncodePublicKey
};

pub fn kpgen() -> (String, String) {
    let mut rng = rand::rngs::OsRng;
    let signing_key: SigningKey = SigningKey::generate(&mut rng);
    let verifying_key = VerifyingKey::from(&signing_key);
    let pem_priv = signing_key.to_pkcs8_pem(pkcs8::LineEnding::LF).unwrap();
    let pem_pub = verifying_key.to_public_key_pem(pkcs8::LineEnding::LF).unwrap();
    (String::from(pem_priv.as_str()), pem_pub)
}