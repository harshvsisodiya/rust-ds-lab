#[derive(Default)]
struct Tree(Option<Box<Node>>);
struct Node {
 key: i32,
 left: Tree,
 right: Tree }
impl Tree {
    fn insert(&mut self,
 key: i32) {
        match &mut self.0 {
            None => self.0 = Some(Box::new(Node {
 key,
                left: Tree::default(),
 right: Tree::default() }
)),
            Some(n) if key < n.key => n.left.insert(key),
            Some(n) if key > n.key => n.right.insert(key),
            Some(_) => (),
 // this set ignores duplicates
        }
    }
    fn contains(&self,
 key: i32) -> bool {
        match &self.0 {
            None => false,
            Some(n) if key == n.key => true,
            Some(n) if key < n.key => n.left.contains(key),
            Some(n) => n.right.contains(key),
        }
    }
}
fn main() {
    let mut tree = Tree::default();
    for key in [12,
 7,
 24,
 9] {
 tree.insert(key);
 }
    assert!(tree.contains(9));
    assert!(!tree.contains(10));
}
