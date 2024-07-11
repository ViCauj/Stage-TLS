use primitive_types::{U256, U512};

use crate::P;

pub fn pow_mod(base: U256, exp: U256) -> U256 {
    // expo rapide modulaire
    // j'aimerai bien ne pas avoir à passer en U512...

    let p = U256::from_dec_str(P).unwrap();
    let mut res = U512::from(1);
    let mut base = U512::from(base%p);
    let mut exponent = U512::from(exp);

    while exponent > U512::from(0){
        if (exponent & U512::from(1)) != U512::from(0) {
            res = (res*base)%p;
        }
        exponent = exponent >> 1;
        base = (base*base)%p;
    }

    U256::try_from(res).unwrap()
}

pub fn inv_mod(x: U256) -> U256 {
    // x^-1 = x^(p-2) [p]
    let p = U256::from_dec_str(P).unwrap();
    return pow_mod(x, p-2);
}