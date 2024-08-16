use primitive_types::{U256, U512};

use crate::{
    arithm::{inv_mod, pow_mod}, 
    D, P, 
    ZERO, UN, DEUX, TROIS, QUATRE,
};

#[derive(Debug)]
pub enum ErrPerso {
    DecompressionImpossibleZeroNegatif,
    DecompressionImpossiblePasUneRacine,
}

pub fn comp(pt: [U512; 4]) -> [u8; 32] {
    let p = U512::from_dec_str(P).unwrap();
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
    if pt_comp[31] >= 128 { // On regarde si bit de poid fort vaut 1
        signe = UN;
        y[31] ^= 0b10000000;
    }
    let y = U512::from_little_endian(&y)%p;
    let x = recup_x(y, signe).unwrap();

    return [x, y, UN, x*y%p]
}

pub fn recup_x(y: U512, signe: U512) -> Result<U512, ErrPerso> { 
    let p = U512::from_dec_str(P).unwrap();
    let d = U512::from_dec_str(D).unwrap();
    
    let y2 = pow_mod(y, DEUX);
    let x2 = ((y2+p-UN)%p)*inv_mod(d*y2%p + UN)%p;

    if x2 == ZERO {
        if signe == UN {
            return Err(ErrPerso::DecompressionImpossibleZeroNegatif);
        } else {
            return Ok(ZERO);
        }
    }

    // let mut x = pow_mod(x2, (p+TROIS)*inv_mod(DEUX*QUATRE)%p);
    let mut x = pow_mod(x2, (p+TROIS)/(DEUX*QUATRE));

    if x*x%p != x2 {
        // x = x*pow_mod(DEUX, (p-UN)*inv_mod(QUATRE)%p)%p;
        x = x*pow_mod(DEUX, (p-UN)/QUATRE)%p;
        if x*x%p != x2 {
            return Err(ErrPerso::DecompressionImpossiblePasUneRacine);
        }
    }

    if x%DEUX != signe {
        x = p-x;
    }

    Ok(x)  
}