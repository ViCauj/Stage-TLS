use std::{
    io::{self, Read}, 
    path::Path,
    collections::HashMap,
};
use reqwest::Client;
use tokio::fs;

mod structures;
mod keygen;
mod signe;
mod hash;

use keygen::kpgen;

const NOMBRE_OTK: usize = 10;

#[tokio::main]
async fn main() -> Result<(), String> {
    let mut data = String::new();
    io::stdin().read_to_string(&mut data).unwrap();

    let user: structures::User = serde_json::from_str(&data).unwrap();

    let client = Client::new();
    let url = "http://0.0.0.0:3000";

    if client.post(format!("{}/check_user", url)).json(&user).send().await.unwrap().text().await.unwrap() != String::from("") {
        return Err("ID déjà utilisé".to_string()) 
    };

    let path = format!("{}", user.id);
    if !Path::new(&path).exists() {
        fs::create_dir(&path).await.unwrap();
        let keys_path = format!("{}/keys.json", &path);

        let id_keyp = kpgen();
        let kp_signed = kpgen();
        let one_time_keyps: Vec<(String, String)>= (0..NOMBRE_OTK).map(|_| kpgen()).collect();
        let (mut one_time_keys_priv, mut one_time_keys_pub) = (HashMap::new(), HashMap::new());
        for key_pair in one_time_keyps.iter() {
            let hash = hash::sha512(key_pair.1.clone());
            one_time_keys_priv.insert(hash.clone(), key_pair.0.clone());
            one_time_keys_pub.insert(hash.clone(), key_pair.1.clone());
        };

        let priv_keys = structures::KeysPriv {
            id_key: id_keyp.0.clone(),    
            signed_key: kp_signed.0,
            one_time_keys: one_time_keys_priv,
            root_keys: HashMap::new()
        };

        let pub_keys = structures::KeysPub {
            id_key: id_keyp.1,    
            signed_key: kp_signed.1.clone(),
            signature: signe::signe(kp_signed.1, id_keyp.0).to_string(), // on signe le pem
            one_time_keys: one_time_keys_pub,
        };

        let user_with_keys = structures::UserWithKeys {
            id: user.id,
            keys: pub_keys
        };
        client.post(format!("{}/register", url)).json(&user_with_keys).send().await.unwrap();
        fs::write(keys_path, serde_json::to_string(&priv_keys).unwrap()).await.unwrap();
    } else {
        return Err("Fichier config déjà présent".to_string()) 
    }

    Ok(())
}

// ATTENTION IL FAUT TOUT ZEROIZE APRES