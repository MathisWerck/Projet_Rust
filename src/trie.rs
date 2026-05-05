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

    pub fn insert(&mut self, _number: &str, _name: &str) {
        todo!()
    }
}

impl Default for Trie {
    fn default() -> Self {
        Self::new()
    }
}
