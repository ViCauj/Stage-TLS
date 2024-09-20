use sha2::{Digest,Sha512};
use std::io::{self, Read, Write, Result};
use hex::encode;
fn main() -> Result<()> {
    let mut data = Vec::new();
    io::stdin().read_to_end(&mut data)?;

    let mut hasher = Sha512::new();
    hasher.update(data);
    io::stdout().write_all((encode(hasher.finalize())).as_bytes()).unwrap();

    Ok(())
}