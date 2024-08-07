#![allow(dead_code)]

use primitive_types::U512;

mod arithm;
mod compress;
mod point_op;

use compress::{comp, decomp, recup_x};
use arithm::inv_mod;
use point_op::mul;

const P: &str = "57896044618658097711785492504343953926634992332820282019728792003956564819949";
const D: &str = "37095705934669439343138083508754565189542113879843219016388785533085940283555";
const Q: &str = "7237005577332262213973186563042994240857116359379907606001950938285454250989"; // 2**252 + 27742317777372353535851937790883648493

const ZERO: U512 = U512([0, 0, 0, 0, 0, 0, 0, 0]);
const UN: U512 = U512([1, 0, 0, 0, 0, 0, 0, 0]);
const DEUX: U512 = U512([2, 0, 0, 0, 0, 0, 0, 0]);
const TROIS: U512 = U512([3, 0, 0, 0, 0, 0, 0, 0]);
const QUATRE: U512 = U512([4, 0, 0, 0, 0, 0, 0, 0]);
const CINQ: U512 = U512([5, 0, 0, 0, 0, 0, 0, 0]);

fn main() {    
    let p = U512::from_dec_str(P).unwrap();
    
    let y = QUATRE*inv_mod(CINQ);
    let x = recup_x(y, ZERO).unwrap();
    let b = [x, y, UN, x*y%p];

    let mut pt = mul(&mut U512::from(2), &mut b.clone());
    pt[1] = pt[1]*inv_mod(pt[2])%p;
    pt[2] = UN;

    // let pt = b;

    println!("pt :");
    println!("{:?}\n", pt);
    
    let pt_compress = comp(pt);
    println!("pt comp :");
    for i in pt_compress.iter() {
        print!("{:02x}", i);
    }
    println!("\n");
    let pt_decompress = decomp(pt_compress);

    println!("X :");
    println!("{:?}", pt[0]);
    println!("{:?}\n", pt_decompress[0]);

    println!("{:?}\n", (p-pt_decompress[0])%p);

    println!("Y :");
    println!("{:?}", pt[1]);
    println!("{:?}\n", pt_decompress[1]);
    println!("T :");
    println!("{:?}", pt[3]*inv_mod(pt[2])%p);
    println!("{:?}\n", pt[3]);
    assert!(pt == pt_decompress);
}


// TODO : il faudrait modifier le trait Mul de U512 dans la crate primitive_type pour avoir une multiplication modulaire i.e a*b%p pour simplifier la lecture
//        On pourrait faire pareil avec la division qui utiliserai Mul et inv_mod au lieux de faire une division classique

// Pour vérifier add et/ou compress marche : https://asecuritysite.com/encryption/ed