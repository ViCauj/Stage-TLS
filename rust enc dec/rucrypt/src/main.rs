use std::{
    io::{self, Read, Write, Result},
    env
    //fs::File,
};

mod aes_gcm;
mod chacha_poly;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("Manque un argument (\"aes\" ou \"cha\")");
    }
    let arg = &args[1];

    // Si je veux créer la clef
    // let mut key_file = File::create("dek.bin")?;
    // key_file.write_all(&[0u8; 32])?;
    // Si je veux la récupérer
    
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


// todo : 
//  -utiliser la crate clap pour gérer les options comme :
//      -"cat clair.data | rucrypt -aes des > enc.data" pour chiffrer avec aes gcm
//      -"cat clair.data | rucrypt -cha des > enc.data" pour chiffrer avec chachapoly
//      ...