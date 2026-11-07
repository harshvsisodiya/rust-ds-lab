fn add(bit: &mut [i64],
 mut i: usize,
 delta: i64) {
    assert!(i > 0);
 // Fenwick indices begin at one
    while i < bit.len() {
 bit[i] += delta;
 i += i & i.wrapping_neg();
 }
}
fn prefix(bit: &[i64],
 mut i: usize) -> i64 {
    let mut total = 0;
    while i > 0 {
 total += bit[i];
 i &= i - 1;
 }
    total
}
fn main() {
    let mut bit = vec![0;
 5];
    for (i,
 value) in [3,
 1,
 4,
 2].into_iter().enumerate() {
 add(&mut bit,
 i + 1,
 value);
 }
    assert_eq!(prefix(&bit,
 3),
 8);
    assert_eq!(prefix(&bit,
 4) - prefix(&bit,
 2),
 6);
}
