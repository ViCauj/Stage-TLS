use std::{
    io::{self, Read, Write, Result},
    env
};

mod rsa_kg;
mod ed25519_kg;
mod p256_kg;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("Manque un argument (\"rsa\", \"ed25519\" ou \"p256\" attendu)");
    }

    let mut data = Vec::new();
    io::stdin().read_to_end(&mut data)?;

    match args[1].as_str() {
        "rsa" => rsa_kg::keyexp(data),
        "ed25519" => ed25519_kg::keyexp(data),
        "p256" => p256_kg::keyexp(data),
        _ => {
            eprintln!("Argument non valide (\"rsa\", \"ed25519\" ou \"p256\" attendu)");
            return Ok(());
        }  
    };

    Ok(())
}