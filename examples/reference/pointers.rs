fn pair(values: &[i32],
 target: i32) -> Option<(usize,
 usize)> {
    if values.len() < 2 {
 return None;
 }
    let (mut left,
 mut right) = (0,
 values.len() - 1);
    while left < right {
        match (values[left] + values[right]).cmp(&target) {
            std::cmp::Ordering::Equal => return Some((left,
 right)),
            std::cmp::Ordering::Less => left += 1,
            std::cmp::Ordering::Greater => right -= 1,
        }
    }
    None
}
fn main() {
 assert_eq!(pair(&[1,
 3,
 5,
 8],
 9),
 Some((0,
 3)));
 }
