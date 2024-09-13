use rsa::{
    pkcs1::{DecodeRsaPrivateKey, EncodeRsaPublicKey}, 
    RsaPrivateKey, RsaPublicKey
};
use crate::{
    Write,
    io,
};

pub fn keyexp(data: Vec<u8>) {
    let priv_key = RsaPrivateKey::from_pkcs1_der(&data).unwrap();
    let pub_key = RsaPublicKey::from(&priv_key);
    io::stdout().write_all(pub_key.to_pkcs1_der().unwrap().as_bytes()).unwrap();
}