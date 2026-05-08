use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Contact {
    pub nb: String,
    pub name: String,
}

pub fn load_contacts(path: &str) -> Result<Vec<Contact>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let contacts = serde_json::from_str(&content)?;
    Ok(contacts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_single_contact() -> Result<(), Box<dyn std::error::Error>> {
        let json = r#"[{"nb": "112", "name": "Urgences"}]"#;
        let contacts: Vec<Contact> = serde_json::from_str(json)?;
        assert_eq!(contacts.len(), 1);
        assert_eq!(contacts[0].name, "Urgences");
        assert_eq!(contacts[0].nb, "112");
        Ok(())
    }

    #[test]
    fn test_deserialize_multiple_contacts() -> Result<(), Box<dyn std::error::Error>> {
        let json = r#"[
            {"nb": "112", "name": "Urgences"},
            {"nb": "15", "name": "SAMU"}
        ]"#;
        let contacts: Vec<Contact> = serde_json::from_str(json)?;
        assert_eq!(contacts.len(), 2);
        assert_eq!(contacts[1].name, "SAMU");
        assert_eq!(contacts[1].nb, "15");
        Ok(())
    }

    #[test]
    fn test_invalid_json_returns_error() {
        let result: Result<Vec<Contact>, _> = serde_json::from_str("not valid json");
        assert!(result.is_err());
    }
}