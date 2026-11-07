fn main() {
    let mut intervals = [(1,
 3),
 (2,
 5),
 (7,
 9),
 (9,
 10)];
    intervals.sort_unstable();
 let mut merged: Vec<(i32,
 i32)> = Vec::new();
    for (start,
 end) in intervals {
        if let Some(last) = merged.last_mut() {
            if start <= last.1 {
 last.1 = last.1.max(end);
 continue;
 }
        }
        merged.push((start,
 end));
    }
    assert_eq!(merged,
 [(1,
 5),
 (7,
 10)]);
 // closed intervals merge when touching
}
