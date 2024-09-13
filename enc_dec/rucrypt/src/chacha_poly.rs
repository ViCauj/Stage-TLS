use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305
};
use rsa::{pkcs1::DecodeRsaPublicKey, Pkcs1v15Encrypt, RsaPublicKey};
use crate::Zeroize;

pub fn enc(data: Vec<u8>) -> Vec<u8> {
    let pub_key = RsaPublicKey::read_pkcs1_der_file("pub_key.bin").unwrap();
    let mut key = ChaCha20Poly1305::generate_key(&mut OsRng);
    let mut rng = rand::thread_rng();
    let enc_key = pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, key.as_slice()).unwrap();
    key.zeroize();

    let cipher = ChaCha20Poly1305::new(&key);
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
    let ciphertext = cipher.encrypt(&nonce, data.as_slice()).unwrap();

    vec![enc_key, nonce.to_vec(), ciphertext].concat()
}

// Pour rajouter des aad :

// use chacha20poly1305::aead::Payload;

// let ciphertext = cipher.encrypt(nonce, Payload {
//     msg: data.as_slice(),
//     aad: aad
// }).unwrap();