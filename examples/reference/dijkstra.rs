fn main() {
    let graph = [vec![(1,
 2),
 (2,
 7)],
 vec![(2,
 1)],
 vec![]];
    let mut distance = vec![i64::MAX;
 graph.len()];
    let mut settled = vec![false;
 graph.len()];
 distance[0] = 0;
    for _ in 0..graph.len() {
        let next = (0..graph.len()).filter(|&i| !settled[i]).min_by_key(|&i| distance[i]);
        let Some(v) = next else {
 break;
 }
;
        if distance[v] == i64::MAX {
 break;
 }
 settled[v] = true;
        for &(to,
 weight) in &graph[v] {
            assert!(weight >= 0);
 distance[to] = distance[to].min(distance[v] + weight);
        }
    }
    assert_eq!(distance,
 [0,
 2,
 3]);
}
