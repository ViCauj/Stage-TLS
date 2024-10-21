use serde::{Serialize, Deserialize};
use indexmap::IndexMap;
use crate::HashMap;

pub fn serial_tuple(tuple: (String, String)) -> String{
    format!("{}|{}", tuple.0, tuple.1)
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
    pub signed_keys: (String, String),
    pub one_time_key_id: String,
    pub cipher: String,
}

#[derive(Serialize, Deserialize)]
pub struct Messages {
    pub premier_message: PremierMessage,
    pub messages_recus: IndexMap<String, Vec<String>>,
    // pub messages_envoye: IndexMap<String, Vec<Message>>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct PremierMessage {
    pub id_sender: String,
    pub id_key: String,
    pub temp_key: String,
    pub one_time_key_id: String,
    pub signed_keys: (String, String), // (receiver, sender)
    pub cipher: String,
}
#[derive(Serialize, Deserialize)]
pub struct Message {
    pub signed_keys: (String, String),
    pub cipher: String,
}

#[derive(Serialize, Deserialize)]
pub struct DataToSend {
    pub message: Message,
    pub session: Session
}
