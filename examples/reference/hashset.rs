use std::collections::HashSet;
fn main() {
    let mut seen = HashSet::new();
    assert!(seen.insert("cat"));
    assert!(!seen.insert("cat"));
 // duplicates do not add another member
    seen.insert("dog");
    assert!(seen.contains("cat"));
    assert_eq!(seen.len(),
 2);
}
