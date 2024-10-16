use xeddsa::{
    xeddsa::Verify,
    xed25519::PublicKey as XEdDSAPubKey,
};
use x25519_dalek::PublicKey;

use crate::decode;

pub fn check(message: String, signature: String, dh_pub: String) -> bool {
    let dh_pub: [u8; 32] = decode(dh_pub).unwrap().try_into().unwrap();
    let pub_key = XEdDSAPubKey::from(&PublicKey::from(dh_pub));

    let signature: &[u8; 64] = &decode(signature).unwrap().try_into().unwrap();
    let message: &[u8] = &decode(message).unwrap();
    pub_key.verify(message, signature).is_ok()
}