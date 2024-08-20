use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    // aead::{Aead, KeyInit},
    ChaCha20Poly1305, Key,
    // aead::Payload
};

use std::{
    io::Read,
    fs::File,
};

// use crate::{
//     NONCE, KEY, CIPHERTXT, TAG, AAD,
//     decode, encode,
// };

pub fn enc(data: Vec<u8>) -> Vec<u8> {
    // let key = decode(KEY).unwrap();
    // let key: &Key = key.as_slice().into();
    let mut key_file = File::open("dek.bin").unwrap();
    let mut key = Vec::new();
    key_file.read_to_end(&mut key).unwrap();
    let key: &Key = key.as_slice().into();

    let cipher = ChaCha20Poly1305::new(&key);

    // let nonce = decode(NONCE).unwrap();
    // let nonce = Nonce::from_slice(nonce.as_slice());
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);

    // let aad = decode(AAD).unwrap();
    // let aad = aad.as_slice();
    // let aad = AAD.as_bytes();

    // let ciphertext = cipher.encrypt(nonce, Payload {
    //     msg: data.as_slice(),
    //     aad: aad
    // }).unwrap();

    let ciphertext = cipher.encrypt(&nonce, data.as_slice()).unwrap();

    // let res = vec![decode(CIPHERTXT).unwrap(), decode(TAG).unwrap()].concat();
    // eprintln!("{}", encode(&ciphertext));
    // assert!(res == ciphertext);

    vec![nonce.to_vec(), ciphertext].concat()
}