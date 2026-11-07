fn main() {
    let mut paths = [[0u64;
 3];
 3];
 paths[0][0] = 1;
    for row in 0..3 {
 for col in 0..3 {
        if (row,
 col) == (1,
 1) {
 continue;
 }
 // blocked square
        if row > 0 {
 paths[row][col] += paths[row - 1][col];
 }
        if col > 0 {
 paths[row][col] += paths[row][col - 1];
 }
    }
}
    assert_eq!(paths[2][2],
 2);
}
