use sha2::{Digest,Sha512};
use hex::encode;

pub fn sha512(data: String) -> String {
    let mut hasher = Sha512::new();
    hasher.update(data);
    encode(hasher.finalize())
}