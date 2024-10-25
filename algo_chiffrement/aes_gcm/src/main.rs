use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key // Or `Aes128Gcm`
};

fn main() {
    // The encryption key can be generated randomly:
    let key = Aes256Gcm::generate_key(OsRng);

    // Transformed from a byte array:
    // let key: &[u8; 32] = &[42; 32];
    // let key: &Key<Aes256Gcm> = key.into();

    // Alternatively, the key can be transformed directly from a byte slice
    // (panicks on length mismatch):
    // let key = Key::<Aes256Gcm>::from_slice(&key);

    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message
    let ciphertext = cipher.encrypt(&nonce, b"plaintext message".as_ref()).unwrap();
    println!("{:?}", b"plaintext message".len()+16);
    println!("{:?}", ciphertext);
    let plaintext = cipher.decrypt(&nonce, ciphertext.as_ref()).unwrap();
    assert_eq!(&plaintext, b"plaintext message");
}
