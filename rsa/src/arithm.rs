use crate::{
    ZERO,UN,DEUX,
    U1024,
};

pub fn pow_mod(base: U1024, exp: U1024, p: U1024) -> U1024 {
    // expo rapide modulaire
    let mut res = UN;
    let mut base = U1024::from(base%p);
    let mut exponent = U1024::from(exp);

    while exponent > ZERO{
        if (exponent & UN) != ZERO {
            res = (res*base)%p;
        }
        exponent = exponent >> 1;
        base = (base*base)%p;
    }

    res
}

pub fn inv_mod(x: U1024, p: U1024) -> U1024 {
    // x^-1 = x^(p-2) [p]
    return pow_mod(x, p-DEUX, p);
}