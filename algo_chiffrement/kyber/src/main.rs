use pqc_kyber::*;

fn main() -> Result<(), KyberError> {
    let mut rng = rand::thread_rng();

// Initialize the key exchange structs
    let mut alice = Uake::new();
    let mut bob = Uake::new();

    // Generate Keypairs
    let alice_keys = keypair(&mut rng)?;
    let bob_keys = keypair(&mut rng)?;

    // Alice initiates key exchange
    let client_init = alice.client_init(&bob_keys.public, &mut rng)?;

    // Bob authenticates and responds
    let server_send = bob.server_receive(
    client_init, &bob_keys.secret, &mut rng
    )?;

    // Alice decapsulates the shared secret
    alice.client_confirm(server_send)?;

    // Both key exchange structs now have the shared secret
    assert_eq!(alice.shared_secret, bob.shared_secret);

    Ok(())
}
