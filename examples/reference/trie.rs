use std::collections::BTreeMap;
#[derive(Default)]
struct Trie {
 terminal: bool,
 children: BTreeMap<char,
 Trie> }
impl Trie {
    fn insert(&mut self,
 word: &str) {
        let mut node = self;
        for c in word.chars() {
 node = node.children.entry(c).or_default();
 }
        node.terminal = true;
    }
    fn contains(&self,
 word: &str) -> bool {
        let mut node = self;
        for c in word.chars() {
            let Some(next) = node.children.get(&c) else {
 return false;
 }
;
            node = next;
        }
        node.terminal
    }
}
fn main() {
    let mut trie = Trie::default();
 trie.insert("cat");
 trie.insert("car");
    assert!(trie.contains("car"));
 assert!(!trie.contains("ca"));
}
