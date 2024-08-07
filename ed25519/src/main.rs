use primitive_types::U256;

mod arithm;
mod compress;
mod point_op;

use compress::{comp, decomp, recup_x};
use arithm::{inv_mod, mult};
use point_op::mul;

const P: &str = "57896044618658097711785492504343953926634992332820282019728792003956564819949";
const D: &str = "37095705934669439343138083508754565189542113879843219016388785533085940283555";
const Q: &str = "7237005577332262213973186563042994240857116359379907606001950938285454250989"; // 2**252 + 27742317777372353535851937790883648493

const ZERO: U256 = U256([0, 0, 0, 0]);
const UN: U256 = U256([1, 0, 0, 0]);
const DEUX: U256 = U256([2, 0, 0, 0]);
const TROIS: U256 = U256([3, 0, 0, 0]);
const QUATRE: U256 = U256([4, 0, 0, 0]);
const CINQ: U256 = U256([5, 0, 0, 0]);

fn main() {    
    let p = U256::from_dec_str(P).unwrap();

    let y = QUATRE*inv_mod(CINQ);
    let x = recup_x(y, ZERO).unwrap();
    let b = [x, y, UN, mult(x, y)];

    let mut pt = mul(&mut U256::from(2), &mut b.clone());
    pt[1] = mult(pt[1], inv_mod(pt[2]));
    pt[2] = UN;
    
    let pt_compress = comp(pt);
    let pt_decompress = decomp(pt_compress);

    println!("{:?}", pt[0]);
    println!("{:?}\n", pt_decompress[0]);
    println!("{:?}", pt[1]);
    println!("{:?}\n", pt_decompress[1]);
    println!("{:?}", mult(pt[3], inv_mod(pt[2])));
    println!("{:?}\n", pt[3]);
    assert!(b == pt_decompress);
}
