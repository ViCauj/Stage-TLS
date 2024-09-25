use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
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
pub struct OutputData {
    pub signature: String,
}

#[derive(Deserialize)]
pub struct CheckData {
    pub merged_json: MergedJson,
    pub output_json: OutputData,
}
