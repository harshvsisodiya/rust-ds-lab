use std::collections::VecDeque;
fn main() {
    let graph = [vec![1,
 2],
 vec![0,
 3],
 vec![0,
 3],
 vec![1,
 2]];
    let mut distance = vec![None;
 graph.len()];
    let mut queue = VecDeque::from([0]);
 distance[0] = Some(0);
    while let Some(vertex) = queue.pop_front() {
        for &neighbor in &graph[vertex] {
            if distance[neighbor].is_none() {
                distance[neighbor] = distance[vertex].map(|d| d + 1);
                queue.push_back(neighbor);
            }
        }
    }
    assert_eq!(distance,
 [Some(0),
 Some(1),
 Some(1),
 Some(2)]);
}
