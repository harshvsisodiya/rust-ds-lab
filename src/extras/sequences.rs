use super::*;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
pub(super) fn lru(input: Vec<i64>, e: &mut Exercise) {
    let mut recent = Vec::new();
    let mut hits = 0;
    cells(
        e,
        vec![],
        vec![],
        "Capacity is three. The left end will be most recent; the right end least recent.".into(),
    );
    for key in input {
        let note = if let Some(index) = recent.iter().position(|v| *v == key) {
            recent.remove(index);
            hits += 1;
            format!("Hit {key}: move it to the most-recent end.")
        } else if recent.len() == 3 {
            let evicted = recent.pop().expect("cache at capacity");
            format!("Miss {key}: evict least-recent key {evicted} and insert the new key.")
        } else {
            format!("Miss {key}: space is available, so insert the new key.")
        };
        recent.insert(0, key);
        let d = model::Diagram {
            nodes: recent
                .iter()
                .enumerate()
                .map(|(i, k)| model::Node {
                    label: k.to_string(),
                    note: if i == 0 {
                        "most recent".into()
                    } else if i + 1 == recent.len() {
                        "least recent".into()
                    } else {
                        "cached".into()
                    },
                    x: 100.0 + i as f64 * 155.0,
                    y: 80.0,
                    active: i == 0,
                })
                .collect(),
            edges: vec![],
            rows: vec![],
            width: 560.0,
            height: 200.0,
        };
        diagram(e, d, note);
    }
    e.answer = format!("Most-to-least recent: {recent:?}; cache hits: {hits}");
}
pub(super) fn monotonic(a: Vec<i64>, e: &mut Exercise) {
    let mut stack: Vec<usize> = vec![];
    let mut answers = vec![None; a.len()];
    for i in 0..a.len() {
        while stack.last().is_some_and(|&j| a[j] < a[i]) {
            let j = stack.pop().expect("nonempty stack checked");
            answers[j] = Some(a[i]);
            cells(e,
a.iter().enumerate().map(|(k,
n)|format!("{n} → {}",
answers[k].map(|n|n.to_string()).unwrap_or_else(||"?".into()))).collect(),
vec![j,
i],
format!("{} is the first greater value for index {j}. Pop {j}; unresolved stack: {stack:?}.",
a[i]));
        }
        stack.push(i);
        cells(
            e,
            a.iter().map(ToString::to_string).collect(),
            vec![i],
            format!("Push index {i}. Stack indices {stack:?} have non-increasing values."),
        );
    }
    let answer: Vec<_> = answers
        .iter()
        .map(|v| v.map(|n| n.to_string()).unwrap_or_else(|| "none".into()))
        .collect();
    cells(
        e,
        answer.clone(),
        vec![],
        "Unresolved positions have no greater value to their right.".into(),
    );
    e.answer = format!("Next greater values: {}", answer.join(", "));
}
fn directed_frame(
    names: &[String],
    graph: &BTreeMap<String, BTreeSet<String>>,
    degree: &BTreeMap<String, usize>,
    done: &BTreeSet<String>,
    active: Option<&str>,
) -> model::Diagram {
    let mut d = model::Diagram::default();
    let radius = (names.len() as f64 * 22.0).max(150.0);
    for (i, name) in names.iter().enumerate() {
        let angle = i as f64 / names.len() as f64 * std::f64::consts::TAU;
        d.nodes.push(model::Node {
            label: name.clone(),
            note: if done.contains(name) {
                "processed".into()
            } else {
                format!("in-degree {}", degree[name])
            },
            x: radius + 80.0 + radius * angle.cos(),
            y: radius + 65.0 + radius * angle.sin(),
            active: active == Some(name.as_str()),
        });
    }
    for (i, name) in names.iter().enumerate() {
        for to in &graph[name] {
            if let Some(j) = names.iter().position(|n| n == to) {
                d.edges.push(model::Edge {
                    from: i,
                    to: j,
                    directed: true,
                    both: false,
                });
            }
        }
    }
    d.width = radius * 2.0 + 160.0;
    d.height = radius * 2.0 + 130.0;
    d
}
pub(super) fn topological(input: &str, e: &mut Exercise) -> Result<(), String> {
    let items = model::tokens(input);
    model::validate("graph", &items)?;
    if items.is_empty() {
        return Err("Enter directed edges such as A-B,B-C.".into());
    }
    let mut graph: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for pair in items {
        let (a, b) = pair.split_once('-').ok_or("Use source-target edges")?;
        graph.entry(a.into()).or_default().insert(b.into());
        graph.entry(b.into()).or_default();
    }
    let names: Vec<_> = graph.keys().cloned().collect();
    let mut degree: BTreeMap<String, usize> = names.iter().map(|n| (n.clone(), 0)).collect();
    for neighbors in graph.values() {
        for n in neighbors {
            *degree.get_mut(n).expect("vertex collected") += 1;
        }
    }
    let mut queue: VecDeque<_> = degree
        .iter()
        .filter(|(_, v)| **v == 0)
        .map(|(k, _)| k.clone())
        .collect();
    let mut done = BTreeSet::new();
    let mut order = vec![];
    diagram(
        e,
        directed_frame(&names, &graph, &degree, &done, None),
        format!("Initial ready queue: {queue:?}. Arrows point from prerequisites to dependents."),
    );
    while let Some(name) = queue.pop_front() {
        done.insert(name.clone());
        order.push(name.clone());
        for to in &graph[&name] {
            let count = degree.get_mut(to).expect("vertex collected");
            *count -= 1;
            if *count == 0 {
                queue.push_back(to.clone());
            }
        }
        diagram(
            e,
            directed_frame(&names, &graph, &degree, &done, Some(&name)),
            format!(
                "Process {name}, decrease successor indegrees. Ready: {queue:?}; order: {order:?}."
            ),
        );
    }
    e.answer = if order.len() == names.len() {
        format!("Topological order: {}", order.join(" → "))
    } else {
        "Cycle detected: no topological order exists.".into()
    };
    diagram(
        e,
        directed_frame(&names, &graph, &degree, &done, None),
        e.answer.clone(),
    );
    Ok(())
}
fn string_frame(e: &mut Exercise, text: &[u8], pattern: &[u8], i: usize, j: usize, note: String) {
    let mut nodes = Vec::new();
    for (k, c) in text.iter().enumerate() {
        nodes.push(model::Node {
            label: char::from(*c).to_string(),
            note: format!("text {k}"),
            x: 45.0 + (k % 12) as f64 * 65.0,
            y: 50.0 + (k / 12) as f64 * 85.0,
            active: k == i,
        });
    }
    let offset = text.len().div_ceil(12).max(1) as f64 * 85.0 + 50.0;
    for (k, c) in pattern.iter().enumerate() {
        nodes.push(model::Node {
            label: char::from(*c).to_string(),
            note: format!("pattern {k}"),
            x: 45.0 + (k % 12) as f64 * 65.0,
            y: offset + (k / 12) as f64 * 85.0,
            active: k == j,
        });
    }
    diagram(
        e,
        model::Diagram {
            nodes,
            edges: vec![],
            rows: vec![],
            width: 830.0,
            height: offset + pattern.len().div_ceil(12) as f64 * 85.0 + 40.0,
        },
        note,
    );
}
pub(super) fn kmp(input: &str, e: &mut Exercise) -> Result<(), String> {
    let (text, pattern) = input
        .split_once('|')
        .ok_or("Use text|pattern, for example ababab|aba.")?;
    if !text.is_ascii()
        || !pattern.is_ascii()
        || text.len() > 48
        || pattern.is_empty()
        || pattern.len() > 16
        || pattern.contains('|')
        || input.chars().any(char::is_control)
    {
        return Err(
            "Use visible ASCII text (up to 48 characters) and a nonempty pattern (up \
                        to 16), separated by |."
                .into(),
        );
    }
    let t = text.as_bytes();
    let p = pattern.as_bytes();
    let mut lps = vec![0; p.len()];
    let mut length = 0;
    string_frame(
        e,
        t,
        p,
        usize::MAX,
        0,
        "First build the pattern's longest-prefix/suffix (LPS) table.".into(),
    );
    for i in 1..p.len() {
        while length > 0 && p[i] != p[length] {
            length = lps[length - 1];
        }
        if p[i] == p[length] {
            length += 1;
        }
        lps[i] = length;
        string_frame(
            e,
            t,
            p,
            usize::MAX,
            i,
            format!("LPS[{i}] = {length}. Proper prefix/suffix lengths: {lps:?}."),
        );
    }
    let (mut i, mut j) = (0, 0);
    let mut matches = vec![];
    while i < t.len() {
        string_frame(
            e,
            t,
            p,
            i,
            j,
            format!(
                "Compare text[{i}]={} with pattern[{j}]={}. LPS={lps:?}.",
                char::from(t[i]),
                char::from(p[j])
            ),
        );
        if t[i] == p[j] {
            i += 1;
            j += 1;
            if j == p.len() {
                matches.push(i - j);
                string_frame(
                    e,
                    t,
                    p,
                    i - 1,
                    j - 1,
                    format!(
                        "Match starts at {}. Reuse LPS to allow overlapping matches.",
                        i - j
                    ),
                );
                j = lps[j - 1];
            }
        } else if j > 0 {
            let before = j;
            j = lps[j - 1];
            string_frame(
                e,
                t,
                p,
                i,
                j,
                format!("Mismatch: move pattern index from {before} to {j}; keep text index {i}."),
            );
        } else {
            i += 1;
        }
    }
    e.answer = format!("Match start indices: {matches:?}");
    string_frame(e, t, p, usize::MAX, usize::MAX, e.answer.clone());
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cache_eviction_and_next_greater_match_known_answers() {
        assert_eq!(
            trace(9, "1,2,3,1,4,2,5,1", 0).unwrap().answer,
            "Most-to-least recent: [1, 5, 2]; cache hits: 1"
        );
        assert_eq!(
            trace(10, "2,1,5,3,4,7", 0).unwrap().answer,
            "Next greater values: 5, 5, 7, 4, 7, none"
        );
    }
    #[test]
    fn directed_cycles_are_reported_and_order_respects_every_edge() {
        assert!(trace(11, "A-B,B-C,C-A", 0)
            .unwrap()
            .answer
            .contains("Cycle detected"));
        let e = trace(11, "A-C,B-C,C-D,B-E", 0).unwrap();
        let list: Vec<_> = e
            .answer
            .strip_prefix("Topological order: ")
            .unwrap()
            .split(" → ")
            .collect();
        for (a, b) in [("A", "C"), ("B", "C"), ("C", "D"), ("B", "E")] {
            assert!(list.iter().position(|v| *v == a) < list.iter().position(|v| *v == b));
        }
    }
    #[test]
    fn kmp_matches_a_naive_search_for_overlaps_and_absence() {
        for case in 0..10 {
            let input = example(12, case);
            let (t, p) = input.split_once('|').unwrap();
            let expected: Vec<_> = t
                .as_bytes()
                .windows(p.len())
                .enumerate()
                .filter(|(_, w)| *w == p.as_bytes())
                .map(|(i, _)| i)
                .collect();
            assert_eq!(
                trace(12, &input, case).unwrap().answer,
                format!("Match start indices: {expected:?}")
            );
        }
        for bad in ["missing delimiter", "text|", "abc|λ", "abc|a|b"] {
            assert!(trace(12, bad, 0).is_err());
        }
    }
}
