use aes_gcm_stream::Aes256GcmStreamDecryptor;
use rsa::{pkcs1::DecodeRsaPrivateKey, RsaPrivateKey, Pkcs1v15Encrypt};

pub fn dec(data_enc: Vec<u8>) -> Vec<u8> {
    let priv_key = RsaPrivateKey::read_pkcs1_der_file("priv_key.bin").unwrap();
    let key = priv_key.decrypt(Pkcs1v15Encrypt, &data_enc[..256]).unwrap();
    let nonce = &data_enc[256..268];
    let ciphertext = &data_enc[268..];

    let mut cipher = Aes256GcmStreamDecryptor::new(key.try_into().unwrap(), &nonce);
    let plaintext = [cipher.update(&ciphertext), cipher.finalize().unwrap()].concat();
    
    plaintext
}