use sha2::{Digest,Sha512};


use crate::{
    arithm::{inv_mod, pow_mod},
    E,P,Q,
    U1024,
};

pub fn keygen() -> ((U1024, U1024), (U1024, U1024)){
    let e = U1024::from_str_radix(E, 16).unwrap();
    let p = U1024::from_str_radix(P, 16).unwrap();
    let q = U1024::from_str_radix(Q, 16).unwrap();

    let n = p*q;
    let phi_n = (p-1)*(q-1);
    let d = inv_mod(e, phi_n);

    ((n, e), (n, d)) // (pub_key, priv_key)
}

pub fn hash(data: &[u8]) -> U1024 {
    let mut hasher = Sha512::new();
    hasher.update(data);
    let hash = hasher.finalize();
    let hash_512 = U1024::from_big_endian(&hash); 
    hash_512
}

pub fn sign(priv_key: (U1024, U1024), message: &[u8]) -> U1024 {
    // attention à choisir p,q tq n plus grand que le hash
    let h = hash(message);

    pow_mod(h, priv_key.1, priv_key.0)
}

pub fn check(pub_key: (U1024, U1024), message: &[u8], signature: U1024) -> bool {
    let h = hash(message);

    signature == pow_mod(h, pub_key.1, pub_key.0)
}