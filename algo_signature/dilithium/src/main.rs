use pqc_dilithium::*;
use hex::encode;

fn main() {
    let keys = Keypair::generate();
    let msg = "Hello".as_bytes();
    let sig = keys.sign(&msg);
    eprintln!("{}", encode(sig));
    let sig_verify = verify(&sig, &msg, &keys.public);
    assert!(sig_verify.is_ok());
}
