use std::{
    io::{self, Read, Write, Result},
    env
};
use zeroize::Zeroize;

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
        let data_enc = [enc_key, nonce, ciphertext, last_block, tag].concat();
        io::stdout().write_all(&data_enc)?;
    } else {
        eprintln!("Manque un argument (\"aes\", \"aes_stream\" ou \"cha\")");
    }

    Ok(())
}
