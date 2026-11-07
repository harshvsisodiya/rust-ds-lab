fn find(parent: &mut [usize],
 x: usize) -> usize {
    if parent[x] != x {
 parent[x] = find(parent,
 parent[x]);
 }
    parent[x]
}
fn main() {
    let mut parent: Vec<_> = (0..5).collect();
    for (a,
 b) in [(0,
 1),
 (2,
 3),
 (1,
 3)] {
        let ra = find(&mut parent,
 a);
 let rb = find(&mut parent,
 b);
        parent[rb] = ra;
 // union; production versions also track rank/size
    }
    assert_eq!(find(&mut parent,
 0),
 find(&mut parent,
 2));
    assert_ne!(find(&mut parent,
 0),
 find(&mut parent,
 4));
}
