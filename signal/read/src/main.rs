use std::io::{self, Read};
use reqwest::Client;
use structures::{Messages, Session, UserID, KeysPub};
use hex::{encode, decode};
use tokio::fs;

mod structures;
mod keygen;
mod dec;

#[tokio::main]
async fn main() -> Result<(), String> {
    let mut data = String::new();
    io::stdin().read_to_string(&mut data).unwrap();

    let input: structures::InputData = serde_json::from_str(&data).unwrap();

    let client = Client::new();
    let url = "http://0.0.0.0:3000";

    let input_session = Session {
        sender: UserID {id: input.sender.id.clone()},   // lui
        receiver: UserID {id: input.receiver.id.clone()},   // moi
    };
    let retour_check = client.post(format!("{}/check_session", url)).json(&input_session)
        .send().await.unwrap()
        .text().await.unwrap();

    if retour_check.clone() == String::from("User non enregistré") {
        return Err("User non enregistré".to_string()) // rajouter un moyen de savoir qui n'est pas enregistré
    } else if retour_check.clone() == String::from("Session non initialisé") {
        return Err("Session non initialisé".to_string())
    } else {
        let messages: Messages = client.post(format!("{}/fetch_messages_receiver", url)).json(&input_session)
            .send().await.unwrap()
            .json().await.unwrap();

        let path_receiver_keys = format!("{}/keys.json", input.receiver.id);
        let file_content = fs::read_to_string(&path_receiver_keys).await.unwrap();
        let receiver_keys: structures::KeysPriv = serde_json::from_str(&file_content).unwrap();

        let sender_keys: KeysPub = client.post(format!("{}/get_keys", url)).json(&UserID {id: input.sender.id.clone()})
            .send().await.unwrap()
            .json().await.unwrap();

        for (keys, chain) in messages.messages_recus {
            // let root_key = receiver_keys.root_keys.get(&input_session.sender.id).unwrap().get(&keys).unwrap();
            // let (my_key, his_key) = structures::deserial_tuple(keys);
            // let dh_out = keygen::dh(receiver_keys.signed_keys.get(&my_key).unwrap().clone(), his_key.clone());
            // let _chain_key = keygen::kdf_first_chain_key(decode(root_key).unwrap().try_into().unwrap(), dh_out);
            // eprintln!("{}\n{}\n", root_key, encode(_chain_key));
            let receive_chain = decode(receiver_keys.send_key.get(&input_session.sender.id).unwrap()).unwrap().try_into().unwrap();
            for message in chain {
                let (_chain_key, message_key) = keygen::kdf_ck(receive_chain);
                eprintln!("{}", dec::aes_gcm(message, keygen::aad_gen(sender_keys.id_key.clone(), keygen::priv_to_pub(receiver_keys.id_key.clone())), message_key));
            }
        };
    };

    Ok(())
}