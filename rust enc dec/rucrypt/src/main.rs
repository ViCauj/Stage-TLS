use std::{
    io::{self, Read, Write, Result},
    env
};

mod aes_gcm;
mod chacha_poly;

fn main() -> Result<()> {
    // Pour savoir quel algo de chiffrement uriliser :
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("Manque un argument (\"aes\" ou \"cha\")");
    }
    let arg = &args[1];
    

    let mut data = Vec::new();
    io::stdin().read_to_end(&mut data)?;

    if arg == "aes" {
        let data_enc = aes_gcm::enc(data);
        io::stdout().write_all(&data_enc)?;
    } else if arg == "cha" {
        let data_enc = chacha_poly::enc(data);
        io::stdout().write_all(&data_enc)?;
    } else {
        eprintln!("Argument invalide (\"aes\" ou \"cha\")");
    }

    Ok(())
}