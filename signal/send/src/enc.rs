use aes_gcm::{
    aead::{AeadCore, AeadMutInPlace, KeyInit, OsRng}, 
    Aes256Gcm
};

pub fn aesgcm(data: String, aad: Vec<u8>, key: [u8; 32]) -> Vec<u8> {
    let mut cipher = Aes256Gcm::new(&key.into());
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let mut ciphertext = data.into_bytes();
    cipher.encrypt_in_place(&nonce, &aad, &mut ciphertext).unwrap();

    eprintln!("{}", nonce.to_vec().len());
    vec![nonce.to_vec(), ciphertext].concat()
} 