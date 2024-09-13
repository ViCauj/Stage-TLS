use std::{
    io::{self, Write, Result},
    env
};

mod rsa_kg;
mod ed25519_kg;
mod p256_kg;

fn main() -> Result<()> {
    // Pour savoir quel algo de chiffrement uriliser :
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("Manque un argument (\"rsa\", \"ed25519\" ou \"p256\" attendu)");
    }

    match args[1].as_str() {
        "rsa" => rsa_kg::keygen(),
        "ed25519" => ed25519_kg::keygen(),
        "p256" => p256_kg::keygen(),
        _ => {
            eprintln!("Argument non valide (\"rsa\", \"ed25519\" ou \"p256\" attendu)");
            return Ok(());            
        }  
    };

    Ok(())
}