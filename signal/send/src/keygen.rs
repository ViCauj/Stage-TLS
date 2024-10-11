use ed25519_dalek::{SigningKey, VerifyingKey};
use hex::encode;
use x25519_dalek::{StaticSecret, PublicKey};
use pkcs8::{
    EncodePrivateKey, EncodePublicKey, DecodePublicKey, DecodePrivateKey
};
use sha2::Sha512;
use hkdf::Hkdf;

pub fn kpgen() -> (String, String) {
    let mut rng = rand::rngs::OsRng;
    let signing_key: SigningKey = SigningKey::generate(&mut rng);
    let verifying_key = VerifyingKey::from(&signing_key);
    let pem_priv = signing_key.to_pkcs8_pem(pkcs8::LineEnding::LF).unwrap();
    let pem_pub = verifying_key.to_public_key_pem(pkcs8::LineEnding::LF).unwrap();
    (String::from(pem_priv.as_str()), pem_pub)
}

pub fn priv_to_pub(priv_key_pem: String) -> String {
    let signing_key =  SigningKey::from_pkcs8_pem(&priv_key_pem).unwrap();
    let verifying_key = VerifyingKey::from(&signing_key);
    verifying_key.to_public_key_pem(pkcs8::LineEnding::LF).unwrap()
}

fn pub_pem_to_dh_key(key_pem: String) -> PublicKey {
    PublicKey::from(*VerifyingKey::from_public_key_pem(&key_pem).unwrap().as_bytes())
}

fn priv_pem_to_dh_key(key_pem: String) -> StaticSecret {
    StaticSecret::from(*SigningKey::from_pkcs8_pem(&key_pem).unwrap().as_bytes())
}

fn kfd(dh_conc: Vec<u8>) -> [u8; 32] {
    let mut input = vec![255;32];
    input.extend(dh_conc);
    let salt = [0u8;64];   // doit faire la taille de l'output du hash i.e 64 bytes (512 bits)
    let info = [];  // pas d'info pour le moment

    let hk = Hkdf::<Sha512>::new(Some(&salt), &input);
    let mut output = [0u8; 32];    // impose la taille de la clef dérivée 
    hk.expand(&info, &mut output).unwrap();

    output
}

pub fn skgen(id_key_sender: String, ephemeral_key: String, id_key_receiver: String, signed_key: String, one_time_key: String) -> [u8; 32] {
    let sender_keys = (priv_pem_to_dh_key(id_key_sender), priv_pem_to_dh_key(ephemeral_key));
    let receiver_keys = (pub_pem_to_dh_key(id_key_receiver), pub_pem_to_dh_key(signed_key));

    let mut dh = Vec::new();
    dh.extend(sender_keys.0.diffie_hellman(&receiver_keys.1).as_bytes());
    dh.extend(sender_keys.1.diffie_hellman(&receiver_keys.0).as_bytes());
    dh.extend(sender_keys.1.diffie_hellman(&receiver_keys.1).as_bytes());

    if !one_time_key.is_empty() {
        dh.extend(sender_keys.1.diffie_hellman(&pub_pem_to_dh_key(one_time_key)).as_bytes());
    }

    eprintln!("dh : {}", encode(&dh));

    kfd(dh)
}

pub fn skrecup(id_key_sender: String, ephemeral_key: String, id_key_receiver: String, signed_key: String, one_time_key: String) -> [u8; 32] {
    let sender_keys = (priv_pem_to_dh_key(id_key_sender), priv_pem_to_dh_key(signed_key));
    let receiver_keys = (pub_pem_to_dh_key(id_key_receiver), pub_pem_to_dh_key(ephemeral_key));

    let mut dh = Vec::new();
    dh.extend(sender_keys.1.diffie_hellman(&receiver_keys.0).as_bytes());
    dh.extend(sender_keys.0.diffie_hellman(&receiver_keys.1).as_bytes());
    dh.extend(sender_keys.1.diffie_hellman(&receiver_keys.0).as_bytes());

    if !one_time_key.is_empty() {
        dh.extend(priv_pem_to_dh_key(one_time_key).diffie_hellman(&receiver_keys.1).as_bytes());
    }

    eprintln!("dh : {}", encode(&dh));

    kfd(dh)
}

pub fn aad_gen(id_key_sender: String, id_key_receiver: String) -> Vec<u8> {
    let mut res = Vec::new();
    res.extend(VerifyingKey::from(&SigningKey::from_pkcs8_pem(&id_key_sender).unwrap()).as_bytes());
    res.extend(VerifyingKey::from_public_key_pem(&id_key_receiver).unwrap().as_bytes());

    res
}

pub fn _dh(private_pem: String, public_pem: String) -> [u8; 32] {
    let private = priv_pem_to_dh_key(private_pem);
    let public = pub_pem_to_dh_key(public_pem);

    *private.diffie_hellman(&public).as_bytes()
}

// KDF du symemetric-key ratchet
pub fn _kdf_ck(chain_key: [u8; 32]) -> ([u8; 32], [u8; 32]) {
    let hk = Hkdf::<Sha512>::new(None, &chain_key);

    let (mut new_chain_key , mut message_key) = ([0u8; 32], [0u8;32]);
    hk.expand(&[0x01], &mut new_chain_key).unwrap();
    hk.expand(&[0x02], &mut message_key).unwrap();

    (new_chain_key, message_key)
}

// KDF du diffie-hellman ratchet
pub fn _kdf_rk(root_key: [u8;32], dh_out: [u8;32]) -> ([u8; 32], [u8; 32]) {
    let hk = Hkdf::<Sha512>::new(Some(&root_key), &dh_out);
    
    let (mut new_root_key, mut chain_key) = ([0u8; 32], [0u8; 32]);
    // je devrais rajouter de l'info
    hk.expand(&[0x01], &mut new_root_key).unwrap();
    hk.expand(&[0x02], &mut chain_key).unwrap();

    (new_root_key, chain_key)
}

pub fn _test() {
    let key_a = kpgen();
    let key_b = kpgen();

    println!("\nTEST");

    println!("{}", encode(priv_pem_to_dh_key(key_a.0).diffie_hellman(&pub_pem_to_dh_key(key_b.1))));
    println!("{}", encode(priv_pem_to_dh_key(key_b.0).diffie_hellman(&pub_pem_to_dh_key(key_a.1))));
}

pub fn test2() {
    let (a_priv, b_priv) = (StaticSecret::random_from_rng(&mut rand::rngs::OsRng), StaticSecret::random_from_rng(&mut rand::rngs::OsRng));
    let (a_pub, b_pub) = (PublicKey::from(&a_priv), PublicKey::from(&b_priv));

    println!("\nTEST");

    println!("{}", encode(a_priv.diffie_hellman(&b_pub)));
    println!("{}", encode(b_priv.diffie_hellman(&a_pub)));
}

// PB AVEC MES CLEFS, pas bien converti!!!!!!!!!! ou pas bone du tout ??????????????