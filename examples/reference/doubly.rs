use std::collections::LinkedList;
fn main() {
    // LinkedList owns a doubly linked sequence. Its stable public API
    // supports both ends; arbitrary indexing still requires traversal.
    let mut list = LinkedList::new();
    list.push_back(2);
    list.push_front(1);
    list.push_back(3);
    assert_eq!(list.iter().rev().copied().collect::<Vec<_>>(),
 [3,
 2,
 1]);
    assert_eq!(list.pop_back(),
 Some(3));
}
