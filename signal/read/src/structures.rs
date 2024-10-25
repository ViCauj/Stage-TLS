use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use indexmap::IndexMap;

pub fn deserial_tuple(a: String) -> (String, String) {
    let res: Vec<&str> = a.split("|").collect();
    (res[0].to_string(), res[1].to_string())
}   

#[derive(Deserialize)]
pub struct InputData {
    pub sender: User,
    pub receiver: UserID,
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

#[derive(Debug)]
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
    pub signed_keys: (String, String), // (receiver, sender),
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
    pub signed_keys: (String, String), // (receiver, sender)
    pub cipher: String,
}

#[derive(Serialize, Deserialize)]
pub struct Session {
    pub sender: UserID,
    pub receiver: UserID
}

#[derive(Serialize, Deserialize)]
pub struct DataToSend {
    pub message: Message,
    pub session: Session
}
