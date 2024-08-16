use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, 
};

use std::{
    io::Read,
    fs::File,
};

pub fn enc(data: Vec<u8>) -> Vec<u8> {
    // On récupère la clef
    let mut key_file = File::open("dek.bin").unwrap();
    let mut key = Vec::new();
    key_file.read_to_end(&mut key).unwrap();
    let key: &Key<Aes256Gcm> = key.as_slice().into();

    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message
    let ciphertext = cipher.encrypt(&nonce, data.as_slice()).unwrap();

    vec![nonce.to_vec(), ciphertext].concat()
}