use rsa::RsaPrivateKey;
use pkcs8::EncodePrivateKey;
use crate::{
    Write,
    io,
};

pub fn keygen() {
    let mut rng = rand::rngs::OsRng;
    let bits = 3072;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).unwrap();
    let pem = priv_key.to_pkcs8_pem(pkcs8::LineEnding::LF).unwrap();
    io::stdout().write_all((*pem).as_bytes()).unwrap();
}

// pub fn _keygen() {
//     let mut rng = rand::rngs::OsRng;
//     let bits = 3072;
//     let priv_key = RsaPrivateKey::new(&mut rng, bits).unwrap();
//     io::stdout().write_all(priv_key.to_pkcs1_der().unwrap().as_bytes()).unwrap();
// }