use std::{
    io::{self, Read}, 
    path::Path
};
use reqwest::Client;
use tokio::fs;

mod structures;

#[tokio::main]
async fn main() -> Result<(), String> {
    let mut data = String::new();
    io::stdin().read_to_string(&mut data).unwrap();

    let user: structures::User = serde_json::from_str(&data).unwrap();

    let client = Client::new();
    let url = "http://0.0.0.0:3000";

    client.post(format!("{}/suppr_user", url)).json(&user).send().await.unwrap();    

    let path = format!("{}", user.id);
    if Path::new(&path).exists() {
        fs::remove_dir_all(path).await.unwrap();
    } 

    Ok(())
}
