use aes_gcm_stream::Aes256GcmStreamEncryptor;
use aes_gcm::{
    aead::{AeadCore, KeyInit, OsRng},
    Aes256Gcm
};
use rsa::{pkcs1::DecodeRsaPublicKey, Pkcs1v15Encrypt, RsaPublicKey};

pub fn enc_init() -> (Vec<u8>, Vec<u8>, Aes256GcmStreamEncryptor) {
    let pub_key = RsaPublicKey::read_pkcs1_der_file("pub_key.bin").unwrap();
    let key = Aes256Gcm::generate_key(OsRng);
    let mut rng = rand::thread_rng();
    let enc_key = pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, key.as_slice()).unwrap();
    
    
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message
    let cipher = Aes256GcmStreamEncryptor::new(key.try_into().unwrap(), &nonce);

    (enc_key, nonce.as_slice().try_into().unwrap(), cipher)
}