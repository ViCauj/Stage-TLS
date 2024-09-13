use rcgen::{CertificateParams, KeyPair};
use std::io::{self, Read, Result};

fn main() -> Result<()> {
    let mut data = Vec::new();
    io::stdin().read_to_end(&mut data)?;

    // Pour créer la keypair il faut soit transformé la data (clée privée) en PrivateKeyDer (peut être que c'est mieux du coup de directement l'exporter dans ce format dans kpgen) et utiliser try_from
    // ou alors utiliser from_der_and_sign_algo mais pas si l'option d'avant ne marche pas
    // let keypair = KeyPair::try_from(data).unwrap();
    let k = "-----BEGIN RSA PRIVATE KEY-----
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
-----END RSA PRIVATE KEY-----
";
    eprintln!("{}", k);
    let keypair = KeyPair::from_pem(&String::from(k)).unwrap();
    // eprintln!("{}", String::from_utf8(data.clone()).unwrap());
    // let keypair = KeyPair::from_pem(&String::from_utf8(data).unwrap()).unwrap();
    
    let csr = CertificateParams::new(vec![
        String::from("a"),
        ]).unwrap()
        .serialize_request(&keypair).unwrap();

    let csr_der = csr.der();
    
    eprintln!("{:?}", csr_der);

    Ok(())
}
