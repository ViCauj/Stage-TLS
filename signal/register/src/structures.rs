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
    pub root_keys: HashMap<String, IndexMap<String, String>>, // (k = id_autre_personne, v = (k = "ma_clef_pub|sa_clef_pub", v = root_key))
    pub send_key: HashMap<String, String>, // (k = id_autre_personne, v = current_send_chain_ke)
    pub receive_key_chain: HashMap<String, IndexMap<String, String>>, // (k = id_autre_personne, v = (k = "ma_clef_pub|sa_clef_pub", v = receive_key))
}

#[derive(Serialize, Deserialize)]
pub struct KeysPub {
    pub id_key: String,
    pub signed_key: String,
    pub signature: String,
    pub one_time_keys: HashMap<String, String>,
}
