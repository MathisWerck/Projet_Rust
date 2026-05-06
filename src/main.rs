use phone_trie::models::load_contacts;
use phone_trie::plantuml::generate_puml;
use phone_trie::trie::Trie;

fn main() {
    let contacts = load_contacts("data/04_common_parts.json");

    let mut trie = Trie::new();
    for contact in &contacts {
        trie.insert(&contact.nb, &contact.name);
    }

    generate_puml(&trie, "graph/output.puml");
    println!("Fichier PlantUML généré : graph/output.puml");
}