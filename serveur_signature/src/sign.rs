use rand::rngs::OsRng;
use ed25519_dalek::{SigningKey as SigningKeyED, VerifyingKey as VerifyingKeyED, Signature as SignatureED, Signer, Verifier as VerifierED};
use std::{
    fs::File,
    io::{Write, Read},
};
use crate::{
    Json, encode, decode, 
    structures::{HashedData, OutputData, CheckData},
};

pub async fn keygen() {
    let mut csprng = OsRng;
    let signing_key: SigningKeyED = SigningKeyED::generate(&mut csprng);

    eprintln!("{}", encode(signing_key.as_bytes()));
    let mut file = File::create("signing_key.key").unwrap();
    file.write_all(signing_key.as_bytes()).unwrap(); 
}

fn signe_ed25519(message: &[u8]) -> (VerifyingKeyED, SignatureED) {
    let mut file = File::open("signing_key.key").unwrap();
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();

    let signing_key: SigningKeyED = SigningKeyED::from_bytes(&data.try_into().unwrap());
    let verifying_key: VerifyingKeyED = signing_key.verifying_key();
    
    let signature: SignatureED = signing_key.sign(message);
    
    (verifying_key, signature)
}

pub async fn sign(Json(payload): Json<HashedData>) -> Json<OutputData> {
    let (_id, _method) = (payload.id, payload.methode);

    let (verif_key, signature) = signe_ed25519(&decode(payload.hash).unwrap());
    Json(OutputData{
        signature: encode(signature.to_bytes()),
        verif_key: encode(verif_key.to_bytes())
    })
}

pub async fn check(Json(payload): Json<CheckData>) {
    let verif_key = VerifyingKeyED::from_bytes(&decode(payload.verif_key).unwrap().try_into().unwrap()).unwrap();
    let signature = SignatureED::from_bytes(&decode(payload.signature).unwrap().try_into().unwrap());
    assert!(verif_key.verify(&decode(payload.hash).unwrap(), &signature).is_ok());
}