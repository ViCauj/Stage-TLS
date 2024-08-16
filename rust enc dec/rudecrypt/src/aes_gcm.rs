use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Key, Nonce
};

use std::{
    io::Read,
    fs::File,
};

pub fn dec(data_enc: Vec<u8>) -> Vec<u8> {
    // On récupère la clef
    let mut key_file = File::open("dek.bin").unwrap();
    let mut key = Vec::new();
    key_file.read_to_end(&mut key).unwrap();
    let key: &Key<Aes256Gcm> = key.as_slice().into();

    let nonce = Nonce::from_slice(&data_enc[0..12]);
    let ciphertext = &data_enc[12..];

    let cipher = Aes256Gcm::new(&key);
    let plaintext = cipher.decrypt(&nonce, ciphertext.as_ref()).unwrap();
    
    plaintext
}