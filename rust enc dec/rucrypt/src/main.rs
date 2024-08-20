use std::{
    io::{self, Read, Write, Result},
    env
    //fs::File,
};

mod aes_gcm;
mod chacha_poly;

// const KEY: &str = "808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9f";
// const NONCE: &str = "070000004041424344454647";
// const MSG: &str = "1";
// const AAD: &str= "123456789abcdef0";
// const CIPHERTXT: &str = "ae";
// const TAG: &str = "3ed2f824f901a8994052f852127c196a";

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("Manque un argument (\"aes\" ou \"cha\")");
    }
    let arg = &args[1];

    // // Si je veux créer la clef
    // // let mut key_file = File::create("dek.bin")?;
    // // key_file.write_all(&[0u8; 32])?;
    // // Si je veux la récupérer
    
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