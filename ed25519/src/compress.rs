use primitive_types::{U256, U512};
use sha2::{Digest,Sha512};

use crate::{
    arithm::{inv_mod, pow_mod, mult}, 
    D, P, Q, 
    ZERO, UN, DEUX, TROIS, QUATRE,
};

#[derive(Debug)]
pub enum ErrPerso {
    ImpossibleRecovery1,
    ImpossibleRecovery2,
}

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

pub fn comp(pt: [U256; 4]) -> [u8; 32] {
    let [x, y, z, _t]= pt;

    let z_inv = inv_mod(z);
    let x = mult(x,z_inv);
    let y = mult(y,z_inv); 

    let mut bytes = [0u8; 32];
    y.to_little_endian(&mut bytes);
    if x%DEUX == UN { // Pas sûr de ce résultat, j'ai l'impression qu'on a équivalence entre le signe et la parité
        bytes[31] ^= 0b10000000; // Je change le dernier bite (premier en big endian)
    }

    bytes
}

pub fn decomp(pt_comp: [u8; 32]) -> [U256; 4] {
    let mut signe = ZERO;
    let mut y = pt_comp;
    if pt_comp[31] == 0b10000000 {
        signe = UN;
        y[31] = 0b00000000;
    }
    let y = U256::from_little_endian(&y);
    let x = recup_x(y, signe).unwrap();

    return [x, y, UN, mult(x, y)]
}

pub fn recup_x(y: U256, signe: U256) -> Result<U256, ErrPerso> { // (1,0) marche pas
    let p = U256::from_dec_str(P).unwrap();
    let d = U256::from_dec_str(D).unwrap();
    
    let y2 = pow_mod(y, DEUX);
    let x2 = mult((y2+p-UN)%p, inv_mod(mult(d, y2) + UN));

    if x2 == ZERO {
        if signe == UN {
            return Err(ErrPerso::ImpossibleRecovery1);
        } else {
            return Ok(ZERO);
        }
    }

    let mut x = pow_mod(x2, (p+TROIS)/(DEUX*QUATRE));

    if mult(x,x) != x2 {
        x = mult(x, pow_mod(DEUX, (p-UN)/QUATRE));
        if mult(x,x) != x2 {
            return Err(ErrPerso::ImpossibleRecovery2);
        }
    }

    if x%DEUX != signe {
        x = p-x;
    }

    Ok(x)  
}