use cryptoki::{
    context::{CInitializeArgs, Pkcs11}, 
    session::UserType, 
    types::AuthPin,
    // mechanism::{Mechanism,aead::GcmParams},
    mechanism::Mechanism,
    // object::{Attribute, ObjectClass, KeyType},
    // object::Attribute,
};
use hex::{decode, encode};

// POUR AFFICHER TOUS LES ATTRIBUTS
mod attributes;
use attributes::get_attribute_types;

const IV: &str = "00000000000000000000000000000000";
const PLAINTEXT: &str = "";

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

    // let aes_key_attributes = vec![
    //         Attribute::Id(vec![1]),
    //         Attribute::Label(vec![99, 108, 101, 102]),
    //         Attribute::Class(ObjectClass::SECRET_KEY),
    //         Attribute::KeyType(KeyType::AES),
    //         Attribute::ValueLen(32.into()),
    //         // Attribute::Value(vec![0u8; 32]),
    //         Attribute::Encrypt(true),
    //         Attribute::Decrypt(true),
    //         Attribute::Token(true),
    //     ];
    // let clef = session.generate_key(&Mechanism::AesKeyGen, &aes_key_attributes).unwrap();

    // On peut aussi la créer manuellement (pour choisir la valeur) puis l'importer (voir les tests)

    // ================================================================================
    // RECUP OBJ
    // ================================================================================

    let empty_attrib= vec![];
    let obj = session.find_objects(&empty_attrib).unwrap();


    // ================================================================================
    // AFFICHE OBJ 
    // ================================================================================

    println!("\nobjets :");
    let attribute_types = get_attribute_types();
    for o in &obj {
        let attributes = session.get_attributes(*o, &attribute_types).unwrap();
        for attribute in attributes {
            println!("{:?}", attribute);
        }
    }   


    // ================================================================================
    // CHIFFRER DECHIFFRER (on créer une clef qu'on détruit après)
    // ================================================================================

    print!("\n");

    // CBC
    let data: Vec<u8> = decode(PLAINTEXT).unwrap();
    let iv: [u8; 16] = decode(IV).unwrap().try_into().unwrap();
    let enc_data: Vec<u8> = session.encrypt(&Mechanism::AesCbc(iv), obj[0], &data).unwrap();
    let dec_data = session.decrypt(&Mechanism::AesCbc(iv), obj[0], &enc_data).unwrap();
    
    // GCM
    // let data: Vec<u8> = decode(PLAINTEXT).unwrap();
    // let iv: Vec<u8> = decode(IV).unwrap();
    // let aad: Vec<u8> = decode(AAD).unwrap();

    // let gcm_param = GcmParams::new(&iv, &aad, (TAG_SIZE*8).into());
    
    // let mut enc_data: Vec<u8> = session.encrypt(&Mechanism::AesGcm(gcm_param), obj[0], &data).unwrap();
    // let dec_data: Vec<u8> = session.decrypt(&Mechanism::AesGcm(gcm_param), obj[0], &enc_data).unwrap();
    
    // let auth_tag = enc_data.split_off(((enc_data.len() as u64)-TAG_SIZE) as usize);

    println!("PLAINTEXT : {}", encode(&dec_data));
    println!("CIPHERTEXT : {}", encode(&enc_data));
    // println!("TAG : {}", encode(&auth_tag));
    // println!("TAG SIZE : {} byte(s)", decode(TAG).unwrap().len());


    // ================================================================================
    // SUPPRIMER UNE CLEF
    // ================================================================================


    // ================================================================================
    // VERIF QUE TOUT MARCHE BIEN
    // ================================================================================
}
