fn stairs(n: usize) -> u64 {
    let (mut previous,
 mut current) = (1,
 1);
    for _ in 1..n {
 (previous,
 current) = (current,
 previous + current);
 }
    current
}
fn main() {
 assert_eq!(stairs(5),
 8);
 }
