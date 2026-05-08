pub struct TrieNode {
    pub children: Vec<(char, TrieNode)>,
    pub name: Option<String>,
}

impl TrieNode {
    pub fn new() -> TrieNode {
        TrieNode {
            children: Vec::new(),
            name: None,
        }
    }
}

impl Default for TrieNode {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Trie {
    pub root: TrieNode,
}

impl Trie {
    pub fn new() -> Trie {
        Trie {
            root: TrieNode::new(),
        }
    }

    pub fn insert(&mut self, number: &str, name: &str) {
        let mut current = &mut self.root;
        for ch in number.chars() {
            let i = match current.children.iter().position(|(c, _)| *c == ch) {
                Some(i) => i,
                None => {
                    current.children.push((ch, TrieNode::new()));
                    current.children.len() - 1
                }
            };
            current = &mut current.children[i].1;
        }
        current.name = Some(name.to_string());
    }
}

impl Default for Trie {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_creates_root_child() {
        let mut trie = Trie::new();
        trie.insert("112", "Urgences");
        assert_eq!(trie.root.children.len(), 1);
        assert_eq!(trie.root.children[0].0, '1');
    }

    #[test]
    fn test_insert_common_prefix() {
        let mut trie = Trie::new();
        trie.insert("15", "SAMU");
        trie.insert("112", "Urgences");
        // La racine a un seul enfant '1'
        assert_eq!(trie.root.children.len(), 1);
        // Le noeud '1' a deux enfants : '5' et '1'
        assert_eq!(trie.root.children[0].1.children.len(), 2);
    }

    #[test]
    fn test_name_stored_at_leaf() {
        let mut trie = Trie::new();
        trie.insert("15", "SAMU");
        let node_1 = &trie.root.children[0].1;
        let node_5 = &node_1.children[0].1;
        assert_eq!(node_5.name, Some("SAMU".to_string()));
    }

    #[test]
    fn test_no_duplicate_nodes() {
        let mut trie = Trie::new();
        trie.insert("15", "SAMU");
        trie.insert("15", "SAMU_doublon");
        assert_eq!(trie.root.children.len(), 1);
    }
}
