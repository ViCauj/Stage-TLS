use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use indexmap::IndexMap;


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
    pub signed_keys: IndexMap<String, String>,
    pub one_time_keys: HashMap<String, String>,
    pub root_keys: HashMap<String, (String, String)>,
    pub chain_keys: HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
pub struct KeysPub {
    pub id_key: String,
    pub signed_key: String,
    pub signature: String,
    pub one_time_keys: HashMap<String, String>,
}
