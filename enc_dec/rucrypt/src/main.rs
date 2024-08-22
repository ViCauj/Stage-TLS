use std::{
    io::{self, Read, Write, Result},
    env
};

mod aes_gcm;
mod aes_gcm_stream;
mod chacha_poly;

fn main() -> Result<()> {
    // Pour savoir quel algo de chiffrement uriliser :
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("Manque un argument (\"aes\", \"aes_stream\" ou \"cha\")");
    }
    let arg = &args[1];
    


    if arg == "aes" {
        let mut data = Vec::new();
        io::stdin().read_to_end(&mut data)?;
        let data_enc = aes_gcm::enc(data);
        io::stdout().write_all(&data_enc)?;
    } else if arg == "cha" {
        let mut data = Vec::new();
        io::stdin().read_to_end(&mut data)?;
        let data_enc = chacha_poly::enc(data);
        io::stdout().write_all(&data_enc)?;
    } else if arg == "aes_stream" {
        // let (enc_key, nonce, mut cipher) = aes_gcm_stream::enc_init();
        // let mut data = Vec::new();
        // io::stdin().read_to_end(&mut data)?;
        // let data_enc = cipher.update(&data);
        // let (last_block, tag) = cipher.finalize();
        // eprintln!("{}", hex::encode([data_enc.clone(), last_block.clone(), tag.clone()].concat()));
        // let data_enc = [enc_key, nonce, data_enc, last_block, tag].concat();
        // io::stdout().write_all(&data_enc)?;

        let (enc_key, nonce, mut cipher) = aes_gcm_stream::enc_init();
        let mut data = [0; 1024];
        let mut ciphertext = vec![];
        loop {
            let bytes_read = io::stdin().read(&mut data)?;
            if bytes_read == 0 {
                break;
            }
            ciphertext.extend_from_slice(&cipher.update(&data[..bytes_read]))
        }
        let (last_block, tag) = cipher.finalize();
        eprintln!("{}", hex::encode([ciphertext.clone(), last_block.clone(), tag.clone()].concat()));
        let data_enc = [enc_key, nonce, ciphertext, last_block, tag].concat();
        io::stdout().write_all(&data_enc)?;
    } else {
        eprintln!("Manque un argument (\"aes\", \"aes_stream\" ou \"cha\")");
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