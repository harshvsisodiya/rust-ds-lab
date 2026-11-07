fn gcd(mut a: u64,
 mut b: u64) -> u64 {
    while b != 0 {
 (a,
 b) = (b,
 a % b);
 }
    a
}
fn main() {
 assert_eq!(gcd(48,
 18),
 6);
 }
