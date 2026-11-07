use std::collections::VecDeque;
fn main() {
    let mut deque = VecDeque::from([2,
 3]);
    deque.push_front(1);
    deque.push_back(4);
    assert_eq!(deque.pop_front(),
 Some(1));
    assert_eq!(deque.pop_back(),
 Some(4));
    assert_eq!(deque,
 [2,
 3]);
}
