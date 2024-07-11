use primitive_types::U256;

use crate::{
    P,D,Q, 
    ZERO,UN,DEUX,QUATRE,
    arithm::inv_mod,
};

pub fn _add(pt1: &[U256; 4],pt2: &[U256; 4]) -> [U256; 4]{
    // retourne pt1 + pt2
    // On utilise les coordonées homogènes étendues pour simplifier et accélérer
    let p = U256::from_dec_str(P).unwrap();
    let d = U256::from_dec_str(D).unwrap();

    let [x, y, z, t]= pt1;
    let [i, j, k, l] = pt2;

    let (_a, _b) = ((y-x)*(j-i)%p, (x+y)*(i+j)%p);
    let (_c, _d) = (DEUX*t*l*d%p, DEUX*z*k%p);
    let (_e, _f,_g,_h) = (_b-_a, _d-_c, _d+_c, _b+_a);

    [_e*_f/QUATRE, _g*_h/QUATRE, _f*_g/QUATRE, _e*_h/QUATRE] // entre 2

    // PB avec la doc : 

    // _e = 2(xj + yi)
    // _h = 2(xi + yj)
    // _f = 2(zl - dtk)
    // _g = 2(zl + dtk)

    // [_e*_f, _g*_h, _f*_g, _e*_h] // Ce qu'il y a dans la doc
    // [_e/_g, _h/_f, _f*_g/four, _e*_h/four] // Ce que j'ai calculé
}

// TODO : fn double {} <- Plus rapide

pub fn _mul(s: &mut U256, pt: &mut [U256; 4]) -> [U256; 4] {
    // retourne s x pt
    // On utilise les coordonées homogènes étendues pour simplifier et accélérer
    let mut e = [ZERO, UN, UN, ZERO]; // element neutre

    while *s > ZERO {
        if *s%DEUX == UN {
            e = _add(&e, &pt);
        }
        *pt = _add(pt, pt);
        *s = s.div_mod(DEUX).0; // s//2
    }
    e
}

pub fn _equ(pt1: [U256; 4], pt2: [U256; 4]) -> bool {
    // On regarde si les coordonées affines (!=coordonées homogènes étendues) x et y sont les mêmes sans faire de divisions
    let p = U256::from_dec_str(P).unwrap();

    let [x, y, z, _t]= pt1;
    let [i, j, k, _l] = pt2;

    if ((x*k - i*z)%p != ZERO) || ((y*k - j*z)%p != ZERO) {
        return false
    }
    true
}