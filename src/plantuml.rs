use crate::trie::{Trie, TrieNode};

// Fonction interne qui parcourt le trie et génère les lignes
fn write_node(node: &TrieNode, depth: usize, lines: &mut Vec<String>) {
    for (ch, child) in &node.children {
        let stars = "*".repeat(depth + 1);
        lines.push(format!("{} {}", stars, ch));
        if let Some(name) = &child.name {
            let name_stars = "*".repeat(depth + 2);
            lines.push(format!("{} {}", name_stars, name));
        }
        write_node(child, depth + 1, lines);
    }
}

pub fn generate_puml(trie: &Trie, output_path: &str) {
    let mut lines = Vec::new();
    lines.push("@startmindmap".to_string());
    write_node(&trie.root, 0, &mut lines);
    lines.push("@endmindmap".to_string());

    // Pour l'instant on affiche juste dans la console
    for line in &lines {
        println!("{}", line);
    }

    let _ = output_path; // sera utilisé à l'étape suivante
}