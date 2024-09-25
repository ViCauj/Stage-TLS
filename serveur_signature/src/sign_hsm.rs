use cryptoki::{
    context::{CInitializeArgs, Pkcs11}, 
    mechanism::Mechanism, 
    session::UserType, 
    types::AuthPin, 
    object::Attribute, 
    session::Session,
};
use sha2::{Digest, Sha512};
use crate::{
    structures::{MergedJson, OutputData, CheckData}, 
    Json, Extension, Arc, Mutex,
    encode, decode,
};

pub fn connect() -> cryptoki::session::Session { //rajouter les détails de co en input (dans le fichier de config)
    let ctx = Pkcs11::new("/usr/lib/softhsm/libsofthsm2.so").unwrap();
    ctx.initialize(CInitializeArgs::OsThreads).unwrap(); // doit être en root!
    let slot = ctx.get_slots_with_initialized_token().unwrap()[0];
    let session = ctx.open_rw_session(slot).unwrap();
    session.login(UserType::User, Some(&AuthPin::new("1111".into()))).unwrap(); 
    session
}

pub async fn sign(Extension(state): Extension<Arc<Mutex<Session>>>, Json(payload): Json<MergedJson>) -> Json<OutputData> {
    let session = state.lock().await;
    let search_attribute = vec![
        Attribute::Private(true),
        Attribute::Id(vec![1])
    ];
    let signing_key = session.find_objects(&search_attribute).unwrap()[0];
    
    let mut hasher = Sha512::new(); // On hash le flux de donnée à signer avand de l'envoyer dans le HSM
    hasher.update(serde_json::to_string(&payload.obj).unwrap());

    let signature = session.sign(&Mechanism::Eddsa, signing_key, encode(hasher.finalize()).as_bytes()).unwrap();
    Json(OutputData{
        signature: encode(signature),
    })
}

pub async fn check(Extension(state): Extension<Arc<Mutex<Session>>>, Json(payload): Json<CheckData>) {    
    let session = state.lock().await;
    let search_attribute = vec![
        Attribute::Private(false),
        Attribute::Id(vec![1])
    ];
    let verifying_key = session.find_objects(&search_attribute).unwrap()[0]; // pareil qu'au dessus
    let signature = decode(payload.output_json.signature).unwrap();

    let mut hasher = Sha512::new();
    hasher.update(serde_json::to_string(&payload.merged_json.obj).unwrap());

    let _res = session.verify(&Mechanism::Eddsa, verifying_key, encode(hasher.finalize()).as_bytes(), &signature).unwrap();
}