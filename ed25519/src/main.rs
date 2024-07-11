use primitive_types::U256;

mod arithm;
mod compress;

use compress::recup_x;

const P: &str = "57896044618658097711785492504343953926634992332820282019728792003956564819949";
const D: &str = "37095705934669439343138083508754565189542113879843219016388785533085940283555";
const Q: &str = "7237005577332262213973186563042994240857116359379907606001950938285454250989"; // 2**252 + 27742317777372353535851937790883648493

const ZERO: U256 = U256([0, 0, 0, 0]);
const UN: U256 = U256([1, 0, 0, 0]);
const DEUX: U256 = U256([2, 0, 0, 0]);
const QUATRE: U256 = U256([5, 0, 0, 0]);

fn main() {
    let p = U256::from_dec_str(P).unwrap();
    
    let x = U256::from(257);
    let y = p-UN;
    let _pt = [x, y, UN, ZERO];
    println!("x = {}", x);
    print!("x_recup = {}", recup_x(y, x%DEUX).unwrap())
}
