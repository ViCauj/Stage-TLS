use std::io::{self, Read};
use reqwest::Client;
use structures::{InitOutput, Messages, Message, Session, UserID, KeysPub, DataToSend};
use tokio::fs;
use hex::{encode, decode};
use indexmap::IndexMap;

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
        sender: UserID {id: input.sender.id.clone()},   // moi
        receiver: UserID {id: input.receiver.id.clone()},   // lui
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

        // calcul clef commune
        let shared_key = keygen::skgen(sender_keys.id_key.clone(), clef_ephemeres.0.clone(), receiver_keys.id_key.clone(), receiver_keys.signed_key.clone(), receiver_keys.one_time_key.clone());

        // init double ratchet
        let send_chain_key = &mut sender_keys.send_key;
        let (current_root_key, current_chain_key) = keygen::kdf_rk(shared_key, keygen::dh(sender_keys.signed_keys.last().unwrap().1.clone(), receiver_keys.signed_key.clone()));
        let mut root_keys = IndexMap::new();
        root_keys.insert(structures::serial_tuple((sender_keys.signed_keys.last().unwrap().0.clone(), receiver_keys.signed_key.clone())), encode(current_root_key));
        sender_keys.root_keys.insert(input.receiver.id.clone(), root_keys);
        let (_new_chain_key, message_key) = keygen::kdf_ck(current_chain_key); // car on envoi directement un message
        send_chain_key.insert(input.receiver.id.clone(), encode(current_chain_key));

        fs::write(&path_sender_keys, serde_json::to_string(&sender_keys).unwrap()).await.unwrap();
        let ciphertext = enc::aesgcm(input.data, keygen::aad_gen(keygen::priv_to_pub(sender_keys.id_key.clone()), receiver_keys.id_key.clone()), message_key);
        let data_to_send = InitOutput {
            session: input_session,
            id_key: keygen::priv_to_pub(sender_keys.id_key.clone()),
            temp_key: clef_ephemeres.1,
            signed_keys: (receiver_keys.signed_key.clone(), sender_keys.signed_keys.last().unwrap().0.clone()),
            one_time_key_id: hash::sha512(receiver_keys.one_time_key),
            cipher: encode(ciphertext),
        };
        client.post(format!("{}/premier_message", url)).json(&data_to_send).send().await.unwrap();
    } else {
        let path_sender_keys = format!("{}/keys.json", input.sender.id);
        let file_content = fs::read_to_string(&path_sender_keys).await.unwrap();
        let mut sender_keys: structures::KeysPriv = serde_json::from_str(&file_content).unwrap();

        let messages: Messages = client.post(format!("{}/fetch_messages_sender", url)).json(&input_session)
            .send().await.unwrap()
            .json().await.unwrap();

        let receiver_keys: KeysPub = client.post(format!("{}/get_keys", url)).json(&UserID {id: input.receiver.id.clone()})
            .send().await.unwrap()
            .json().await.unwrap();

        // on récupère la dernière root key calculé ou bien on met fin à l'initialisation en calculant la première root key
        let (my_old_signed_key, his_old_signed_key, root_key): (String, String, [u8; 32]) = match sender_keys.root_keys.get(&input.receiver.id) {
            Some(x) => {
                let (tuple, c) = x.last().unwrap();
                let (a, b) = structures::deserial_tuple(tuple.clone());
                (a.clone(), b.clone(), decode(c).unwrap().try_into().unwrap())
            },
            None => {
                let first_msg = messages.premier_message;

                let one_time_key = match sender_keys.one_time_keys.get(&first_msg.one_time_key_id) {
                    Some(x) => x.clone(),
                    None => {eprintln!("pas de otk");String::from("")},
                };
                
                // calcul clef commune
                let shared_key = keygen::skrecup(first_msg.id_key.clone(), first_msg.temp_key.clone(), sender_keys.id_key.clone(), sender_keys.signed_keys.get(&first_msg.signed_keys.0.clone()).unwrap().clone(), one_time_key.clone());

                // init double ratchet
                let send_chain_key = &mut sender_keys.send_key;
                let (current_root_key, current_chain_key) = keygen::kdf_rk(shared_key, keygen::dh(sender_keys.signed_keys.last().unwrap().1.clone(), receiver_keys.signed_key.clone()));
                let mut root_keys = IndexMap::new();
                root_keys.insert(structures::serial_tuple((sender_keys.signed_keys.last().unwrap().0.clone(), receiver_keys.signed_key.clone())), encode(current_root_key));
                sender_keys.root_keys.insert(input.receiver.id.clone(), root_keys);
                send_chain_key.insert(input.receiver.id.clone(), encode(current_chain_key));

                fs::write(&path_sender_keys, serde_json::to_string(&sender_keys).unwrap()).await.unwrap();
                (first_msg.signed_keys.0, first_msg.signed_keys.1, current_root_key)
            }
        };

        // rajouter un tcheck su ma signed key
        if receiver_keys.signed_key == his_old_signed_key {
            let mut _current_send_key = sender_keys.send_key.get(&input_session.receiver.id).unwrap();
            let (new_send_key, message_key) = keygen::kdf_ck(decode(_current_send_key).unwrap().try_into().unwrap());
            _current_send_key = &encode(new_send_key);

            let ciphertext = enc::aesgcm(input.data, keygen::aad_gen(keygen::priv_to_pub(sender_keys.id_key.clone()), receiver_keys.id_key), message_key);  
  
            let message = Message { 
                signed_keys: (his_old_signed_key, my_old_signed_key), // on inverse pour que le format reste le même (ce message sera envoyé chez lui ou l'ordre des clef est inversé)
                cipher: encode(&ciphertext)
            };

            client.post(format!("{}/send", url)).json(&DataToSend{message: message, session: input_session}).send().await.unwrap();
        } else {
            // on met à jour 

            // pour savoir ou j'en suis il faut que je regarde la clef que j'ai utilisé dans le dernier message que j'ai envoyé et générer des rk jusqu'à ce que la chaine soit au même endroit que les messages que j'ai reçus, quitte à créer des (k,v) avec v vide
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