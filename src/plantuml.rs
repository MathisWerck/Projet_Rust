use crate::trie::{Trie, TrieNode};
use std::fs;

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

    let content = lines.join("\n");
    fs::write(output_path, content)
        .expect("Impossible d'écrire le fichier PlantUML");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trie::Trie;

    #[test]
    fn test_generate_puml_simple() {
        let mut trie = Trie::new();
        trie.insert("15", "SAMU");
        trie.insert("112", "Urgences");

        let mut lines = Vec::new();
        lines.push("@startmindmap".to_string());
        write_node(&trie.root, 0, &mut lines);
        lines.push("@endmindmap".to_string());

        assert!(lines.contains(&"* 1".to_string()));
        assert!(lines.contains(&"*** SAMU".to_string()));
        assert!(lines.contains(&"**** Urgences".to_string()));
        assert_eq!(lines[0], "@startmindmap");
        assert_eq!(lines[lines.len() - 1], "@endmindmap");
    }
}