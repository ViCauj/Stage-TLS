use std::{
    io::{self, Read, Write, Result},
    env
};
use structures::{CheckData, CheckDataHashed, Hash, MergedJson, Signature};
use sha2::{Digest, Sha512};
use reqwest::Client;
use hex::encode;

mod structures;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("Manque un argument (\"signe\" ou \"check\" attendu)");
    }

    let mut data = String::new();
    io::stdin().read_to_string(&mut data).unwrap();

    let client = Client::new();
    let url = "http://0.0.0.0:3000";

    match args[1].as_str() {
        "signe" => {
            let input: MergedJson = serde_json::from_str(&data).unwrap(); //permet de s'assurer qe le format est bon
            let mut hasher = Sha512::new();
            hasher.update(serde_json::to_string(&input).unwrap());
            let signature: Signature = client.post(format!("{}/sign", url)).json(&Hash{hash: encode(hasher.finalize())})
                .send().await.unwrap()
                .json().await.unwrap();

                io::stdout().write_all(serde_json::to_string(&signature).unwrap().as_bytes()).unwrap();
        },
        "check" => {
            let input: CheckData = serde_json::from_str(&data).unwrap();
            let mut hasher = Sha512::new();
            hasher.update(serde_json::to_string(&input.merged_json).unwrap());
            client.post(format!("{}/check", url)).json(&CheckDataHashed{hash: encode(hasher.finalize()), signature: input.signature.signature}).send().await.unwrap();
        },
        _ => {
            eprintln!("Argument non valide (\"signe\" ou \"check\" attendu)");
            return Ok(());
        }  
    };

    Ok(())
}
