struct Node {
 value: i32,
 next: usize }
fn main() {
    // Stable arena indices express logical links without raw pointers.
    let nodes = [Node {
 value: 1,
 next: 1 }
,
 Node {
 value: 2,
 next: 2 }
,
        Node {
 value: 3,
 next: 0 }
];
    let mut cursor = 0;
    let mut visited = Vec::new();
    for _ in 0..nodes.len() {
 // a count prevents an infinite traversal
        visited.push(nodes[cursor].value);
        cursor = nodes[cursor].next;
    }
    assert_eq!(visited,
 [1,
 2,
 3]);
    assert_eq!(cursor,
 0);
}
