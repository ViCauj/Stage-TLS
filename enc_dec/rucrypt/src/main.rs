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

// chiffrement au cours de l'eau : 
    // let mut buffer = [0; 1024];
    
    // // Boucle de lecture/écriture
    // loop {
    //     // Lire un morceau de données depuis stdin
    //     let bytes_read = io::stdin().read(&mut buffer)?;
    //     if bytes_read == 0 {
    //         break; // Fin du flux
    //     }

    //     // Chiffrer les données lues
    //     let encrypted_data = aes_gcm::enc(&buffer[..bytes_read]);

    //     // Écrire les données chiffrées sur stdout
    //     io::stdout().write_all(&encrypted_data)?;
    // }

    // // Assurez-vous que tout est écrit dans stdout
    // io::stdout().flush()?;