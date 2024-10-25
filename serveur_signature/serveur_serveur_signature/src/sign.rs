use ed25519_dalek::{SigningKey as SigningKeyED, VerifyingKey as VerifyingKeyED, Signature as SignatureED, Signer, Verifier as VerifierED};
use std::{
    fs::File,
    io::Write,
};
use crate::{
    decode, encode, 
    structures::{CheckDataHashed, Hash, OutputData}, 
    Json, Extension, Arc, Mutex,
};

pub async fn _keygen() {
    let mut csprng = rand::rngs::OsRng;
    let signing_key: SigningKeyED = SigningKeyED::generate(&mut csprng);

    let mut file = File::create("signing_key.bin").unwrap();
    file.write_all(signing_key.as_bytes()).unwrap(); 
}

pub async fn sign(Extension(state): Extension<Arc<Mutex<[u8; 32]>>>, Json(payload): Json<Hash>) -> Json<OutputData> {
    let signing_key: SigningKeyED = SigningKeyED::from_bytes(&*state.lock().await);
    let signature: SignatureED = signing_key.sign(payload.hash.as_bytes());
    Json(OutputData{
        signature: encode(signature.to_bytes())
    })
}

pub async fn check(Extension(state): Extension<Arc<Mutex<[u8; 32]>>>, Json(payload): Json<CheckDataHashed>) {
    let signing_key: SigningKeyED = SigningKeyED::from_bytes(&*state.lock().await);
    let verifying_key: VerifyingKeyED = signing_key.verifying_key();
    let signature = SignatureED::from_bytes(&decode(payload.signature).unwrap().try_into().unwrap());
    assert!(verifying_key.verify(payload.hash.as_bytes(), &signature).is_ok());
}