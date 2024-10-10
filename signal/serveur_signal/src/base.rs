use std::path::Path;
use serde_json::json;
use tokio::{
    fs,
    io::{AsyncReadExt, AsyncWriteExt}
};
use crate::{
    structures::{CheckSession, Data2Send, InitOutput, KeysPub, KeysPubOutput, Message, Reader, User, UserID, UserWithKeys}, Json
};

pub async fn send(Json(payload): Json<Data2Send>) -> Result<(), String> {
    let receiver_path = format!("user/{}", payload.reciever_id);

    if !Path::new(&receiver_path).exists() {
        return Err("Destinataire inconnu".to_string())
    }
    let file_path = format!("{}/received_data.json", receiver_path);

    let mut data = if Path::new(&file_path).exists() {
        let content = fs::read_to_string(&file_path).await.unwrap_or_else(|_| "{}".to_string());
        serde_json::from_str::<serde_json::Value>(&content).unwrap_or_else(|_| json!({}))
    } else {
        json!({})
    };

    if !data.get(&payload.sender_id).is_some() {
        data[&payload.sender_id] = json!([]);
    }
    if let Some(sender_entry) = data[&payload.sender_id].as_array_mut() {
        sender_entry.push(json!(payload.data));
    }

    let mut file = fs::File::create(&file_path).await.unwrap();
    let serialized_data = serde_json::to_string_pretty(&data).unwrap();
    file.write_all(serialized_data.as_bytes()).await.unwrap();

    Ok(())
}

pub async fn read(Json(payload): Json<Reader>) -> Result<(), String> {
    let reader_path = format!("user/{}", payload.reader_id);

    if !Path::new(&reader_path).exists() {
        return Err("ID inconnu".to_string())
    }
    let file_path = format!("user/{}/received_data.json", payload.reader_id);
    
    if Path::new(&file_path).exists() {
        let content = fs::read_to_string(file_path).await.unwrap();
        let data: serde_json::Value = serde_json::from_str(&content).unwrap();
        let messages = data.get(&payload.sender_id).unwrap().as_array().unwrap();
    
        for message in messages.iter() {
            eprintln!("{}", message.as_str().unwrap());
        }
    }
    
    Ok(())
}

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

pub async fn check_session(Json(payload): Json<CheckSession>) -> Result<(), String> { // Changer
    let (path1, path2) = (format!("user/{}", payload.user1.id), format!("user/{}", payload.user2.id));
    if !Path::new(&path1).exists() || !Path::new(&path2).exists(){
        return Err("User non enregistré".to_string())
    };

    if Path::new(&format!("{}/{}", path1, payload.user2.id)).exists() && Path::new(&format!("{}/{}", path2, payload.user1.id)).exists() {
        return Ok(())
    } else if Path::new(&format!("{}/{}", path1, payload.user2.id)).exists() {
        fs::remove_dir_all(format!("{}/{}", path1, payload.user2.id)).await.unwrap();
    } else if Path::new(&format!("{}/{}", path2, payload.user1.id)).exists() {
        fs::remove_dir_all(format!("{}/{}", path2, payload.user1.id)).await.unwrap();
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
        pre_key_signed: keys.pre_key_signed,
        signature: keys.signature,
        one_time_key: one_time_key     
    })
}

pub async fn premier_message(Json(payload): Json<InitOutput>) {
    let path_sender = format!("user/{}/{}", payload.sender.id, payload.receiver.id);
    let path_receiver = format!("user/{}/{}", payload.receiver.id, payload.sender.id);
    // normalement le check session s'assure que la création des dir en dessous est ok
    fs::create_dir(&path_sender).await.unwrap();
    fs::create_dir(&path_receiver).await.unwrap();

    let message = Message {
        id_key: payload.id_key,
        temp_key: payload.temp_key,
        one_time_key_id: payload.one_time_key_id,
        cipher: payload.cipher,
    };
    fs::write(&format!("{}/send.json", path_sender), serde_json::to_string(&message).unwrap()).await.unwrap(); // je ne sais pas si c'est nécessaire
    fs::write(&format!("{}/receive.json", path_receiver), serde_json::to_string(&message).unwrap()).await.unwrap();
    // changer le format de send.json et receive.json, il faut que ça puisse être une liste de message (là il ne peut y avoir qu'un seul message, le dernier)
}