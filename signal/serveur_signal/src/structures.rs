use serde::{Serialize, Deserialize};

use crate::HashMap;

#[derive(Deserialize)]
pub struct Data2Send {
    pub sender_id: String,
    pub reciever_id: String,
    pub data: String,
}

#[derive(Deserialize)]
pub struct Reader {
    pub reader_id: String,
    pub sender_id: String,
    // info pour déchiffrer
}

#[derive(Serialize, Deserialize)]
pub struct KeysPub {
    pub id_key: String,
    pub pre_key_signed: String,
    pub signature: String,
    pub one_time_keys: HashMap<String, String>,
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
pub struct CheckSession {
    pub user1: UserID,
    pub user2: UserID
}

#[derive(Deserialize)]
pub struct UserWithKeys {
    pub id: String,
    pub keys: KeysPub
}

#[derive(Serialize, Deserialize)]
pub struct KeysPubOutput {
    pub id_key: String,
    pub pre_key_signed: String,
    pub signature: String,
    pub one_time_key: String
}

#[derive(Serialize, Deserialize)]
pub struct InitOutput {
    pub sender: User,
    pub receiver: UserID,
    pub id_key: String,
    pub temp_key: String,
    pub one_time_key_id: String,
    pub cipher: String,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub id_key: String,
    pub temp_key: String,
    pub one_time_key_id: String,
    pub cipher: String,
}