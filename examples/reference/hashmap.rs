use std::collections::HashMap;
fn main() {
    let values = [3,
 1,
 3,
 2,
 1,
 3];
    let mut counts = HashMap::new();
    for value in values {
 *counts.entry(value).or_insert(0) += 1;
 }
    assert_eq!(counts.get(&3),
 Some(&3));
    // HashMap iteration order is intentionally unspecified.
    println!("{counts:?}");
}
