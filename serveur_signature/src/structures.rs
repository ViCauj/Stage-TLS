use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct HashedData {
    pub id: String,
    pub hash: String,
    pub methode: String,
}

#[derive(Serialize)]
pub struct OutputData {
    pub signature: String,
    pub verif_key: String,
}

#[derive(Deserialize)]
pub struct CheckData {
    pub hash: String,
    pub signature: String,
    pub verif_key: String,
}
