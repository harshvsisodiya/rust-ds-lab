fn main() {
    let mut intervals = [(0,
 3),
 (1,
 2),
 (2,
 4),
 (4,
 7),
 (3,
 8)];
    intervals.sort_unstable_by_key(|&(_,
 end)| end);
    let mut finish = i32::MIN;
 let mut chosen = Vec::new();
    for (start,
 end) in intervals {
        if start >= finish {
 chosen.push((start,
 end));
 finish = end;
 }
    }
    assert_eq!(chosen,
 [(1,
 2),
 (2,
 4),
 (4,
 7)]);
}
