fn count_ones(mut n: u64) -> usize {
    let mut count = 0;
    while n != 0 {
 n &= n - 1;
 count += 1;
 }
    count
}
fn main() {
 assert_eq!(count_ones(0b10110),
 3);
 }
