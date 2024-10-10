use std::io::{self, Read};
use reqwest::Client;
use structures::{CheckSession, InitOutput, UserID};
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

    let input_check_session = CheckSession {
        user1: UserID {id: input.sender.id.clone()},
        user2: UserID {id: input.receiver.id.clone()},
    };
    let retour_check = client.post(format!("{}/check_session", url)).json(&input_check_session)
        .send().await.unwrap()
        .text().await.unwrap();

    if retour_check.clone() == String::from("User non enregistré") {
        return Err("User non enregistré".to_string()) // rajouter un moyen de savoir qui n'est pas enregistré
    } else if retour_check.clone() == String::from("Session non initialisé") {
        let receiver_keys: structures::KeysPubOutput = client.post(format!("{}/init_session", url)).json(&input.receiver)
            .send().await.unwrap()
            .json().await.unwrap();

        if !signe::check(receiver_keys.pre_key_signed.clone(), receiver_keys.signature, receiver_keys.id_key.clone()) {
            return Err("La clef signé du receveur ne correspond pas à la signature".to_string());
        }
        let clef_ephemeres = keygen::kpgen();

        let path_sender_keys = format!("{}/keys.json", input.sender.id);
        let file_content = fs::read_to_string(path_sender_keys).await.unwrap();
        let sender_keys: structures::KeysPriv = serde_json::from_str(&file_content).unwrap();

        let shared_sk = keygen::skgen(sender_keys.id_key.clone(), clef_ephemeres.0.clone(), receiver_keys.id_key.clone(), receiver_keys.pre_key_signed.clone(), receiver_keys.one_time_key.clone());

        let ciphertext = enc::aesgcm(input.data, keygen::aad_gen(sender_keys.id_key.clone(), receiver_keys.id_key.clone()),shared_sk);  

        let data_to_send = InitOutput {
            sender: input.sender,
            receiver: input.receiver,
            id_key: keygen::priv_to_pub(sender_keys.id_key),
            temp_key: clef_ephemeres.1,
            one_time_key_id: hash::sha512(receiver_keys.one_time_key),
            cipher: encode(ciphertext),
        };

        client.post(format!("{}/premier_message", url)).json(&data_to_send).send().await.unwrap();
    } else {
        eprintln!("il ne se passe rien pour le moment");
        // les 2 utilisateurs existent, la session à été initialisé chez au moins un des 2 donc il a déjà au moins reçu ou envoyé un message
    }

    // si premier message (dis par le serveur) {
        // on récupère les info sur le serveur pour créer la clef secrète partagé
        // i.e on configure la session avec X3DH
    // } sinon {
        // si j'ai reçu un message depuis mon dernier message {
            // rachet asymétrique
        // } sinon {
            // rachet symétrique
        // }
    // }

    Ok(())
}

// ATTENTION IL FAUT TOUT ZEROIZE APRES