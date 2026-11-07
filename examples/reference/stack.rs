fn main() {
    let mut stack = Vec::new();
    stack.push("first");
    stack.push("second");
    assert_eq!(stack.last(),
 Some(&"second"));
    assert_eq!(stack.pop(),
 Some("second"));
    assert_eq!(stack.pop(),
 Some("first"));
}
