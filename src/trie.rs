pub struct TrieNode {
    pub children: Vec<(char, TrieNode)>,
    pub name: Option<String>,
}

pub struct Trie {
    pub root: TrieNode,
}

impl Trie {
    pub fn new() -> Trie {
        todo!()
    }

    pub fn insert(&mut self, _number: &str, _name: &str) {
        todo!()
    }
}