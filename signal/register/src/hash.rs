use sha2::{Digest,Sha512};
use crate::encode;

pub fn sha512(data: String) -> String {
    let mut hasher = Sha512::new();
    hasher.update(data);
    encode(hasher.finalize())
}