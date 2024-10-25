use rand::rngs::OsRng;
use ed25519_dalek::{SigningKey as SigningKeyED, VerifyingKey as VerifyingKeyED, Signature as SignatureED, Signer, Verifier as VerifierED};
use rsa::{
    RsaPrivateKey,
    pkcs1v15::{SigningKey as SigningKeyRSA, VerifyingKey as VerifyingKeyRSA, Signature as SignatureRSA},
    signature::{Keypair, RandomizedSigner},
    sha2::Sha256
};


pub fn signe_ed25519(message: &[u8]) -> (VerifyingKeyED, SignatureED) {
    let mut csprng = OsRng;

    let signing_key: SigningKeyED = SigningKeyED::generate(&mut csprng);
    let verifying_key: VerifyingKeyED = signing_key.verifying_key();
    
    let signature: SignatureED = signing_key.sign(message);
    
    (verifying_key, signature)
}

pub fn signe_rsa(message: &[u8]) -> (VerifyingKeyRSA<Sha256> , SignatureRSA){
    let mut csprng = OsRng;

    let private_key = RsaPrivateKey::new(&mut csprng, 2048).unwrap();
    let signing_key = SigningKeyRSA::<Sha256>::new(private_key);
    let verifying_key: VerifyingKeyRSA<Sha256> = signing_key.verifying_key();

    let signature: SignatureRSA = signing_key.sign_with_rng(&mut csprng, message);

    (verifying_key, signature)
}

pub fn check_rsa(message: &[u8], verifying_key: VerifyingKeyRSA<Sha256>, signature: SignatureRSA) {
    verifying_key.verify(message, &signature).unwrap();
}

pub fn check_ed25519(message: &[u8], verifying_key: VerifyingKeyED, signature: SignatureED) {
    assert!(verifying_key.verify(message, &signature).is_ok());
}