use rsa::{RsaPrivateKey, RsaPublicKey};
use pkcs8::{EncodePublicKey, DecodePrivateKey};
use crate::{
    Write,
    io,
};

pub fn keyexp(data: Vec<u8>) {
    let priv_key = RsaPrivateKey::from_pkcs8_pem(&String::from_utf8(data).unwrap()).unwrap();
    let pub_key = RsaPublicKey::from(&priv_key);
    let pem = pub_key.to_public_key_pem(pkcs8::LineEnding::LF).unwrap();
    io::stdout().write_all((*pem).as_bytes()).unwrap();
}