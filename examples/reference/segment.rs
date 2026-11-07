fn main() {
    // Iterative sum tree: leaves start at n, parents contain child sums.
    let values = [3,
 1,
 4,
 2];
 let n = values.len();
    let mut tree = vec![0;
 2 * n];
 tree[n..].copy_from_slice(&values);
    for i in (1..n).rev() {
 tree[i] = tree[2 * i] + tree[2 * i + 1];
 }
    let (mut left,
 mut right) = (1 + n,
 4 + n);
 // half-open [1, 4)
    let mut sum = 0;
    while left < right {
        if left % 2 == 1 {
 sum += tree[left];
 left += 1;
 }
        if right % 2 == 1 {
 right -= 1;
 sum += tree[right];
 }
        left /= 2;
 right /= 2;
    }
    assert_eq!(sum,
 7);
}
