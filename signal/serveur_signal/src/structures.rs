use serde::{Serialize, Deserialize};
use indexmap::IndexMap;


use crate::HashMap;

#[derive(Deserialize)]
pub struct Data2Send {
    pub sender_id: String,
    pub reciever_id: String,
    pub data: String,
}

#[derive(Serialize, Deserialize)]
pub struct KeysPub {
    pub id_key: String,
    pub signed_key: String,
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
pub struct Session {
    pub sender: UserID,
    pub receiver: UserID
}

#[derive(Deserialize)]
pub struct UserWithKeys {
    pub id: String,
    pub keys: KeysPub
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

#[derive(Serialize, Deserialize, Debug)]
pub struct MessagesRecus {
    pub premier_message: PremierMessage,
    pub messages: IndexMap<String, Vec<Message>> 
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PremierMessage {
    pub id_key: String,
    pub temp_key: String,
    pub one_time_key_id: String,
    pub signed_key_id: String,
    pub cipher: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub id_key: String,
    pub sender_signed_key: String,
    pub cipher: String,
}