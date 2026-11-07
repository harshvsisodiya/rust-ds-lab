fn search(values: &[i32],
 target: i32) -> Option<usize> {
    let (mut low,
 mut high) = (0,
 values.len());
 // half-open range
    while low < high {
        let mid = low + (high - low) / 2;
        match values[mid].cmp(&target) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => low = mid + 1,
            std::cmp::Ordering::Greater => high = mid,
        }
    }
    None
}
fn main() {
 assert_eq!(search(&[1,
 3,
 5,
 8],
 5),
 Some(2));
 }
