use std::{
cmp::Reverse,
 collections::BinaryHeap}
;
fn main() {
    // BinaryHeap is a max heap; Reverse makes the smallest key pop first.
    let mut heap = BinaryHeap::new();
    for value in [12,
 7,
 24,
 9] {
 heap.push(Reverse(value));
 }
    let mut order = Vec::new();
    while let Some(Reverse(value)) = heap.pop() {
 order.push(value);
 }
    assert_eq!(order,
 [7,
 9,
 12,
 24]);
}
