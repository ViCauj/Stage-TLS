use x25519_dalek::{StaticSecret, PublicKey};
use sha2::Sha512;
use hkdf::Hkdf;

use crate::decode;

fn string_to_sec(key: String) -> StaticSecret {
    let priv_key: [u8; 32] = decode(key).unwrap().try_into().unwrap();
    StaticSecret::from(priv_key)
}

fn string_to_pub(key: String) -> PublicKey {
    let pub_key: [u8; 32] = decode(key).unwrap().try_into().unwrap();
    PublicKey::from(pub_key)
}

pub fn aad_gen(id_key_sender: String, id_key_receiver: String) -> Vec<u8> {
    let mut res = Vec::new();
    res.extend(decode(id_key_sender).unwrap());
    res.extend(decode(id_key_receiver).unwrap());

    res
}

pub fn dh(private: String, public: String) -> [u8; 32] {
    let private = string_to_sec(private);
    let public = string_to_pub(public);

    *private.diffie_hellman(&public).as_bytes()
}

// KDF du symemetric-key ratchet
pub fn kdf_ck(chain_key: [u8; 32]) -> ([u8; 32], [u8; 32]) {
    let hk = Hkdf::<Sha512>::new(None, &chain_key);

    let (mut new_chain_key , mut message_key) = ([0u8; 32], [0u8;32]);
    hk.expand(&[0x01], &mut new_chain_key).unwrap();
    hk.expand(&[0x02], &mut message_key).unwrap();

    (new_chain_key, message_key)
}

// KDF du diffie-hellman ratchet
pub fn kdf_first_chain_key(root_key: [u8;32], dh_out: [u8;32]) -> [u8; 32] {
    let hk = Hkdf::<Sha512>::new(Some(&root_key), &dh_out);
    
    let mut chain_key = [0u8; 32];
    hk.expand(&[0x02], &mut chain_key).unwrap();

    chain_key
}