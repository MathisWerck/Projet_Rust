use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Contact {
    pub nb: String,
    pub name: String,
}

pub fn load_contacts(_path: &str) -> Vec<Contact> {
    todo!()
}