use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    // aead::{Aead, KeyInit},
    Aes256Gcm, Key
};
use std::{
    io::Read,
    fs::File,
};

// use crate::{
//     NONCE,KEY,TAG,CIPHERTXT,
//     decode, encode,
// };

pub fn enc(data: Vec<u8>) -> Vec<u8> {
    // let key = decode(KEY).unwrap();
    let mut key_file = File::open("dek.bin").unwrap();
    let mut key = Vec::new();
    key_file.read_to_end(&mut key).unwrap();
    let key: &Key<Aes256Gcm> = key.as_slice().into();

    let cipher = Aes256Gcm::new(&key);

    // let nonce = decode(NONCE).unwrap();
    // let nonce = Nonce::from_slice(nonce.as_slice());
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message
    
    let ciphertext = cipher.encrypt(&nonce, data.as_slice()).unwrap();

    // let res = vec![decode(CIPHERTXT).unwrap(), decode(TAG).unwrap()].concat();
    // eprintln!("{}", encode(&res));
    // assert!(res == ciphertext);
    
    vec![nonce.to_vec(), ciphertext].concat()
}