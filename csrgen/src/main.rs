use rcgen::{CertificateParams, KeyPair, SignatureAlgorithm};
use std::{
    io::{self, Read, Write, Result},
    env
};

const RSA: [u64; 7] = [1, 2, 840, 113549, 1, 1, 11]; // RSA_sha256
const ED25519: [u64; 4] = [1, 3, 101, 112];
const P256: [u64; 7] = [1, 2, 840, 10045, 4, 3, 2]; // P256_sha256

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("Manque un argument (\"rsa\", \"ed25519\" ou \"p256\" attendu)");
    }

    let sig_algo = match args[1].as_str() {
        "rsa" => SignatureAlgorithm::from_oid(&RSA),
        "ed25519" => SignatureAlgorithm::from_oid(&ED25519),
        "p256" => SignatureAlgorithm::from_oid(&P256),
        _ => {
            eprintln!("Argument non valide (\"rsa\", \"ed25519\" ou \"p256\" attendu)");
            return Ok(());
        }  
    }.unwrap();

    let mut data = Vec::new();
    io::stdin().read_to_end(&mut data)?;
    let pem = String::from_utf8(data).unwrap();

    let keypair = KeyPair::from_pkcs8_pem_and_sign_algo(&pem, sig_algo).unwrap();

    let csr = CertificateParams::new(vec![
        String::from("a"),
        ]).unwrap()
        .serialize_request(&keypair).unwrap();

    let csr_pem = csr.pem().unwrap();
    
    io::stdout().write_all((*csr_pem).as_bytes()).unwrap();
    Ok(())
}
