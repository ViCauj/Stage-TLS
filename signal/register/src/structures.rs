use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserWithKeys {
    pub id: String,
    pub keys: KeysPub
}

#[derive(Serialize, Deserialize)]
pub struct KeysPriv {
    pub id_key: String,
    pub pre_key_signed: String,
    pub one_time_keys: HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
pub struct KeysPub {
    pub id_key: String,
    pub pre_key_signed: String,
    pub signature: String,
    pub one_time_keys: HashMap<String, String>,
}
