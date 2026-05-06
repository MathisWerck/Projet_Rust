use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Contact {
    pub nb: String,
    pub name: String,
}

pub fn load_contacts(path: &str) -> Vec<Contact> {
    let content = fs::read_to_string(path)
        .expect("Impossible de lire le fichier JSON");
    serde_json::from_str(&content)
        .expect("Impossible de parser le JSON")
}