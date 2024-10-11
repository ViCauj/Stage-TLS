use std::io::{self, Read};
use reqwest::Client;
use structures::{InitOutput, Message, Session, SessionData, UserID};
use tokio::fs;
use hex::encode;

mod structures;
mod signe;
mod keygen;
mod enc;
mod hash;

#[tokio::main]
async fn main() -> Result<(), String> {
    let mut data = String::new();
    io::stdin().read_to_string(&mut data).unwrap();

    let input: structures::InputData = serde_json::from_str(&data).unwrap();

    let client = Client::new();
    let url = "http://0.0.0.0:3000";

    let input_session = Session {
        sender: UserID {id: input.sender.id.clone()},
        receiver: UserID {id: input.receiver.id.clone()},
    };
    let retour_check = client.post(format!("{}/check_session", url)).json(&input_session)
        .send().await.unwrap()
        .text().await.unwrap();

    if retour_check.clone() == String::from("User non enregistré") {
        return Err("User non enregistré".to_string()) // rajouter un moyen de savoir qui n'est pas enregistré
    } else if retour_check.clone() == String::from("Session non initialisé") {
        let receiver_keys: structures::KeysPubOutput = client.post(format!("{}/init_session", url)).json(&UserID{id: input.receiver.id.clone()})
            .send().await.unwrap()
            .json().await.unwrap();

        if !signe::check(receiver_keys.signed_key.clone(), receiver_keys.signature, receiver_keys.id_key.clone()) {
            return Err("La clef signé du receveur ne correspond pas à la signature".to_string());
        }
        let clef_ephemeres = keygen::kpgen();

        let path_sender_keys = format!("{}/keys.json", input.sender.id);
        let file_content = fs::read_to_string(&path_sender_keys).await.unwrap();
        let mut sender_keys: structures::KeysPriv = serde_json::from_str(&file_content).unwrap();

        let shared_key = keygen::skgen(sender_keys.id_key.clone(), clef_ephemeres.0.clone(), receiver_keys.id_key.clone(), receiver_keys.signed_key.clone(), receiver_keys.one_time_key.clone());

        let ciphertext = enc::aesgcm(input.data, keygen::aad_gen(sender_keys.id_key.clone(), receiver_keys.id_key.clone()),shared_key);  

        let data_to_send = InitOutput {
            session: input_session,
            id_key: keygen::priv_to_pub(sender_keys.id_key.clone()),
            temp_key: clef_ephemeres.1,
            signed_key_id: hash::sha512(receiver_keys.signed_key),
            one_time_key_id: hash::sha512(receiver_keys.one_time_key),
            cipher: encode(ciphertext),
        };

        eprintln!("{}", encode(shared_key));

        client.post(format!("{}/premier_message", url)).json(&data_to_send).send().await.unwrap();
        sender_keys.root_keys.insert(input.receiver.id.clone(), encode(shared_key));
        fs::write(&path_sender_keys, serde_json::to_string(&sender_keys).unwrap()).await.unwrap();
    } else {
        // let _session_data: SessionData = client.post(format!("{}/fetch_session_data", url)).json(&UserID{id: input.receiver.id.clone()})
        //     .send().await.unwrap()
        //     .json().await.unwrap();

        let path_sender_keys = format!("{}/keys.json", input.sender.id);
        let file_content = fs::read_to_string(&path_sender_keys).await.unwrap();
        let mut sender_keys: structures::KeysPriv = serde_json::from_str(&file_content).unwrap();

        match sender_keys.root_keys.get(&input.receiver.id) {
            Some(x) => println!("{}, récup", x),
            None => {
                let last_msg: Message = client.post(format!("{}/feth_dernier_message", url)).json(&input_session)
                    .send().await.unwrap()
                    .json().await.unwrap();

                    let one_time_key = match sender_keys.one_time_keys.get(&last_msg.one_time_key_id) {
                        Some(x) => x.clone(),
                        None => {eprintln!("pas de otk");String::from("")},
                    };
                    //  pas sur de ce que j'ai fait, il y a moy que la signed key ne soit plus la bonne si je la change avant de répondre, il faudrait stocker mes signed key
                    let shared_key = keygen::skrecup(sender_keys.id_key.clone(), last_msg.temp_key.clone(), last_msg.id_key.clone(), sender_keys.signed_key.clone(), one_time_key.clone());
                    
                    // priv // pub // pub // priv
                    eprintln!("{}", encode(shared_key))
            }
        }

        // si le clef publique du receveur est différente de celle du dernier message
            // recalculer la root key
        // sinon on récupère la root key stocké

        // utiliser le symemetric-key ratchet pour générer la clef du message


        // signed key c'est les clefs utilisé dans le dh du dh ratchet
    }

    keygen::test2();
    Ok(())
}

// ATTENTION IL FAUT TOUT ZEROIZE APRES

// rajouter un header dans les messages pour garder l'ordre <- pour savoir quelle clef root utiliser