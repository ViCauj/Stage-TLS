use x25519_dalek::{StaticSecret, PublicKey};
use sha2::Sha512;
use hkdf::Hkdf;

use crate::{encode, decode};

pub fn kpgen() -> (String, String) {
    let static_sec = StaticSecret::random_from_rng(&mut rand::rngs::OsRng);
    let pub_key = PublicKey::from(&static_sec);
    (encode(static_sec.to_bytes()), encode(pub_key.to_bytes()))
}

fn string_to_sec(key: String) -> StaticSecret {
    let priv_key: [u8; 32] = decode(key).unwrap().try_into().unwrap();
    StaticSecret::from(priv_key)
}

fn string_to_pub(key: String) -> PublicKey {
    let pub_key: [u8; 32] = decode(key).unwrap().try_into().unwrap();
    PublicKey::from(pub_key)
}

pub fn priv_to_pub(priv_key: String) -> String {
    encode(PublicKey::from(&string_to_sec(priv_key)).to_bytes())
}

fn kfd(dh_conc: Vec<u8>) -> [u8; 32] {
    let mut input = vec![255;32];
    input.extend(dh_conc);
    let salt = [0u8;64];   // doit faire la taille de l'output du hash i.e 64 bytes (512 bits)
    let info = [];  // pas d'info pour le moment

    let hk = Hkdf::<Sha512>::new(Some(&salt), &input);
    let mut output = [0u8; 32];    // impose la taille de la clef dérivée 
    hk.expand(&info, &mut output).unwrap();

    output
}

pub fn skgen(id_key_sender: String, ephemeral_key: String, id_key_receiver: String, signed_key: String, one_time_key: String) -> [u8; 32] {
    let sender_keys = (string_to_sec(id_key_sender), string_to_sec(ephemeral_key));
    let receiver_keys = (string_to_pub(id_key_receiver), string_to_pub(signed_key));

    let mut dh = Vec::new();
    dh.extend(sender_keys.0.diffie_hellman(&receiver_keys.1).as_bytes());
    dh.extend(sender_keys.1.diffie_hellman(&receiver_keys.0).as_bytes());
    dh.extend(sender_keys.1.diffie_hellman(&receiver_keys.1).as_bytes());

    if !one_time_key.is_empty() {
        dh.extend(sender_keys.1.diffie_hellman(&string_to_pub(one_time_key)).as_bytes());
    }

    kfd(dh)
}

pub fn skrecup(id_key_old_sender: String, ephemeral_key: String, id_key_old_receiver: String, signed_key: String, one_time_key: String) -> [u8; 32] {
    let old_sender_keys = (string_to_pub(id_key_old_sender), string_to_pub(ephemeral_key));
    let old_receiver_keys = (string_to_sec(id_key_old_receiver), string_to_sec(signed_key));

    let mut dh = Vec::new();
    dh.extend(old_receiver_keys.1.diffie_hellman(&old_sender_keys.0).as_bytes());
    dh.extend(old_receiver_keys.0.diffie_hellman(&old_sender_keys.1).as_bytes());
    dh.extend(old_receiver_keys.1.diffie_hellman(&old_sender_keys.1).as_bytes());

    if !one_time_key.is_empty() {
        dh.extend(string_to_sec(one_time_key).diffie_hellman(&old_sender_keys.1).as_bytes());
    }

    kfd(dh)
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
pub fn kdf_rk(root_key: [u8;32], dh_out: [u8;32]) -> ([u8; 32], [u8; 32]) {
    let hk = Hkdf::<Sha512>::new(Some(&root_key), &dh_out);
    
    let (mut new_root_key, mut chain_key) = ([0u8; 32], [0u8; 32]);
    // je devrais rajouter de l'info
    hk.expand(&[0x01], &mut new_root_key).unwrap();
    hk.expand(&[0x02], &mut chain_key).unwrap();

    (new_root_key, chain_key)
}