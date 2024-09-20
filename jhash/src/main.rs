use sha2::{Digest,Sha512};
use std::io::{self, Read, Write, Result};
use hex::encode;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct InputData {
    id: String,
    data: String,
    methode: String,
}

#[derive(Serialize)]
struct OutputData {
    id: String,
    hash: String,
    methode: String,
}

fn main() -> Result<()> {
    let mut data = String::new();
    io::stdin().read_to_string(&mut data)?;
    let parsed_data: InputData = serde_json::from_str(&data)?;

    let mut hasher = Sha512::new();
    hasher.update(parsed_data.data);

    let output = OutputData {
        id: parsed_data.id,
        hash: encode(hasher.finalize()),
        methode: parsed_data.methode,
    };
    io::stdout().write_all(serde_json::to_string(&output)?.as_bytes()).unwrap();

    Ok(())
}