use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AllowedPub {
    pub allowed_pub: Vec<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Input {
    pub allowed_ip: String,
    pub key_pub: String
}