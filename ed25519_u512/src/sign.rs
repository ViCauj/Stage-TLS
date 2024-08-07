use primitive_types::{U256, U512};
use sha2::{Digest,Sha512};

use crate::{
    Q,P,
    ZERO,UN,QUATRE,CINQ,
    compress::{comp, recup_x},
    point_op::mul,
    arithm::inv_mod
};

pub fn hash(data: &[u8]) -> U512 {
    let mut hasher = Sha512::new();
    hasher.update(data);
    let hash = hasher.finalize();
    let hash_512 = U512::from_big_endian(&hash); 
    hash_512
}

pub fn hash_mod_q(data: &[u8]) -> U512 {
    let q = U512::from_dec_str(Q).unwrap();

    let mut hasher = Sha512::new();
    hasher.update(data);
    let hash = hasher.finalize();
    let hash_512 = U512::from_big_endian(&hash); 
    let hash_mod = hash_512%U512::from(q);
    hash_mod
}

pub fn key_expand(priv_key: &[u8; 32]) -> (U512, [u8;32], [u8;32]) { // pas du tout sûr de cette fonction
    let p = U512::from_dec_str(P).unwrap();
    let y = QUATRE*inv_mod(CINQ);
    let x = recup_x(y, ZERO).unwrap();
    let g = [x, y, UN, x*y%p];

    let h = hash(priv_key);
    let mut bytes = [0u8; 64];
    h.to_little_endian(&mut bytes);

    bytes[0] &= 0b11111000; 
    bytes[31] &= 0b01111111; 
    bytes[31] |= 0b01000000;   

    let s = U512::from_little_endian(&bytes[0..32]);
    let pub_key = comp(mul(&mut s.clone(), &mut g.clone()));
    let prefix: &[u8; 32] = &bytes[32..64].try_into().unwrap();

    (s, pub_key, *prefix)
}

pub fn signe(priv_key: &[u8; 32], message: &[u8]) -> [u8; 64]{
    let p = U512::from_dec_str(P).unwrap();
    let q = U512::from_dec_str(Q).unwrap();
    let y = QUATRE*inv_mod(CINQ);
    let x = recup_x(y, ZERO).unwrap();
    let g = [x, y, UN, x*y%p];

    let (s, pub_key, prefix) = key_expand(priv_key);
    let r = hash_mod_q(&[&prefix, message].concat()[..]);
    let rs = comp(mul(&mut r.clone(), &mut g.clone()));
    let h = hash_mod_q(&[&rs, &pub_key, message].concat()[..]);
    let s: U256 = U256::try_from((r + h*s)%q).unwrap();
    let mut bytes = [0u8; 32];
    s.to_little_endian(&mut bytes);

    return [rs, bytes].concat().try_into().unwrap()
}