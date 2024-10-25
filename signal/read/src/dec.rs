use aes_gcm::{
    aead::{AeadMutInPlace, KeyInit}, 
    Aes256Gcm,
    Nonce
};
use hex::decode;
use hex::encode;

pub fn aes_gcm(ciphertext: String, aad: Vec<u8>, key: [u8; 32]) -> String {
    let mut cipher = Aes256Gcm::new(&key.into());
    let mut ciphertext = decode(ciphertext).unwrap();
    let nonce: Vec<u8> = ciphertext.drain(..12).collect();
    let nonce = Nonce::from_slice(&nonce);
    cipher.decrypt_in_place(&nonce, &aad, &mut ciphertext).unwrap();

    String::from_utf8(ciphertext).unwrap()
}