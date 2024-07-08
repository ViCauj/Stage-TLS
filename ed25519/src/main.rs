use primitive_types::{U256, U512};
use sha2::{Digest, Sha512};

const P: &str = "57896044618658097711785492504343953926634992332820282019728792003956564819949";
const D: &str = "37095705934669439343138083508754565189542113879843219016388785533085940283555";
const Q: &str = "7237005577332262213973186563042994240857116359379907606001950938285454250989"; // 2**252 + 27742317777372353535851937790883648493

fn hash_mod_q(data: &[u8], q: U256){
    let mut hasher = Sha512::new();
    hasher.update(data);
    let hash = hasher.finalize();
    let hash_512 = U512::from_big_endian(&hash); // peut être en big endian???
    let hash_256: U512 = hash_512.div_mod(U512::from(q)).1;

    for i in 0..64 {
        print!("{} ", hash_256.byte(i));
    }

    // écrire un truc qui converti en U256, ie retire les 32 derniers bytes du hash modlué (dans tt les cas ils valent 0
    // On peut vérifier si c'est big ou little endian jsp si es 0 doivent être au début ou a la fin, regarder
}
fn main() {
    let _p = U256::from_dec_str(P).unwrap();
    let _d = U256::from_dec_str(D).unwrap();
    let q = U256::from_dec_str(Q).unwrap();

    hash_mod_q(b"data", q);
}
