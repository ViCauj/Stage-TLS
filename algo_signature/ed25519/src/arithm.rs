use primitive_types::U512;

use crate::{
    P,
    ZERO,UN,DEUX
};

pub fn pow_mod(base: U512, exp: U512) -> U512 {
    // expo rapide modulaire

    let p = U512::from_dec_str(P).unwrap();
    let mut res = UN;
    let mut base = U512::from(base%p);
    let mut exponent = U512::from(exp);

    while exponent > ZERO{
        if (exponent & UN) != ZERO {
            res = (res*base)%p;
        }
        exponent = exponent >> 1;
        base = (base*base)%p;
    }

    res
}

pub fn inv_mod(x: U512) -> U512 {
    // x^-1 = x^(p-2) [p]
    let p = U512::from_dec_str(P).unwrap();
    return pow_mod(x, p-DEUX);
}