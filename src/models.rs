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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_contacts() {
        let contacts = load_contacts("data/04_common_parts.json");
        assert_eq!(contacts.len(), 5);
    }

    #[test]
    fn test_first_contact_name() {
        let contacts = load_contacts("data/04_common_parts.json");
        assert_eq!(contacts[0].name, "Alice");
    }

    #[test]
    fn test_first_contact_number() {
        let contacts = load_contacts("data/04_common_parts.json");
        assert_eq!(contacts[0].nb, "0412578440");
    }
}