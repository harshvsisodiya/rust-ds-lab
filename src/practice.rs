//! Executable teaching traces: every displayed result is computed from the input.
use crate::{curriculum::LESSONS, model};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
#[derive(Clone, Debug)]
pub struct Frame {
    pub cells: Vec<String>,
    pub active: Vec<usize>,
    pub note: String,
    pub columns: usize,
    pub edges: Vec<(usize, usize)>,
    pub diagram: Option<model::Diagram>,
}
#[derive(Clone, Debug)]
pub struct Exercise {
    pub title: String,
    pub difficulty: &'static str,
    pub prompt: String,
    pub explanation: String,
    pub cost: &'static str,
    pub frames: Vec<Frame>,
    pub answer: String,
}
fn frame(frames: &mut Vec<Frame>, cells: Vec<String>, active: Vec<usize>, note: impl Into<String>) {
    frames.push(Frame {
        cells,
        active,
        note: note.into(),
        columns: 6,
        edges: vec![],
        diagram: None,
    });
}
fn strings(a: &[i64]) -> Vec<String> {
    a.iter().map(ToString::to_string).collect()
}
pub fn exercise(topic: usize, case: usize) -> Exercise {
    let case = case.min(9);
    let seed = case as i64;
    let mut a = vec![3 + seed, 1, 4, 1 + seed, 5, 2, 6];
    if case >= 3 {
        a.extend([2, 0, 3]);
    }
    if case >= 7 {
        a.extend([-2, 7, -1, 2]);
    }
    let difficulty = if case < 3 {
        "Easy"
    } else if case < 7 {
        "Medium"
    } else {
        "Hard"
    };
    let variants = [
        "First principles",
        "Trace each change",
        "Check the boundary",
        "Repeated values",
        "A longer sequence",
        "Compare alternatives",
        "Find the invariant",
        "Mixed edge cases",
        "Prove the result",
        "Explain the cost",
    ];
    let mut e = Exercise {
        title: format!("{} · {}", case + 1, variants[case]),
        difficulty,
        prompt: String::new(),
        explanation: String::new(),
        cost: "",
        frames: vec![],
        answer: String::new(),
    };
    match topic {
        0 => {
            e.prompt = format!(
                "Count each value in {a:?}. Which values repeat, and which has the \
                        highest count? Resolve ties by the smaller value."
            );
            e.explanation = concat!(
                "Scan once. The map stores counts for the prefix already visited. ",
                "Increment only the current key. Keeping the entire prefix is ",
                "unnecessary; the counts are sufficient. The ordered map in this ",
                "teaching trace sorts keys for readable output; a hash map gives ",
                "average constant-time updates."
            )
            .into();
            e.cost = concat!(
                "Hash map: expected O(n) time, O(k) space; this ordered display ",
                "model uses O(n log k)."
            );
            let mut counts = BTreeMap::new();
            for x in &a {
                *counts.entry(*x).or_insert(0) += 1;
                let cells = counts.iter().map(|(k, v)| format!("{k}: {v}")).collect();
                frame(
                    &mut e.frames,
                    cells,
                    vec![],
                    format!(
                        "Read {x}; increment its count. Counts cover exactly the consumed prefix."
                    ),
                );
            }
            let best = counts
                .iter()
                .max_by(|a, b| a.1.cmp(b.1).then_with(|| b.0.cmp(a.0)))
                .unwrap();
            e.answer = format!(
                "Counts {counts:?}; most frequent {} appears {} times.",
                best.0, best.1
            );
        }
        1 => {
            a.sort();
            let target = if case.is_multiple_of(3) {
                100
            } else {
                a[1] + a[a.len() - 2]
            };
            e.prompt = format!(
                "In sorted {a:?}, find two distinct positions whose values sum to {target}."
            );
            e.cost = "O(n) scan after sorting; O(1) search space. Sorting is O(n log n).";
            e.explanation = concat!(
                "The sum increases when the left pointer moves right and decreases ",
                "when the right pointer moves left. Therefore a sum below target ",
                "rules out the current left value with every remaining right ",
                "candidate; a sum above target rules out the current right value."
            )
            .into();
            let (mut l, mut r) = (0, a.len() - 1);
            e.answer = "No pair exists.".into();
            while l < r {
                let sum = a[l] + a[r];
                frame(
                    &mut e.frames,
                    strings(&a),
                    vec![l, r],
                    format!(
                        "left={l}, right={r}: {} + {} = {sum}; target={target}.",
                        a[l], a[r]
                    ),
                );
                if sum == target {
                    e.answer = format!("Positions {l} and {r}; values {} and {}.", a[l], a[r]);
                    break;
                }
                if sum < target {
                    l += 1;
                } else {
                    r -= 1;
                }
            }
        }
        2 => {
            let inputs = [
                "()",
                "([])",
                "([)]",
                "(()())",
                "{[()]}",
                "(()",
                "()[]{}",
                "{([])}([])",
                "]()",
                "(([]){})",
            ];
            let input = inputs[case];
            e.prompt = format!(
                "Are the brackets in {input} balanced? Predict the stack after each character."
            );
            e.cost = "O(n) time, O(n) worst-case space.";
            e.explanation = concat!(
                "Push opening brackets. A closing bracket must match the most ",
                "recent unmatched opening bracket. At the end the stack must be ",
                "empty. An unexpected closing bracket fails immediately."
            )
            .into();
            let mut stack = Vec::new();
            let mut valid = true;
            for c in input.chars() {
                if "([{".contains(c) {
                    stack.push(c);
                } else {
                    let want = match c {
                        ')' => '(',
                        ']' => '[',
                        _ => '{',
                    };
                    if stack.pop() != Some(want) {
                        valid = false;
                    }
                }
                frame(
                    &mut e.frames,
                    stack.iter().map(ToString::to_string).collect(),
                    vec![],
                    format!("Read {c}; top is the last cell. Matching so far: {valid}."),
                );
                if !valid {
                    break;
                }
            }
            e.answer = format!("Balanced: {}", valid && stack.is_empty());
        }
        3 => {
            a.sort();
            a.dedup();
            let target = if case.is_multiple_of(2) {
                a[a.len() / 2]
            } else {
                99
            };
            e.prompt =
                format!("Find {target} in {a:?}. Use a half-open search interval [left, right).");
            e.explanation = concat!(
                "Every discarded half cannot contain the target because the input ",
                "is ordered. Keep right exclusive: when the midpoint is too large, ",
                "set right=mid; when too small, set left=mid+1. The interval ",
                "strictly shrinks."
            )
            .into();
            e.cost = "O(log n) time, O(1) auxiliary space.";
            let (mut l, mut r) = (0, a.len());
            e.answer = "Target absent.".into();
            while l < r {
                let m = l + (r - l) / 2;
                frame(
                    &mut e.frames,
                    strings(&a),
                    vec![m],
                    format!("Search [{l}, {r}); midpoint {m} holds {}.", a[m]),
                );
                if a[m] == target {
                    e.answer = format!("Found at index {m}.");
                    break;
                }
                if a[m] < target {
                    l = m + 1;
                } else {
                    r = m;
                }
            }
        }
        4 => {
            let k = 2 + case % 4;
            e.prompt=format!("Find the largest sum of exactly {k} consecutive values in {a:?}. Include negative values correctly.");
            e.cost = "O(n) time, O(1) auxiliary space.";
            e.explanation = concat!(
                "The next window differs by two values: one leaves and one enters. ",
                "Subtract the leaving value and add the entering value. Initialize ",
                "the best sum from the first real window, not zero, to support ",
                "entirely negative data."
            )
            .into();
            let mut sum: i64 = a[..k].iter().sum();
            let mut best = sum;
            let mut start = 0;
            frame(
                &mut e.frames,
                strings(&a),
                (0..k).collect(),
                format!("Initial window sum={sum}; best={best}."),
            );
            for r in k..a.len() {
                sum += a[r] - a[r - k];
                if sum > best {
                    best = sum;
                    start = r + 1 - k;
                }
                frame(
                    &mut e.frames,
                    strings(&a),
                    (r + 1 - k..=r).collect(),
                    format!("Remove {}, add {}; sum={sum}, best={best}.", a[r - k], a[r]),
                );
            }
            e.answer = format!(
                "Maximum sum {best}, first attaining window at indices {start}..{}.",
                start + k - 1
            );
        }
        5 => {
            e.prompt=format!("Reverse the singly linked chain {a:?}. Track previous, current, and saved next at every rewiring.");
            e.cost = "O(n) time, O(1) working pointers for a linked list.";
            e.explanation = concat!(
                "Save current.next before changing it. Point current.next to ",
                "previous; then advance previous and current. The reversed prefix ",
                "and untouched suffix jointly contain every original node."
            )
            .into();
            for i in 0..=a.len() {
                let mut display = a[..i].to_vec();
                display.reverse();
                display.extend_from_slice(&a[i..]);
                frame(
                    &mut e.frames,
                    strings(&display),
                    if i < a.len() {
 vec![i] }
 else {
 vec![] }
,
                    format!(
                        "Reversed prefix length {i}; suffix length {}. previous={}, current={}, saved next={}. {}",
                        a.len() - i,
                        if i>0 {
 a[i-1].to_string() }
 else {
 "null".into() }
,
                        a.get(i).map(ToString::to_string).unwrap_or("null".into()),
                        a.get(i+1).map(ToString::to_string).unwrap_or("null".into()),
                        if i < a.len() {
                            "Save next, reverse one link, advance both pointers."
                        }
 else {
                            "Current is null; previous becomes the new head."
                        }
                    ),
                );
                let f = e.frames.last_mut().unwrap();
                f.edges = (1..i)
                    .map(|j| (j - 1, j))
                    .chain((i + 1..a.len()).map(|j| (j - 1, j)))
                    .collect();
            }
            a.reverse();
            e.answer = format!("Reversed values {a:?}.");
        }
        6 | 8 => {
            let kind = if topic == 6 { "bst" } else { "heap" };
            e.prompt = format!(
                "Insert {a:?} into a {}. Explain every resulting parent-child relationship. {}",
                if topic == 6 {
                    "binary search tree"
                } else {
                    "min heap"
                },
                if topic == 6 {
                    "Duplicate keys are ignored."
                } else {
                    "Duplicate values are retained."
                }
            );
            e.cost = if topic == 6 {
                "O(nh) build, h up to n; O(n) stored nodes."
            } else {
                "O(n log n) repeated insertions; O(n) storage."
            };
            e.explanation = if topic == 6 {
                concat!(
                    "Compare with the current node; descend left for a smaller key and ",
                    "right for a larger key. Inorder traversal is sorted. Ordered ",
                    "input can create a chain."
                )
            } else {
                concat!(
                    "Append at the next array slot, then sift toward the root while ",
                    "smaller than the parent. Completeness comes from array order; ",
                    "heap order comes from comparisons. The root is the minimum."
                )
            }
            .into();
            for i in 0..=a.len() {
                let values: Vec<_> = a[..i].iter().map(ToString::to_string).collect();
                let d = model::build(kind, &values);
                e.frames.push(from_diagram(d,
format!("After {i} complete insertions. Inspect the parent relationship; each insertion can contain several internal comparisons.")));
            }
            e.answer = if topic == 6 {
                let sorted: BTreeSet<_> = a.iter().collect();
                format!("Sorted unique inorder: {sorted:?}")
            } else {
                format!("Minimum: {}", a.iter().min().unwrap())
            };
        }
        7 => {
            let groups = [
                vec!["car", "cat", "cart"],
                vec!["a", "an", "ant"],
                vec!["to", "tea", "ten"],
                vec!["dog", "dot", "do"],
                vec!["app", "apple", "apt"],
                vec!["be", "bee", "beer"],
                vec!["cap", "cape", "cat"],
                vec!["in", "inn", "ink"],
                vec!["go", "gone", "good"],
                vec!["at", "atom", "atlas"],
            ];
            let words = &groups[case];
            e.prompt=format!("Build a trie for {words:?}. Mark complete words. How many distinct words and non-root prefix nodes exist?");
            e.cost = "O(total characters) insertion; storage proportional to distinct prefixes.";
            e.explanation = concat!(
                "Reuse a node whenever a prefix already exists. A terminal marker ",
                "records membership of the complete word, independently of whether ",
                "the node also has children. This is why a prefix can be a valid ",
                "word too."
            )
            .into();
            for i in 0..=words.len() {
                let v: Vec<_> = words[..i].iter().map(|s| s.to_string()).collect();
                e.frames.push(from_diagram(
                    model::build("trie", &v),
                    format!("Inserted {i} words. Highlighted cells are terminal nodes."),
                ));
            }
            let mut prefixes = BTreeSet::new();
            for w in words {
                for j in 1..=w.len() {
                    prefixes.insert(&w[..j]);
                }
            }
            e.answer = format!(
                "{} words, {} non-root prefix nodes.",
                words.len(),
                prefixes.len()
            );
        }
        9 => {
            a = (1..=3 + case % 3).map(|v| v as i64).collect();
            e.prompt=format!("Generate every subset of {a:?}, preserving input order. Explain how the recursion restores the selected path.");
            e.cost = "O(n × 2^n) output work; O(n) recursion and path, excluding output.";
            e.explanation = concat!(
                "At each position, choose either exclusion or inclusion. After an ",
                "inclusion branch returns, remove that value. Each of the 2^n ",
                "decision paths gives exactly one subset."
            )
            .into();
            let mut out = vec![];
            fn visit(
                a: &[i64],
                i: usize,
                path: &mut Vec<i64>,
                out: &mut Vec<Vec<i64>>,
                f: &mut Vec<Frame>,
            ) {
                frame(
                    f,
                    strings(path),
                    vec![],
                    format!("At decision {i}; current selected path {path:?}."),
                );
                if i == a.len() {
                    out.push(path.clone());
                    return;
                }
                visit(a, i + 1, path, out, f);
                path.push(a[i]);
                visit(a, i + 1, path, out, f);
                path.pop();
            }
            visit(&a, 0, &mut vec![], &mut out, &mut e.frames);
            e.answer = format!("{} subsets: {out:?}", out.len());
        }
        10 | 14 => {
            let n = 5 + case % 5;
            let mut edges: Vec<(usize, usize, i64)> = (0..n - 1)
                .map(|i| (i, i + 1, 1 + (i + case) % 4))
                .map(|(a, b, w)| (a, b, w as i64))
                .collect();
            edges.push((0, n - 1, if case.is_multiple_of(2) { 2 } else { 20 }));
            if case >= 4 {
                edges.push((1, n - 2, 1));
            }
            e.prompt = format!(
                "{} from vertex 0 in an undirected graph with edges {edges:?}. {}",
                if topic == 10 {
                    "Run BFS"
                } else {
                    "Run Dijkstra"
                },
                if topic == 10 {
                    "Ignore weights; find distances measured in edge count."
                } else {
                    "Find minimum total weight to every vertex."
                }
            );
            e.cost = if topic == 10 {
                "BFS O(V+E) time, O(V) auxiliary memory."
            } else {
                concat!(
                    "This small linear-minimum implementation is O(V²+E), O(V) memory; ",
                    "a heap gives O((V+E) log V)."
                )
            };
            e.explanation = if topic == 10 {
                concat!(
                    "FIFO traversal visits vertices in distance layers. Mark on ",
                    "discovery so a cycle cannot enqueue a vertex repeatedly."
                )
            } else {
                concat!(
                    "All weights are nonnegative. Choose the unsettled vertex of ",
                    "smallest distance and relax its edges. Once finalized, its ",
                    "distance cannot improve through another unsettled vertex."
                )
            }
            .into();
            let mut adjacency = vec![Vec::new(); n];
            for &(a, b, w) in &edges {
                adjacency[a].push((b, w));
                adjacency[b].push((a, w));
            }
            let mut dist = vec![i64::MAX; n];
            dist[0] = 0;
            let mut seen = vec![false; n];
            let mut q = VecDeque::from([0]);
            for _ in 0..n {
                let u = if topic == 10 {
                    if let Some(u) = q.pop_front() {
                        u
                    } else {
                        break;
                    }
                } else {
                    if let Some(u) = (0..n)
                        .filter(|i| !seen[*i] && dist[*i] != i64::MAX)
                        .min_by_key(|i| dist[*i])
                    {
                        u
                    } else {
                        break;
                    }
                };
                seen[u] = true;
                for &(v, w) in &adjacency[u] {
                    if topic == 10 {
                        if dist[v] == i64::MAX {
                            dist[v] = dist[u] + 1;
                            q.push_back(v);
                        }
                    } else if dist[u] + w < dist[v] {
                        dist[v] = dist[u] + w;
                    }
                }
                frame(
                    &mut e.frames,
                    dist.iter()
                        .enumerate()
                        .map(|(i, d)| {
                            format!(
                                "{i}: {}",
                                if *d == i64::MAX {
                                    "∞".into()
                                } else {
                                    d.to_string()
                                }
                            )
                        })
                        .collect(),
                    vec![u],
                    if topic == 10 {
                        format!("Processed vertex {u}. Pending BFS queue: {q:?}. Distances count edges.")
                    } else {
                        format!("Settled vertex {u}; relax its neighbors. Unsettled vertices: {:?}. Distances sum weights.",
 (0..n).filter(|i| !seen[*i]).collect::<Vec<_>>())
                    },
                );
                let f = e.frames.last_mut().unwrap();
                f.edges = edges.iter().map(|(a, b, _)| (*a, *b)).collect();
                f.diagram = Some(model::Diagram {
                    nodes: (0..n)
                        .map(|i| {
                            let angle = std::f64::consts::TAU * i as f64 / n as f64;
                            model::Node {
                                label: format!("vertex {i}"),
                                note: format!(
                                    "distance {}",
                                    if dist[i] == i64::MAX {
                                        "∞".into()
                                    } else {
                                        dist[i].to_string()
                                    }
                                ),
                                x: 410.0 + 300.0 * angle.cos(),
                                y: 280.0 + 210.0 * angle.sin(),
                                active: i == u,
                            }
                        })
                        .collect(),
                    edges: edges
                        .iter()
                        .map(|(a, b, _)| model::Edge {
                            from: *a,
                            to: *b,
                            directed: false,
                            both: false,
                        })
                        .collect(),
                    rows: vec![],
                    width: 820.0,
                    height: 560.0,
                });
            }
            e.answer = format!("Distances from 0: {dist:?}.");
        }
        11 => {
            let n = 3 + case;
            e.prompt=format!("A staircase has {n} steps. You may climb 1 or 2 at a time. Count distinct ordered ways to reach the top.");
            e.cost = "O(n) time; table O(n), reducible to O(1) for the final count.";
            e.explanation = concat!(
                "The last move came from step i-1 or i-2, and these cases do not ",
                "overlap. Thus ways[i]=ways[i-1]+ways[i-2]. There is one way to ",
                "climb zero steps: the empty sequence."
            )
            .into();
            let mut dp = vec![0; n + 1];
            dp[0] = 1;
            for i in 1..=n {
                dp[i] = dp[i - 1] + if i > 1 { dp[i - 2] } else { 0 };
                frame(
                    &mut e.frames,
                    strings(&dp),
                    vec![i],
                    format!(
                        "ways[{i}] = ways[{}] + {} = {}",
                        i - 1,
                        if i > 1 {
                            format!("ways[{}]", i - 2)
                        } else {
                            "0".into()
                        },
                        dp[i]
                    ),
                );
            }
            e.answer = format!("{} ordered ways.", dp[n]);
        }
        12 | 13 => {
            let mut intervals = vec![
                (1, 3),
                (2, 4),
                (4, 7),
                (6, 8),
                (8, 10),
                (0, 2 + case as i64),
            ];
            e.prompt = format!(
                "{} intervals {intervals:?}. {}",
                if topic == 12 {
                    "Merge overlapping"
                } else {
                    "Select the maximum number of nonoverlapping"
                },
                if topic == 12 {
                    "Treat closed intervals that touch as overlapping."
                } else {
                    "Use half-open intervals; an end equal to the next start is compatible."
                }
            );
            e.cost = "O(n log n) sorting, O(n) scan, O(n) output.";
            e.explanation = if topic == 12 {
                concat!(
                    "Sort by start. If the next interval starts before or at the ",
                    "current end, extend the end to the maximum. Otherwise the current ",
                    "interval is complete. Sorting ensures no later interval can ",
                    "bridge the gap."
                )
            } else {
                concat!(
                    "Sort by finishing time. Keep the first compatible interval. An ",
                    "earliest-finishing choice leaves at least as much room for the ",
                    "remaining choices as any alternative first choice: the exchange ",
                    "argument."
                )
            }
            .into();
            let mut out: Vec<(i64, i64)> = vec![];
            if topic == 12 {
                intervals.sort();
            } else {
                intervals.sort_by_key(|x| x.1);
            }
            for (s, t) in intervals {
                if topic == 12 {
                    if let Some(last) = out.last_mut() {
                        if s <= last.1 {
                            last.1 = last.1.max(t);
                        } else {
                            out.push((s, t));
                        }
                    } else {
                        out.push((s, t));
                    }
                } else if out.last().is_none_or(|last| s >= last.1) {
                    out.push((s, t));
                }
                frame(
                    &mut e.frames,
                    out.iter()
                        .map(|(a, b)| format!("[{a},{b}{}", if topic == 12 { "]" } else { ")" }))
                        .collect(),
                    vec![],
                    format!("Considered [{s},{t}]; accepted/merged spans shown."),
                );
            }
            e.answer = format!("{} resulting intervals: {out:?}", out.len());
        }
        15 => {
            let rows = 3 + case % 3;
            let cols = 3 + case % 4;
            e.prompt=format!("Count paths in a {rows} by {cols} grid from top-left to bottom-right, moving only right or down. The cell (1,1) is blocked.");
            e.cost = "O(rows × columns) time and table space; can compress to one row.";
            e.explanation = concat!(
                "Each unblocked cell can be entered from above or from the left, ",
                "so add those two counts. The start has one path. Blocked cells ",
                "have zero paths; do not propagate through them."
            )
            .into();
            let mut dp = vec![0; rows * cols];
            dp[0] = 1;
            for r in 0..rows {
                for c in 0..cols {
                    let i = r * cols + c;
                    if r == 1 && c == 1 {
                        dp[i] = 0;
                    } else if i > 0 {
                        dp[i] = if r > 0 { dp[i - cols] } else { 0 }
                            + if c > 0 { dp[i - 1] } else { 0 };
                    }
                    frame(&mut e.frames,
dp.iter().enumerate().map(|(j,
v)|if j==cols+1{
"BLOCK".into()}
else{
v.to_string()}
).collect(),
vec![i],
format!("Cell ({r},{c}); ways={}. Read top and left after their values are known.",
dp[i]));
                    e.frames.last_mut().unwrap().columns = cols;
                }
            }
            e.answer = format!("{} paths.", dp[rows * cols - 1]);
        }
        16 => {
            let n = 13 + case as u32 * 7;
            e.prompt=format!("Count set bits in {n} (binary {n:08b}) using n & (n-1). Why does each operation remove exactly one set bit?");
            e.cost = "O(number of set bits) time, O(1) auxiliary memory.";
            e.explanation = concat!(
                "Subtracting one flips the lowest set bit to zero and changes the ",
                "trailing zeroes to ones. AND with the original clears that lowest ",
                "set bit and leaves all higher bits unchanged."
            )
            .into();
            let mut x = n;
            let mut count = 0;
            loop {
                frame(
                    &mut e.frames,
                    format!("{x:08b}").chars().map(|c| c.to_string()).collect(),
                    vec![],
                    format!("Remaining value={x}, cleared set bits={count}."),
                );
                if x == 0 {
                    break;
                }
                x &= x - 1;
                count += 1;
            }
            e.answer = format!("{count} set bits.");
        }
        _ => {
            let (mut a, mut b) = (48 + case as i64 * 12, 18 + case as i64 * 6);
            e.prompt=format!("Find gcd({a},{b}) with Euclid's algorithm. Explain why each remainder step preserves the common divisors.");
            e.cost = "O(log min(a,b)) remainder steps; O(1) auxiliary memory.";
            e.explanation = concat!(
                "Any divisor common to a and b also divides a-qb, the remainder. ",
                "Conversely, a common divisor of b and the remainder divides a. ",
                "Replace the pair until the second value is zero."
            )
            .into();
            while b != 0 {
                let r = a % b;
                frame(
                    &mut e.frames,
                    vec![a.to_string(), b.to_string(), r.to_string()],
                    vec![2],
                    format!("{a} mod {b} = {r}; replace (a,b) by ({b},{r})."),
                );
                a = b;
                b = r;
            }
            e.answer = format!("Greatest common divisor: {a}.");
        }
    }
    if case >= 7 {
        e.prompt.push_str(concat!(
            " Also justify correctness, name a tempting incorrect approach, ",
            "and derive time and space costs."
        ));
    }
    if e.frames.is_empty() {
        frame(
            &mut e.frames,
            vec![],
            vec![],
            "Empty input: no operations required.",
        );
    }
    e
}
fn from_diagram(d: model::Diagram, note: String) -> Frame {
    Frame {
        cells: d
            .nodes
            .iter()
            .map(|n| format!("{} {}", n.label, n.note))
            .collect(),
        active: d
            .nodes
            .iter()
            .enumerate()
            .filter_map(|(i, n)| n.active.then_some(i))
            .collect(),
        note,
        columns: 5,
        edges: d.edges.iter().map(|e| (e.from, e.to)).collect(),
        diagram: Some(d),
    }
}
pub fn total_questions() -> usize {
    LESSONS.len() * 10
}
/// Ten operation-sequence questions for each concrete structure, in addition to the roadmap bank.
pub fn structure_exercise(topic: usize, case: usize) -> Exercise {
    let t = &crate::catalog::TOPICS[topic];
    let case = case.min(9);
    let original = model::tokens(t.example);
    let count = (2 + case).min(original.len());
    let mut current = original[..count].to_vec();
    let difficulty = if case < 3 {
        "Easy"
    } else if case < 7 {
        "Medium"
    } else {
        "Hard"
    };
    let mut operations: Vec<(String, String, usize)> = Vec::new();
    let value = if model::numeric(t.id) {
        (case as i64 * 2 - 3).to_string()
    } else if t.id == "trie" {
        "care".into()
    } else if t.id == "hashmap" {
        format!("cat:{}", case + 1)
    } else if matches!(t.id, "graph" | "dsu") {
        "E-G".into()
    } else {
        "eel".into()
    };
    operations.push(("insert".into(), value.clone(), case % (current.len() + 1)));
    if case >= 3 {
        match t.id {
            "bst" => operations.push(("insert".into(), value.clone(), 0)),
            "graph" | "dsu" => operations.push(("insert".into(), "G-H".into(), 0)),
            "fenwick" | "segment" => operations.push(("set".into(), (case + 9).to_string(), 1)),
            "hashmap" | "hashset" | "trie" => {
                operations.push(("remove".into(), value.split(':').next().unwrap().into(), 0))
            }
            _ => operations.push(("remove".into(), String::new(), 1)),
        }
    }
    if case >= 7 {
        operations.push(("insert".into(), original[0].clone(), 0));
        if t.id == "deque" {
            operations.push(("popfront".into(), String::new(), 0));
        }
    }
    let prompt=format!("Start with {} built from {:?}. Apply {:?} in order; each tuple is (operation, value/key, index). Predict \
                        the final contents and explain the invariant after every operation. {}",
t.title,
current,
operations,
if case>=7{
concat!(
"Explain duplicate handling and distinguish logical operation cost ",
"from finding a position."
)}
else{
"Indices are zero-based; end-based structures ignore the index."}
);
    let mut frames = vec![from_diagram(
        model::build(t.id, &current),
        "Initial structure before any operation.".into(),
    )];
    for (op, value, index) in operations {
        match model::mutate(t.id, &current, &op, &value, index) {
            Ok((next, msg)) => {
                current = next;
                frames.push(from_diagram(
                    model::build(t.id, &current),
                    format!("{msg} Rule: {}", t.invariant),
                ));
            }
            Err(error) => {
                frames.push(from_diagram(
                    model::build(t.id, &current),
                    format!("Operation rejected without changing state: {error}"),
                ));
            }
        }
    }
    let final_d = model::build(t.id, &current);
    let answer = format!(
        "Final logical representation: {}. {}",
        final_d
            .nodes
            .iter()
            .filter(|n| n.note != "bucket")
            .map(|n| format!("{} ({})", n.label, n.note))
            .collect::<Vec<_>>()
            .join(", "),
        t.invariant
    );
    Exercise{
title:format!("{} · {} operation sequence",
case+1,
t.title),
difficulty,
prompt,
explanation:format!("{} {} Read the initial snapshot, apply one operation at a time, and check the rule before continuing. \
                        Diagrams describe logical relationships; a position is not a machine memory address.",
t.about,
t.invariant),
cost:t.cost,
frames,
answer}
}
