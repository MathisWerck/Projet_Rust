pub struct TrieNode {
    pub children: Vec<(char, TrieNode)>,
    pub name: Option<String>,
}
