use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone)]
pub struct Config {
    pub hsm: bool,
    // pub key_id: u8,
    pub addr: String,
    pub key_file: String, 
}

#[derive(Deserialize, Serialize)]
pub struct OutputData {
    pub signature: String,
}

#[derive(Deserialize, Serialize)]
pub struct Hash {
    pub hash: String,
}

#[derive(Deserialize, Serialize)]
pub struct CheckDataHashed {
    pub hash: String,
    pub signature: String,
}
