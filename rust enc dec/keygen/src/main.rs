use rsa::{
    pkcs1::{EncodeRsaPrivateKey, EncodeRsaPublicKey}, 
    RsaPrivateKey, RsaPublicKey
};

fn main() {
    let mut rng = rand::thread_rng();
    let bits = 2048;

    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);

    priv_key.write_pkcs1_der_file("priv_key.bin").unwrap();
    pub_key.write_pkcs1_der_file("pub_key.bin").unwrap();
}
