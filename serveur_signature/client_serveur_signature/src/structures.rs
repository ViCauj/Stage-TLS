use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct MergedJson {
    pub obj: Vec<HashedData>
}

#[derive(Deserialize, Serialize)]
pub struct HashedData {
    pub id: String,
    pub hash: String,
    pub methode: String,
}


#[derive(Deserialize, Serialize)]
pub struct Hash {
    pub hash: String,
}

#[derive(Deserialize, Serialize)]
pub struct Signature {
    pub signature: String,
}

#[derive(Deserialize)]
pub struct CheckData {
    pub merged_json: MergedJson,
    pub signature: Signature,
}

#[derive(Deserialize, Serialize)]
pub struct CheckDataHashed {
    pub hash: String,
    pub signature: String,
}
