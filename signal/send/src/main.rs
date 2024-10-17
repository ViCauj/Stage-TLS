use std::io::{self, Read};
use reqwest::Client;
use structures::{InitOutput, MessagesRecus, Session, UserID};
use tokio::fs;
use hex::{encode, decode};

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

        client.post(format!("{}/premier_message", url)).json(&data_to_send).send().await.unwrap();
        sender_keys.root_keys.insert(input.receiver.id.clone(), (encode(shared_key), sender_keys.signed_keys.last().unwrap().0.clone()));
        fs::write(&path_sender_keys, serde_json::to_string(&sender_keys).unwrap()).await.unwrap();
    } else {
        let path_sender_keys = format!("{}/keys.json", input.sender.id);
        let file_content = fs::read_to_string(&path_sender_keys).await.unwrap();
        let mut sender_keys: structures::KeysPriv = serde_json::from_str(&file_content).unwrap();

        let messages_reçus: MessagesRecus = client.post(format!("{}/fetch_messages_recus", url)).json(&input_session)
            .send().await.unwrap()
            .json().await.unwrap();

        // on récupère la dernière root key calculé ou bien on met fin à l'initialisation en calculant la première root key
        let (root_key, id_signed_key): ([u8; 32], String) = match sender_keys.root_keys.get(&input.receiver.id) {
            Some(x) => (decode(x.0.clone()).unwrap().try_into().unwrap(), x.1.clone()), 
            None => {
                let first_msg = messages_reçus.premier_message;

                let one_time_key = match sender_keys.one_time_keys.get(&first_msg.one_time_key_id) {
                    Some(x) => x.clone(),
                    None => {eprintln!("pas de otk");String::from("")},
                };
                
                let shared_key = keygen::skrecup(first_msg.id_key.clone(), first_msg.temp_key.clone(), sender_keys.id_key.clone(), sender_keys.signed_keys.get(&first_msg.signed_key_id).unwrap().clone(), one_time_key.clone());

                sender_keys.root_keys.insert(input.receiver.id.clone(), (encode(shared_key), sender_keys.signed_keys.last().unwrap().0.clone()));
                fs::write(&path_sender_keys, serde_json::to_string(&sender_keys).unwrap()).await.unwrap();
                (shared_key, sender_keys.signed_keys.last().unwrap().0.clone())
            }
        };
        
        if sender_keys.signed_keys.last().unwrap().0 == &id_signed_key {
            println!("on continu la chainnnnnnnnnnnnnnne")
        } else {
            println!("on descend la chaine des root key puis on crée la chaaaine")
        }
    }


    // en fait je m'en fiche des messages reçus une fois que j'ai regardé où j'en suis de la chaine de root key, maintenant il faut que je regarde que les messages que j'ai envoyé pour continué la sending_key_chain, la receiving_key_chain c'est bien pour déchiffrer.
    Ok(())
}


// si premier message
    // récup première root_key
// sinon si public key a changé
    // calcuer nouvelle root_key
    // calculer première chain_key
// sinon
    // calculer nouvelle chain_key