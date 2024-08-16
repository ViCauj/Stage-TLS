#![allow(dead_code)]

use primitive_types::U512;
use hex::{decode, encode};

mod arithm;
mod compress;
mod point_op;
mod sign;

use sign::{check, signe};

const P: &str = "57896044618658097711785492504343953926634992332820282019728792003956564819949";
const D: &str = "37095705934669439343138083508754565189542113879843219016388785533085940283555";
const Q: &str = "7237005577332262213973186563042994240857116359379907606001950938285454250989"; // 2**252 + 27742317777372353535851937790883648493

const ZERO: U512 = U512([0, 0, 0, 0, 0, 0, 0, 0]);
const UN: U512 = U512([1, 0, 0, 0, 0, 0, 0, 0]);
const DEUX: U512 = U512([2, 0, 0, 0, 0, 0, 0, 0]);
const TROIS: U512 = U512([3, 0, 0, 0, 0, 0, 0, 0]);
const QUATRE: U512 = U512([4, 0, 0, 0, 0, 0, 0, 0]);
const CINQ: U512 = U512([5, 0, 0, 0, 0, 0, 0, 0]);

const KEY: &str = "833fe62409237b9d62ec77587520911e9a759cec1d19755b7da901b96dca3d42";
const PUBKEY: &str = "ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf";
const MSG: &str = "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f";
const SIGN: &str = "dc2a4459e7369633a52b1bf277839a00201009a3efbf3ecb69bea2186c26b58909351fc9ac90b3ecfdfbc7c66431e0303dca179c138ac17ad9bef1177331a704";

fn main() {   
    let (pub_key, signature) = signe(&(decode(KEY).unwrap()).try_into().unwrap(), &(decode(MSG).unwrap()));

    assert!(encode(pub_key) == PUBKEY);
    assert!(encode(signature) == SIGN);
    assert!(check(pub_key, &(decode(MSG).unwrap()), signature));
}


// TODO : il faudrait modifier le trait Mul de U512 dans la crate primitive_type pour avoir une multiplication modulaire i.e a*b%p pour simplifier la lecture
//        On pourrait faire pareil avec la division qui utiliserai Mul et inv_mod au lieux de faire une division classique

// Pour vérifier add et/ou compress marche : https://asecuritysite.com/encryption/ed
//                                           https://asecuritysite.com/nacl/nacl07?a=5