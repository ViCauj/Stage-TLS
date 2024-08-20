use std::{
    io::{self, Read, Write},
    env
};

mod aes_gcm;
mod chacha_poly;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("Manque un argument (\"aes\" ou \"cha\")");
    }
    let arg = &args[1];

    let mut data_enc = Vec::new();
    io::stdin().read_to_end(&mut data_enc).unwrap();
    
    if arg == "aes" {
        let data = aes_gcm::dec(data_enc);
        io::stdout().write_all(&data).unwrap();
    } else if arg == "cha" {
        let data = chacha_poly::dec(data_enc);
        io::stdout().write_all(&data).unwrap();
    } else {
        eprintln!("Argument invalide (\"aes\" ou \"cha\")");
    }
}