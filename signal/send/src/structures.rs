use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct InputData {
    pub sender: User,
    pub receiver: UserID,
    pub data: String,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: String,
}

#[derive(Serialize, Deserialize)] // pour le moment identique à user mais peut être que plus tard il y aura plus dans user, userID permet d'identifier n'importe qui pas d'utiliser son compte
pub struct UserID {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct KeysPub {
    pub id_key: String,
    pub signed_key: String,
    pub signature: String,
    pub one_time_keys: HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
pub struct KeysPriv {
    pub id_key: String,
    pub signed_key: String,
    pub one_time_keys: HashMap<String, String>,
    pub root_keys: HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
pub struct KeysPubOutput {
    pub id_key: String,
    pub signed_key: String,
    pub signature: String,
    pub one_time_key: String
}

#[derive(Serialize, Deserialize)]
pub struct InitOutput {
    pub session: Session,
    pub id_key: String,
    pub temp_key: String,
    pub signed_key_id: String,
    pub one_time_key_id: String,
    pub cipher: String,
}

#[derive(Serialize, Deserialize)]
pub struct Session {
    pub sender: UserID,
    pub receiver: UserID
}

#[derive(Serialize, Deserialize)]
pub struct SessionData {
    pub receiver_pub_key: String
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub id_key: String,
    pub temp_key: String,
    pub one_time_key_id: String,
    pub cipher: String,
}