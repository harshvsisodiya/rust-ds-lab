#[derive(Debug)]
struct Node {
 value: i32,
 next: Option<Box<Node>> }
fn reverse(mut head: Option<Box<Node>>) -> Option<Box<Node>> {
    let mut previous = None;
    while let Some(mut current) = head {
        head = current.next.take();
 // save the old next link
        current.next = previous;
  // reverse one link
        previous = Some(current);
    }
    previous
}
fn main() {
    let head = Some(Box::new(Node {
 value: 1,
        next: Some(Box::new(Node {
 value: 2,
 next: None }
)) }
));
    let reversed = reverse(head);
    assert_eq!(reversed.as_ref().map(|n| n.value),
 Some(2));
    println!("{reversed:?}");
}
