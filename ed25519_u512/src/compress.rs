use primitive_types::{U256, U512};
use sha2::{Digest,Sha512};

use crate::{
    arithm::{inv_mod, pow_mod}, 
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

pub fn comp(pt: [U512; 4]) -> [u8; 32] {
    let p = U256::from_dec_str(P).unwrap();
    let [x, y, z, _t]= pt;

    let z_inv = inv_mod(z);
    let x = x*z_inv%p;
    let y = y*z_inv%p; 
    let y = U256::try_from(y).unwrap();

    let mut bytes = [0u8; 32];
    y.to_little_endian(&mut bytes);
    if x%DEUX == UN { // Pas sûr de ce résultat, j'ai l'impression qu'on a équivalence entre le signe et la parité
        bytes[31] ^= 0b10000000; // Je change le dernier bite (premier en big endian)
    }

    bytes
}

pub fn decomp(pt_comp: [u8; 32]) -> [U512; 4] {
    let p = U256::from_dec_str(P).unwrap();

    let mut signe = ZERO;
    let mut y = pt_comp;
    if pt_comp[31] == 1 {
        signe = UN;
        y[31] = 0;
    }
    let y = U512::from_little_endian(&y);
    let x = recup_x(y, signe).unwrap();

    return [x, y, UN, x*y%p]
}

pub fn recup_x(y: U512, signe: U512) -> Result<U512, ErrPerso> { 
    // besoin d'imposer Z = 1 pour avoir unicité du point dans le format de coordonnées étendues : changer add en mettant Z = 1 et Y = Y/Z ?
    // (1,0,1,0) marche pas
    // marche que pour (0,1,1,0) et le pt générateur "b", j'ai essayé sur x*b mais pour x dans {2, ...,12} ne marche jamais.
    // pb : j'ai l'impression que x2 ne fournit pas le bon res car l'algo de Tonelli-Shanks à l'air de marcher (je récupère bien une racine carré qui match x2 quand c'est possible)

    let p = U512::from_dec_str(P).unwrap();
    let d = U512::from_dec_str(D).unwrap();
    
    let y2 = pow_mod(y, DEUX);
    let x2 = ((y2+p-UN)%p)*inv_mod(d*y2%p + UN)%p;

    if x2 == ZERO {
        if signe == UN {
            return Err(ErrPerso::ImpossibleRecovery1);
        } else {
            return Ok(ZERO);
        }
    }

    let mut x = pow_mod(x2, (p+TROIS)*inv_mod(DEUX*QUATRE)%p);

    println!("{}\n", x);
    println!("{}", x*x%p);
    println!("{}", x2);

    if x*x%p != x2 {
        x = x*pow_mod(DEUX, (p-UN)*inv_mod(QUATRE)%p)%p;
        if x*x%p != x2 {
            return Err(ErrPerso::ImpossibleRecovery2);
        }
    }

    if x%DEUX != signe {
        x = p-x;
    }

    println!();
    Ok(x)  
}