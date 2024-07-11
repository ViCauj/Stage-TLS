use primitive_types::{U256, U512};
use sha2::{Digest, Sha512};
// use std::fmt;

use crate::{
    arithm::{inv_mod, pow_mod}, 
    D, P, Q, 
    ZERO, UN, DEUX, QUATRE,
};

#[derive(Debug)]
pub enum err {
    ImpossibleRecovery1,
    ImpossibleRecovery2,
}

// impl fmt::Display for err {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             err::ImpossibleRecovery => write!(f, "impossible de récupérer x"),
//         }
//     }
// }

pub fn hash_mod_q(data: &[u8]) -> U256 {
    let q = U256::from_dec_str(Q).unwrap();

    let mut hasher = Sha512::new();
    hasher.update(data);
    let hash = hasher.finalize();
    let hash_512 = U512::from_big_endian(&hash); 
    let hash_mod = hash_512%U512::from(q);
    let hash_256 = U256::try_from(hash_mod).unwrap();
    hash_256
}

pub fn enc(pt: [U256; 4]) -> [u8; 32] {
    let p = U256::from_dec_str(P).unwrap();

    let [x, y, z, _t]= pt;

    let z_inv = inv_mod(z);
    let x = x*z_inv%p;
    let y = y*z_inv%p; 

    let mut bytes = [0u8; 32];
    y.to_little_endian(&mut bytes);
    if x%DEUX == UN { // Pas sûr de ce résultat, j'ai l'impression qu'on a équivalence entre le signe et la parité
        bytes[31] ^= 0b10000000; // Je change le dernier bite (premier en big endian)
    }

    bytes
}

pub fn recup_x(y: U256, signe: U256) -> Result<U256, err> { // remettre signe: u8 quand finit de débugger : pb avec pow_mod
    let p = U256::from_dec_str(P).unwrap();
    let d = U256::from_dec_str(D).unwrap();
    
    let y2 = pow_mod(y, DEUX);
    let x2 = U256::try_from(U512::from(y2-UN)*U512::from(inv_mod(d*y2))%p).unwrap(); // pas sur d'avoir le droit de faire un modulo ici mais c'est pratique

    println!("{}", y2);
    if x2 == ZERO {
        if signe == UN {
            return Err(err::ImpossibleRecovery1);
        } else {
            return Ok(ZERO);
        }
    }

    let mut x = pow_mod(x2, d*y2 + UN);

    if (pow_mod(x, DEUX) - x2)%p != ZERO {
        x = x*pow_mod(DEUX, (p-UN)/QUATRE);

        if (pow_mod(x, DEUX) - x2)%p != ZERO {
            return Err(err::ImpossibleRecovery2);
        }
    }

    if x%DEUX != U256::from(signe) {
        x = p-x;
    }

    Ok(x)  
}