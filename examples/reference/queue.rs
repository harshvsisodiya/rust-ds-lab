use std::collections::VecDeque;
fn main() {
    let mut queue = VecDeque::new();
    queue.push_back("first");
    queue.push_back("second");
    assert_eq!(queue.pop_front(),
 Some("first"));
    assert_eq!(queue.front(),
 Some(&"second"));
}
