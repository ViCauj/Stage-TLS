use x25519_dalek::StaticSecret;
use  xeddsa::{xed25519::PrivateKey, Sign};

use crate::{encode, decode};

pub fn signe(message: String, dh_sec: String) -> String {
    let dh_sec: [u8; 32] = decode(dh_sec).unwrap().try_into().unwrap();
    let private_key = PrivateKey::from(&StaticSecret::from(dh_sec));
    let signature: [u8; 64] = private_key.sign(&decode(message).unwrap(), &mut rand::rngs::OsRng);
    encode(signature)
}