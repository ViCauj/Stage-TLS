use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm
};
use rsa::{pkcs1::DecodeRsaPublicKey, Pkcs1v15Encrypt, RsaPublicKey};


pub fn enc(data: Vec<u8>) -> Vec<u8> {
    let pub_key = RsaPublicKey::read_pkcs1_der_file("pub_key.bin").unwrap();
    let key = Aes256Gcm::generate_key(OsRng);
    let mut rng = rand::thread_rng();
    let enc_key = pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, key.as_slice()).unwrap();

    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message
    let ciphertext = cipher.encrypt(&nonce, data.as_slice()).unwrap();

    vec![enc_key, nonce.to_vec(), ciphertext].concat()
}