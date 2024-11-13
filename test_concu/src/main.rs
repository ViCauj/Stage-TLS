use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

fn main() {
    // Crée un Mutex pour partager une HashMap entre les threads
    let hashmap = Arc::new(Mutex::new(HashMap::new()));

    // Crée un nouveau thread qui exécute une boucle infinie
    let hashmap_clone = Arc::clone(&hashmap);
    thread::spawn(move || {
        let mut i = 0;
        loop {
            if let Ok(mut hashmap) = hashmap_clone.try_lock() {
                hashmap.insert(format!("clé {}", i), format!("valeur {}", i));
                println!("Boucle 1 : inséré élément {}, hashmap len = {}", i, hashmap.len());
                i += 1;
            }
            thread::sleep(Duration::from_millis(10000));
        }
    });

    println!("Programme principal en cours d'exécution...");
    loop {
        if let Ok(mut hashmap) = hashmap.try_lock() {
            hashmap.clear();
            println!("Programme principal : hashmap vidé, len = {}", hashmap.len());
        }
        thread::sleep(Duration::from_millis(500));
    }
}