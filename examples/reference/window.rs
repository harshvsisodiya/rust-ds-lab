fn max_window(values: &[i64],
 k: usize) -> Option<i64> {
    if k == 0 || k > values.len() {
 return None;
 }
    let mut sum: i64 = values[..k].iter().sum();
 let mut best = sum;
    for i in k..values.len() {
        sum += values[i] - values[i - k];
 best = best.max(sum);
    }
    Some(best)
}
fn main() {
 assert_eq!(max_window(&[1,
 4,
 2,
 7,
 3],
 3),
 Some(13));
 }
