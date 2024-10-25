mod sign;

use sign::{signe_rsa, check_rsa};

fn main() {
    let message: &[u8] = b"This is a test of the tsunami alert system.";
    let (verif_key, signature) = signe_rsa(message);
    check_rsa(message, verif_key, signature);
}
