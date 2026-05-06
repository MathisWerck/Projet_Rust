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
            if !current.children.iter().any(|(c, _)| *c == ch) {
                current.children.push((ch, TrieNode::new()));
            }
            let i = current.children.iter().position(|(c, _)| *c == ch).unwrap();
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
