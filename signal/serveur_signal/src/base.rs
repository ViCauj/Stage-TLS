use std::path::Path;
use tokio::{
    fs,
    io::AsyncReadExt
};
use indexmap::IndexMap;
use crate::{
    structures::{self, DataToSend, InitOutput, KeysPub, KeysPubOutput, Messages, PremierMessage, Session, User, UserID, UserWithKeys}, Json
};

pub async fn register(Json(payload): Json<UserWithKeys>) -> Result<(), String> {
    let path = format!("user/{}", payload.id);
    fs::create_dir(&path).await.unwrap();

    let keys_path = format!("{}/keys.json", &path);
    fs::write(keys_path, serde_json::to_string(&payload.keys).unwrap()).await.unwrap();

    Ok(())
}

pub async fn check_user(Json(payload): Json<User>) -> Result<(), String> {
    let path = format!("user/{}", payload.id);
    if Path::new(&path).exists() {
        return Err("ID déjà utilisé".to_string())
    };
    
    Ok(())
}

pub async fn suppr_user(Json(payload): Json<User>) -> Result<(), String> {
    let path = format!("user/{}", payload.id);
    if Path::new(&path).exists() {
        fs::remove_dir_all(path).await.unwrap();
    };

    Ok(())
}

pub async fn check_session(Json(payload): Json<Session>) -> Result<(), String> {
    let (path1, path2) = (format!("user/{}", payload.sender.id), format!("user/{}", payload.receiver.id));
    if !Path::new(&path1).exists() || !Path::new(&path2).exists(){
        return Err("User non enregistré".to_string())
    };

    if Path::new(&format!("{}/{}", path1, payload.receiver.id)).exists() && Path::new(&format!("{}/{}", path2, payload.sender.id)).exists() {
        return Ok(())
    } else if Path::new(&format!("{}/{}", path1, payload.receiver.id)).exists() {
        fs::remove_dir_all(format!("{}/{}", path1, payload.receiver.id)).await.unwrap();
    } else if Path::new(&format!("{}/{}", path2, payload.sender.id)).exists() {
        fs::remove_dir_all(format!("{}/{}", path2, payload.sender.id)).await.unwrap();
    } // ces 2 else if sont au cas ou un utilisateur est supprimé après qu'une session soit init et qu'il recré un compte
    return Err("Session non initialisé".to_string())
}

pub async fn init_session(Json(payload): Json<UserID>) -> Json<KeysPubOutput> {  
    let path = format!("user/{}/keys.json", payload.id);

    let mut file = fs::File::open(&path).await.unwrap();
    let mut keys = String::new();
    file.read_to_string(&mut keys).await.unwrap();

    let mut keys: KeysPub = serde_yaml::from_str(&keys).unwrap();
    let one_time_key = if keys.one_time_keys.is_empty() {
        String::from("")
    } else {
        let key = keys.one_time_keys.keys().next().unwrap().clone();
        keys.one_time_keys.remove(&key).unwrap()
    };

    fs::write(&path, serde_json::to_string(&keys).unwrap()).await.unwrap();

    Json(KeysPubOutput{
        id_key: keys.id_key,
        signed_key: keys.signed_key,
        signature: keys.signature,
        one_time_key: one_time_key     
    })
}

pub async fn get_keys(Json(payload): Json<UserID>) -> Json<KeysPub> {
    let path = format!("user/{}/keys.json", payload.id);

    let mut file = fs::File::open(&path).await.unwrap();
    let mut keys = String::new();
    file.read_to_string(&mut keys).await.unwrap();

    let keys: KeysPub = serde_yaml::from_str(&keys).unwrap();
    Json(keys)
}

pub async fn premier_message(Json(payload): Json<InitOutput>) {
    let path_sender = format!("user/{}/{}", payload.session.sender.id, payload.session.receiver.id);
    let path_receiver = format!("user/{}/{}", payload.session.receiver.id, payload.session.sender.id);
    // normalement le check session s'assure que la création des dir en dessous est ok
    fs::create_dir(&path_sender).await.unwrap();
    fs::create_dir(&path_receiver).await.unwrap();

    let mut messages = Messages {
        premier_message: PremierMessage {
            id_sender: payload.session.sender.id,
            id_key: payload.id_key,
            temp_key: payload.temp_key,
            one_time_key_id: payload.one_time_key_id,
            signed_keys: payload.signed_keys,
            cipher: payload.cipher
        },
        messages_recus: IndexMap::new(),
        // messages_envoye: IndexMap::new()
    };

    fs::write(&format!("{}/messages.json", path_sender), serde_json::to_string(&messages).unwrap()).await.unwrap();

    messages.messages_recus.insert(structures::serial_tuple(messages.premier_message.signed_keys.clone()), vec![messages.premier_message.cipher.clone()]);
    fs::write(&format!("{}/messages.json", path_receiver), serde_json::to_string(&messages).unwrap()).await.unwrap();
}

pub async fn fetch_messages_sender(Json(payload): Json<Session>) -> Json<Messages> {
    let path = format!("user/{}/{}/messages.json", payload.sender.id, payload.receiver.id);
    let file_content = fs::read_to_string(&path).await.unwrap();
    let messages: Messages = serde_json::from_str(&file_content).unwrap();

    Json(messages)
}

pub async fn fetch_messages_receiver(Json(payload): Json<Session>) -> Json<Messages> {
    let path = format!("user/{}/{}/messages.json", payload.receiver.id, payload.sender.id);
    let file_content = fs::read_to_string(&path).await.unwrap();
    let messages: Messages = serde_json::from_str(&file_content).unwrap();

    Json(messages)
}

pub async fn send(Json(payload): Json<DataToSend>) {
    // let path_sender = format!("user/{}/{}/messages.json", payload.session.sender.id, payload.session.receiver.id);
    let path_receiver = format!("user/{}/{}/messages.json", payload.session.receiver.id, payload.session.sender.id);
    // let sender_file = fs::read_to_string(&path_sender).await.unwrap();
    let receiver_file = fs::read_to_string(&path_receiver).await.unwrap();

    // let mut messages_sender: Messages = serde_json::from_str(&sender_file).unwrap();
    let mut messages_receiver: Messages = serde_json::from_str(&receiver_file).unwrap();
    match messages_receiver.messages_recus.get_mut(&structures::serial_tuple(payload.message.signed_keys.clone())) {
        Some(x) => x.push(payload.message.cipher.clone()),
        None => {
            messages_receiver.messages_recus.insert(structures::serial_tuple(payload.message.signed_keys.clone()), vec![payload.message.cipher.clone()]);
        }
    };

    fs::write(&path_receiver, serde_json::to_string(&messages_receiver).unwrap()).await.unwrap();
}