use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Key, Nonce
};
use rsa::{pkcs1::DecodeRsaPrivateKey, RsaPrivateKey, Pkcs1v15Encrypt};
// use crate::Zeroize;

pub fn dec(data_enc: Vec<u8>) -> Vec<u8> {
    let priv_key = RsaPrivateKey::read_pkcs1_der_file("priv_key.bin").unwrap();
    let key = priv_key.decrypt(Pkcs1v15Encrypt, &data_enc[..256]).unwrap();
    let key: &Key = key.as_slice().into();
    let nonce = Nonce::from_slice(&data_enc[256..268]);
    let ciphertext = &data_enc[268..];

    let cipher = ChaCha20Poly1305::new(&key);
    let plaintext = cipher.decrypt(&nonce, ciphertext.as_ref()).unwrap();

    // key.zeroize();

    plaintext
}