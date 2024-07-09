use primitive_types::{U256, U512};
use sha2::{Digest, Sha512};

const P: &str = "57896044618658097711785492504343953926634992332820282019728792003956564819949";
const D: &str = "37095705934669439343138083508754565189542113879843219016388785533085940283555";
const Q: &str = "7237005577332262213973186563042994240857116359379907606001950938285454250989"; // 2**252 + 27742317777372353535851937790883648493

fn _hash_mod_q(data: &[u8]) -> U256 {
    let q = U256::from_dec_str(Q).unwrap();

    let mut hasher = Sha512::new();
    hasher.update(data);
    let hash = hasher.finalize();
    let hash_512 = U512::from_big_endian(&hash); 
    let hash_mod = hash_512%U512::from(q);
    let hash_256 = U256::try_from(hash_mod).unwrap();
    hash_256
}

fn add(pt1: &[U256; 4],pt2: &[U256; 4]) -> [U256; 4]{
    // retourne pt1 + pt2
    // On utilise les coordonées homogènes étendues pour simplifier et accélérer

    let p = U256::from_dec_str(P).unwrap();
    let d = U256::from_dec_str(D).unwrap();

    let [x, y, z, t]= pt1;
    let [i, j, k, l] = pt2;

    let two = U256::from(2);
    let (_a, _b) = ((y-x)*(j-i)%p, (x+y)*(i+j)%p);
    let (_c, _d) = (two*t*l*d%p, two*z*k%p);
    let (_e, _f,_g,_h) = (_b-_a, _d-_c, _d+_c, _b+_a);

    let four = U256::from(4);
    [_e*_f/four, _g*_h/four, _f*_g/four, _e*_h/four] // entre 2

    // PB avec la doc : 

    // _e = 2(xj + yi)
    // _h = 2(xi + yj)
    // _f = 2(zl - dtk)
    // _g = 2(zl + dtk)

    // [_e*_f, _g*_h, _f*_g, _e*_h] // Ce qu'il y a dans la doc
    // [_e/_g, _h/_f, _f*_g/four, _e*_h/four] // Ce que j'ai calculé
}

// TODO : fn double {} <- Plus rapide

fn _mul(s: &mut U256, pt: &mut [U256; 4]) -> [U256; 4] {
    // retourne s*pt
    // On utilise les coordonées homogènes étendues pour simplifier et accélérer

    let (zero, one, two) = (U256::from(0), U256::from(1), U256::from(2));
    let mut e = [zero, one, one, zero]; // element neutre

    while *s > zero {
        if *s%two == one {
            e = add(&e, &pt);
        }
        *pt = add(pt, pt);
        *s = s.div_mod(two).0; // s//2
    }
    e
}

fn main() {
    let _d = U256::from_dec_str(D).unwrap();
    let _p = U256::from_dec_str(P).unwrap();
    let _q = U256::from_dec_str(Q).unwrap();

    // test :
    let (zero, one) = (U256::from(0), U256::from(1));
    let e = [zero, one, one, zero];
    let res = _mul(&mut U256::from(4),&mut e.clone());
    println!("{:?}", res)
}
