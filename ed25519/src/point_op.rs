use primitive_types::U256;

use crate::{
    P,D,Q, 
    ZERO,UN,DEUX,QUATRE,
    arithm::{inv_mod, mult},
};

pub fn add(pt1: &[U256; 4],pt2: &[U256; 4]) -> [U256; 4]{
    // retourne pt1 + pt2
    // On utilise les coordonées homogènes étendues pour simplifier et accélérer
    let p = U256::from_dec_str(P).unwrap();
    let d = U256::from_dec_str(D).unwrap();

    let [x, y, z, t]= pt1;
    let [i, j, k, l] = pt2;

    println!("{}\n{}\n{}", t, l, d);

    let (_a, _b) = (mult((y+p-x)%p,(j+p-i)%p), mult(x+y,i+j));
    let (_c, _d) = (mult(mult(DEUX,*t), mult(*l,d)), mult(DEUX, mult(*z,*k)));
    let (_e, _f,_g,_h) = ((_b+p-_a)%p, (_d+p-_c)%p, (_d+_c)%p, (_b+_a)%p);
    [mult(_e,_f)/QUATRE, mult(_g,_h)/QUATRE, mult(_f,_g)/QUATRE, mult(_e,_h)/QUATRE] // entre 2

    // PB avec la doc : 

    // _e = 2(xj + yi)
    // _h = 2(xi + yj)
    // _f = 2(zl - dtk)
    // _g = 2(zl + dtk)

    // [_e*_f, _g*_h, _f*_g, _e*_h] // Ce qu'il y a dans la doc
    // [_e/_g, _h/_f, _f*_g/four, _e*_h/four] // Ce que j'ai calculé
}

// TODO : fn double {} <- Plus rapide

pub fn mul(s: &mut U256, pt: &mut [U256; 4]) -> [U256; 4] {
    // retourne s x pt
    // On utilise les coordonées homogènes étendues pour simplifier et accélérer
    let mut e = [ZERO, UN, UN, ZERO]; // element neutre

    while *s > ZERO {
        if *s%DEUX == UN {
            e = add(&e, &pt);
        }
        *pt = add(pt, pt);
        *s = s.div_mod(DEUX).0; // s//2
    }
    e
}

pub fn equ(pt1: [U256; 4], pt2: [U256; 4]) -> bool {
    // On regarde si les coordonées affines (!=coordonées homogènes étendues) x et y sont les mêmes sans faire de divisions
    let p = U256::from_dec_str(P).unwrap();

    let [x, y, z, _t]= pt1;
    let [i, j, k, _l] = pt2;

    if ((x*k - i*z)%p != ZERO) || ((y*k - j*z)%p != ZERO) {
        return false
    }
    true
}