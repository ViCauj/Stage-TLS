use cryptoki::{
    context::{CInitializeArgs, Pkcs11}, 
    session::UserType, 
    types::AuthPin,
    mechanism::{Mechanism,aead::GcmParams}, // GCM
    // mechanism::Mechanism, // CBC
};
use hex::{decode, encode};

mod bash;
use bash::{creer_clef, supr_clef};

const KEY: &str = "603DEB1015CA71BE2B73AEF0857D77811F352C073B6108D72D9810A30914DFF4";
const IV: &str = "39F23369A9D9BACFA530E26304231461";
const PLAINTEXT: &str = "";
const CIPHERTEXT: &str = "";
const AAD: &str = "";
const TAG: &str = "";
const TAG_SIZE: u64 = 16;

fn main() {
    // ================================================================================
    // INITIALISE
    // ================================================================================

    let ctx = Pkcs11::new("/usr/lib/softhsm/libsofthsm2.so").unwrap();
    ctx.initialize(CInitializeArgs::OsThreads).unwrap(); // doit être en root!


    // ================================================================================
    // RECUP SLOT
    // ================================================================================

    println!("slots :");
    let slot = ctx.get_slots_with_initialized_token().unwrap()[0];
    println!("{:?}", slot);


    // ================================================================================
    // OUVRE SESSION
    // ================================================================================

    let session = ctx.open_rw_session(slot).unwrap();
    session.login(UserType::User, Some(&AuthPin::new("1111".into()))).unwrap(); // On peut aussi mettre None à la place du PIN si on veut utiliser un mode d'authentification protégé
    

    // ================================================================================
    // CREER UNE CLEF
    // ================================================================================

    // On la créer manuellement (pour choisir la valeur) puis on l'importe dans le softhsm
    creer_clef(KEY);

    // ================================================================================
    // RECUP OBJ
    // ================================================================================

    let empty_attrib= vec![];
    let obj = session.find_objects(&empty_attrib).unwrap();

    // ================================================================================
    // CHIFFRER DECHIFFRER (on créer une clef qu'on détruit après)
    // ================================================================================

    print!("\n");

    // CBC
    // let data: Vec<u8> = decode(PLAINTEXT).unwrap();
    // let iv: [u8; 16] = decode(IV).unwrap().try_into().unwrap();
    // let enc_data: Vec<u8> = session.encrypt(&Mechanism::AesCbc(iv), obj[0], &data).unwrap();
    // let dec_data = session.decrypt(&Mechanism::AesCbc(iv), obj[0], &enc_data).unwrap();
    
    // GCM
    let data: Vec<u8> = decode(PLAINTEXT).unwrap();
    let iv: Vec<u8> = decode(IV).unwrap();
    let aad: Vec<u8> = decode(AAD).unwrap();

    let gcm_param = GcmParams::new(&iv, &aad, (TAG_SIZE*8).into());
    
    let mut enc_data: Vec<u8> = session.encrypt(&Mechanism::AesGcm(gcm_param), obj[0], &data).unwrap();
    let dec_data: Vec<u8> = session.decrypt(&Mechanism::AesGcm(gcm_param), obj[0], &enc_data).unwrap();
    
    let auth_tag = enc_data.split_off(((enc_data.len() as u64)-TAG_SIZE) as usize);

    println!("PLAINTEXT : {}", encode(&dec_data));
    println!("CIPHERTEXT : {}", encode(&enc_data));
    println!("TAG : {}", encode(&auth_tag)); // GCM
    println!("TAG SIZE : {} byte(s)", decode(TAG).unwrap().len()); // GCM


    // ================================================================================
    // SUPPRIMER UNE CLEF
    // ================================================================================
    
    // via bash (donc opensc)
    supr_clef();


    // ================================================================================
    // VERIF QUE TOUT MARCHE BIEN
    // ================================================================================

    assert!(PLAINTEXT.to_lowercase() == encode(&dec_data));
    assert!(CIPHERTEXT.to_lowercase() == encode(&enc_data));
    assert!(TAG.to_lowercase() == encode(&auth_tag)); // GCM
}


// TODO : 
//  -faire en sorte que la clef qu'on choisissent soit tjrs la bonne (verifier l'id)
//  -rejouter une version de supr_clef() sans bash.