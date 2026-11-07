//! Deterministic teaching models. Diagram node IDs are not memory addresses.
use std::collections::{BTreeMap, BTreeSet};
pub const MAX_ITEMS: usize = 16;
pub fn tokens(data: &str) -> Vec<String> {
    data.split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .collect()
}
pub fn numeric(kind: &str) -> bool {
    matches!(
        kind,
        "array"
            | "stack"
            | "queue"
            | "deque"
            | "singly"
            | "doubly"
            | "circular"
            | "bst"
            | "heap"
            | "fenwick"
            | "segment"
    )
}
pub fn number(s: &str) -> Result<i64, String> {
    s.parse::<i64>()
        .ok()
        .filter(|n| (-9999..=9999).contains(n))
        .ok_or_else(|| "Use integers between -9999 and 9999.".to_string())
}
fn key(s: &str) -> bool {
    !s.is_empty() && s.len() <= 12 && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}
pub fn validate(kind: &str, items: &[String]) -> Result<(), String> {
    if !numeric(kind) && !matches!(kind, "hashmap" | "hashset" | "trie" | "graph" | "dsu") {
        return Err("Unknown data structure.".into());
    }
    if items.len() > MAX_ITEMS {
        return Err("Keep the diagram to 16 input items or fewer.".into());
    }
    for value in items {
        if numeric(kind) {
            number(value)?;
        } else if kind == "trie" {
            if value.len() > 8 || value.is_empty() || !value.chars().all(|c| c.is_ascii_lowercase())
            {
                return Err("Trie words must contain 1–8 lowercase ASCII letters.".into());
            }
        } else if kind == "hashmap" {
            let (k, v) = value
                .split_once(':')
                .ok_or("Use key:value entries, for example cat:7.")?;
            if !key(k) {
                return Err("Keys must be 1–12 ASCII letters, digits, or underscores.".into());
            }
            number(v)?;
        } else if kind == "graph" || kind == "dsu" {
            let (a, b) = value
                .split_once('-')
                .ok_or("Use A-B pairs separated by commas.")?;
            if !key(a) || !key(b) {
                return Err(
                    "Vertex labels must be 1–12 ASCII letters, digits, or underscores.".into(),
                );
            }
            if a == b {
                return Err("Use two distinct vertices in each pair.".into());
            }
        } else if !key(value) {
            return Err("Keys must be 1–12 ASCII letters, digits, or underscores.".into());
        }
    }
    Ok(())
}
#[derive(Debug, Clone)]
pub struct Node {
    pub label: String,
    pub note: String,
    pub x: f64,
    pub y: f64,
    pub active: bool,
}
#[derive(Debug, Clone)]
pub struct Edge {
    pub from: usize,
    pub to: usize,
    pub directed: bool,
    pub both: bool,
}
#[derive(Debug, Default, Clone)]
pub struct Diagram {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub rows: Vec<(String, String)>,
    pub width: f64,
    pub height: f64,
}
impl Diagram {
    fn new() -> Self {
        Self {
            width: 1000.0,
            height: 240.0,
            ..Self::default()
        }
    }
    fn node(&mut self, label: String, note: String, x: f64, y: f64, active: bool) -> usize {
        let id = self.nodes.len();
        self.nodes.push(Node {
            label,
            note,
            x,
            y,
            active,
        });
        self.height = self.height.max(y + 70.0);
        id
    }
    fn edge(&mut self, from: usize, to: usize, directed: bool, both: bool) {
        self.edges.push(Edge {
            from,
            to,
            directed,
            both,
        });
    }
}
#[derive(Debug)]
pub struct Bst {
    pub value: i64,
    pub left: Option<Box<Bst>>,
    pub right: Option<Box<Bst>>,
}
impl Bst {
    pub fn insert(root: &mut Option<Box<Self>>, value: i64) {
        match root {
            None => {
                *root = Some(Box::new(Self {
                    value,
                    left: None,
                    right: None,
                }))
            }
            Some(node) if value < node.value => Self::insert(&mut node.left, value),
            Some(node) if value > node.value => Self::insert(&mut node.right, value),
            _ => (),
        }
    }
    pub fn contains(root: &Option<Box<Self>>, value: i64) -> bool {
        match root {
            None => false,
            Some(node) if value == node.value => true,
            Some(node) if value < node.value => Self::contains(&node.left, value),
            Some(node) => Self::contains(&node.right, value),
        }
    }
    fn draw(
        &self,
        diagram: &mut Diagram,
        depth: usize,
        rank: &mut usize,
        total: usize,
        last: i64,
    ) -> usize {
        let left = self
            .left
            .as_ref()
            .map(|node| node.draw(diagram, depth + 1, rank, total, last));
        let x = 70.0 + *rank as f64 * (diagram.width - 140.0) / total.max(1) as f64;
        *rank += 1;
        let id = diagram.node(
            self.value.to_string(),
            format!("depth {depth}"),
            x,
            60.0 + depth as f64 * 100.0,
            self.value == last,
        );
        let right = self
            .right
            .as_ref()
            .map(|node| node.draw(diagram, depth + 1, rank, total, last));
        for child in [left, right].iter().flatten() {
            diagram.edge(id, *child, true, false);
        }
        id
    }
}
pub fn heap_push(heap: &mut Vec<i64>, value: i64) {
    heap.push(value);
    let mut i = heap.len() - 1;
    while i > 0 {
        let parent = (i - 1) / 2;
        if heap[parent] <= heap[i] {
            break;
        }
        heap.swap(parent, i);
        i = parent;
    }
}
pub fn heap_pop(heap: &mut Vec<i64>) -> Option<i64> {
    if heap.is_empty() {
        return None;
    }
    let result = heap.swap_remove(0);
    let mut i = 0;
    while i * 2 + 1 < heap.len() {
        let mut child = i * 2 + 1;
        if child + 1 < heap.len() && heap[child + 1] < heap[child] {
            child += 1;
        }
        if heap[i] <= heap[child] {
            break;
        }
        heap.swap(i, child);
        i = child;
    }
    Some(result)
}
pub fn fenwick(values: &[i64]) -> Vec<i64> {
    let mut bit = vec![0; values.len() + 1];
    for (i, value) in values.iter().enumerate() {
        let mut j = i + 1;
        while j < bit.len() {
            bit[j] += value;
            j += j & j.wrapping_neg();
        }
    }
    bit
}
pub fn prefix(bit: &[i64], mut i: usize) -> i64 {
    let mut sum = 0;
    while i > 0 {
        sum += bit[i];
        i &= i - 1;
    }
    sum
}
#[derive(Default, Debug)]
pub struct Dsu {
    pub parents: BTreeMap<String, String>,
    sizes: BTreeMap<String, usize>,
}
impl Dsu {
    pub fn find(&mut self, x: &str) -> String {
        if !self.parents.contains_key(x) {
            self.parents.insert(x.into(), x.into());
            self.sizes.insert(x.into(), 1);
        }
        let parent = self.parents[x].clone();
        if parent == x {
            return parent;
        }
        let root = self.find(&parent);
        self.parents.insert(x.into(), root.clone());
        root
    }
    pub fn union(&mut self, a: &str, b: &str) {
        let (mut a, mut b) = (self.find(a), self.find(b));
        if a == b {
            return;
        }
        if self.sizes[&a] < self.sizes[&b] {
            std::mem::swap(&mut a, &mut b);
        }
        let size = self.sizes[&a] + self.sizes[&b];
        self.parents.insert(b, a.clone());
        self.sizes.insert(a, size);
    }
}
#[derive(Default)]
struct Trie {
    terminal: bool,
    children: BTreeMap<char, Trie>,
}
impl Trie {
    fn insert(&mut self, word: &str) {
        let mut node = self;
        for c in word.chars() {
            node = node.children.entry(c).or_default();
        }
        node.terminal = true;
    }
    fn draw(
        &self,
        d: &mut Diagram,
        label: String,
        depth: usize,
        rows: &mut Vec<Vec<usize>>,
    ) -> usize {
        let id = d.node(
            label,
            if self.terminal {
                "word end".into()
            } else {
                "".into()
            },
            0.0,
            60.0 + depth as f64 * 100.0,
            self.terminal,
        );
        if rows.len() <= depth {
            rows.push(Vec::new());
        }
        rows[depth].push(id);
        for (c, child) in &self.children {
            let child_id = child.draw(d, c.to_string(), depth + 1, rows);
            d.edge(id, child_id, true, false);
        }
        id
    }
}
/// Small deterministic teaching hash, not Python's or Rust's production hash.
pub fn bucket(key: &str) -> usize {
    key.bytes()
        .fold(0usize, |hash, b| (hash * 31 + b as usize) % 8)
}
fn tree_position(i: usize, canvas_width: f64) -> (f64, f64) {
    let depth = (usize::BITS - (i + 1).leading_zeros() - 1) as usize;
    let first = (1usize << depth) - 1;
    let width = 1usize << depth;
    (
        canvas_width * (i - first + 1) as f64 / (width + 1) as f64,
        60.0 + depth as f64 * 100.0,
    )
}
fn segment_draw(values: &[i64], lo: usize, hi: usize, depth: usize, d: &mut Diagram) -> usize {
    let x = 60.0 + (d.width - 120.0) * (lo + hi) as f64 / (2.0 * values.len() as f64);
    let id = d.node(
        values[lo..hi].iter().sum::<i64>().to_string(),
        format!("[{lo}, {})", hi),
        x,
        60.0 + depth as f64 * 100.0,
        hi - lo == 1,
    );
    if hi - lo > 1 {
        let mid = (lo + hi) / 2;
        let l = segment_draw(values, lo, mid, depth + 1, d);
        let r = segment_draw(values, mid, hi, depth + 1, d);
        d.edge(id, l, false, false);
        d.edge(id, r, false, false);
    }
    id
}
/// Build a bounded teaching diagram. Invalid input yields an empty diagram
/// with an `Input error` row; callers that need a Result should use try_build.
pub fn build(kind: &str, items: &[String]) -> Diagram {
    try_build(kind, items).unwrap_or_else(|error| {
        let mut diagram = Diagram::new();
        diagram.rows.push(("Input error".into(), error));
        diagram
    })
}
/// Validate at the public API boundary, even when the UI is bypassed.
pub fn try_build(kind: &str, items: &[String]) -> Result<Diagram, String> {
    validate(kind, items)?;
    Ok(build_validated(kind, items))
}
fn build_validated(kind: &str, items: &[String]) -> Diagram {
    let mut d = Diagram::new();
    let nums: Vec<i64> = items.iter().filter_map(|s| s.parse().ok()).collect();
    match kind {
        "bst" => {
            let mut root = None;
            for n in &nums {
                Bst::insert(&mut root, *n);
            }
            let count = nums.iter().collect::<BTreeSet<_>>().len();
            d.width = (count as f64 * 140.0).max(1000.0);
            if let Some(root) = root {
                if let Some(last) = nums.last() {
                    root.draw(&mut d, 0, &mut 0, count, *last);
                }
            }
        }
        "heap" => {
            let mut heap = Vec::new();
            for n in nums {
                heap_push(&mut heap, n);
            }
            d.width = ((heap.len() + 1).next_power_of_two() as f64 * 125.0).max(1000.0);
            for (i, value) in heap.iter().enumerate() {
                let (x, y) = tree_position(i, d.width);
                d.node(value.to_string(), format!("index {i}"), x, y, i == 0);
                if i > 0 {
                    d.edge((i - 1) / 2, i, false, false);
                }
            }
            d.rows.push(("Backing array".into(), format!("{heap:?}")));
        }
        "trie" => {
            let mut root = Trie::default();
            for word in items {
                root.insert(word);
            }
            let mut rows = Vec::new();
            root.draw(&mut d, "root".into(), 0, &mut rows);
            d.width = (rows.iter().map(Vec::len).max().unwrap_or(1) as f64 * 150.0).max(1000.0);
            for row in rows {
                for (i, id) in row.iter().enumerate() {
                    d.nodes[*id].x = d.width * (i + 1) as f64 / (row.len() + 1) as f64;
                }
            }
        }
        "hashmap" | "hashset" => {
            let mut map = BTreeMap::new();
            for item in items {
                if kind == "hashmap" {
                    let Some((k, v)) = item.split_once(':') else {
                        continue;
                    };
                    map.insert(k.to_string(), v.to_string());
                } else {
                    map.insert(item.clone(), String::new());
                }
            }
            for b in 0..8 {
                let root = d.node(
                    b.to_string(),
                    "bucket".into(),
                    60.0,
                    60.0 + b as f64 * 90.0,
                    false,
                );
                let mut previous = root;
                let mut col = 0;
                for (k, v) in map.iter().filter(|(k, _)| bucket(k) == b) {
                    col += 1;
                    let label = if kind == "hashmap" {
                        format!("{k}:{v}")
                    } else {
                        k.clone()
                    };
                    let id = d.node(
                        label,
                        "entry".into(),
                        60.0 + col as f64 * 160.0,
                        60.0 + b as f64 * 90.0,
                        true,
                    );
                    d.edge(previous, id, true, false);
                    previous = id;
                    d.width = d.width.max(160.0 * (col + 1) as f64);
                }
            }
            d.rows.push((
                "Teaching hash".into(),
                "h = (h × 31 + byte) mod 8, starting at 0; separate chaining".into(),
            ));
            d.rows.push(("Unique keys".into(), map.len().to_string()));
        }
        "graph" => {
            let mut graph: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
            for pair in items {
                let Some((a, b)) = pair.split_once('-') else {
                    continue;
                };
                graph.entry(a.into()).or_default().insert(b.into());
                graph.entry(b.into()).or_default().insert(a.into());
            }
            let names: Vec<_> = graph.keys().cloned().collect();
            let radius = (names.len() as f64 * 30.0).max(160.0);
            d.width = (radius * 2.0 + 180.0).max(1000.0);
            for (i, name) in names.iter().enumerate() {
                let angle = i as f64 / names.len() as f64 * std::f64::consts::TAU;
                d.node(
                    name.clone(),
                    format!("degree {}", graph[name].len()),
                    d.width / 2.0 + radius * angle.cos(),
                    radius + 70.0 + radius * angle.sin(),
                    false,
                );
                d.rows.push((
                    name.clone(),
                    graph[name].iter().cloned().collect::<Vec<_>>().join(", "),
                ));
            }
            for (i, a) in names.iter().enumerate() {
                for (j, b) in names.iter().enumerate().skip(i + 1) {
                    if graph[a].contains(b) {
                        d.edge(i, j, false, false);
                    }
                }
            }
        }
        "dsu" => {
            let mut sets = Dsu::default();
            for pair in items {
                let Some((a, b)) = pair.split_once('-') else {
                    continue;
                };
                sets.union(a, b);
            }
            let names: Vec<_> = sets.parents.keys().cloned().collect();
            let mut depth_rows: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
            for name in &names {
                let mut root = name.clone();
                let mut depth = 0;
                while sets.parents[&root] != root {
                    root = sets.parents[&root].clone();
                    depth += 1;
                }
                let id = d.node(
                    name.clone(),
                    if depth == 0 {
                        "root".into()
                    } else {
                        format!("parent {}", sets.parents[name])
                    },
                    0.0,
                    60.0 + depth as f64 * 120.0,
                    depth == 0,
                );
                depth_rows.entry(depth).or_default().push(id);
                d.rows
                    .push((name.clone(), format!("representative {root}")));
            }
            d.width =
                (depth_rows.values().map(Vec::len).max().unwrap_or(1) as f64 * 150.0).max(1000.0);
            for row in depth_rows.values() {
                for (i, id) in row.iter().enumerate() {
                    d.nodes[*id].x = d.width * (i + 1) as f64 / (row.len() + 1) as f64;
                }
            }
            for (i, name) in names.iter().enumerate() {
                if sets.parents[name] != *name {
                    if let Some(parent) = names.iter().position(|x| x == &sets.parents[name]) {
                        d.edge(i, parent, true, false);
                    }
                }
            }
        }
        "segment" => {
            d.width = (nums.len() as f64 * 130.0).max(1000.0);
            if !nums.is_empty() {
                segment_draw(&nums, 0, nums.len(), 0, &mut d);
            }
        }
        "fenwick" => {
            let bit = fenwick(&nums);
            for (i, value) in bit.iter().enumerate().skip(1) {
                let lo = i - (i & i.wrapping_neg()) + 1;
                d.node(
                    value.to_string(),
                    format!("i={i} covers {lo}..{i}"),
                    90.0 + ((i - 1) % 6) as f64 * 160.0,
                    70.0 + ((i - 1) / 6) as f64 * 120.0,
                    false,
                );
                d.rows
                    .push((format!("prefix({i})"), prefix(&bit, i).to_string()));
                let parent = i + (i & i.wrapping_neg());
                if parent < bit.len() {
                    d.edge(i - 1, parent - 1, true, false);
                }
            }
            d.rows
                .insert(0, ("Input array".into(), format!("{nums:?}")));
        }
        _ => {
            for (i, value) in items.iter().enumerate() {
                let note = if kind == "stack" && i + 1 == items.len() {
                    "top".into()
                } else if matches!(kind, "queue" | "deque") && i == 0 {
                    "front".into()
                } else if matches!(kind, "queue" | "deque") && i + 1 == items.len() {
                    "back".into()
                } else {
                    format!("index {i}")
                };
                let (x, y) = if kind == "stack" {
                    (500.0, 60.0 + (items.len() - 1 - i) as f64 * 90.0)
                } else {
                    (85.0 + (i % 6) as f64 * 165.0, 70.0 + (i / 6) as f64 * 130.0)
                };
                d.node(value.clone(), note, x, y, i + 1 == items.len());
                if i > 0 && matches!(kind, "singly" | "doubly" | "circular") {
                    d.edge(i - 1, i, true, kind == "doubly");
                }
            }
            if kind == "circular" && !items.is_empty() {
                d.edge(items.len() - 1, 0, true, false);
            }
            if matches!(kind, "singly" | "doubly") {
                d.rows.push((
                    "Boundary".into(),
                    if kind == "doubly" {
                        "head.prev = null; tail.next = null".into()
                    } else {
                        "tail.next = null".into()
                    },
                ));
            }
        }
    }
    d
}
/// Mutations operate on the currently displayed prefix. Every result is revalidated.
pub fn mutate(
    kind: &str,
    items: &[String],
    op: &str,
    value: &str,
    index: usize,
) -> Result<(Vec<String>, String), String> {
    let mut next = items.to_vec();
    let message = match op {
        "insert" => {
            let entry = value.trim();
            if entry.is_empty() || entry.contains(',') {
                return Err("Enter exactly one item.".into());
            }
            validate(kind, &[entry.into()])?;
            if matches!(kind, "array" | "singly" | "doubly" | "circular") {
                if index > next.len() {
                    return Err("Insertion index must be between 0 and length.".into());
                }
                next.insert(index, entry.into());
            } else {
                next.push(entry.into());
            }
            format!("Inserted {entry}. The diagram shows the completed operation.")
        }
        "front" if kind == "deque" => {
            number(value)?;
            next.insert(0, value.into());
            format!("Pushed {value} at the front.")
        }
        "remove" => {
            if next.is_empty() {
                return Err("The structure is empty; nothing to remove.".into());
            }
            let removed = match kind {
                "stack" | "deque" => next.pop().unwrap(),
                "queue" => next.remove(0),
                "heap" => {
                    let mut heap = Vec::new();
                    for item in &next {
                        heap_push(&mut heap, number(item)?);
                    }
                    let min = heap_pop(&mut heap).unwrap();
                    next = heap.iter().map(ToString::to_string).collect();
                    min.to_string()
                }
                "array" | "singly" | "doubly" | "circular" => {
                    if index >= next.len() {
                        return Err("Removal index must be smaller than length.".into());
                    }
                    next.remove(index)
                }
                "hashset" | "hashmap" | "trie" => {
                    let before = next.len();
                    next.retain(|item| item.split(':').next().unwrap_or(item) != value);
                    if before == next.len() {
                        return Err("That key or word is absent.".into());
                    }
                    value.into()
                }
                _ => {
                    return Err(
                        "This structure supports insertion and rebuilding from edited input."
                            .into(),
                    )
                }
            };
            format!("Removed {removed}.")
        }
        "popfront" if kind == "deque" => {
            if next.is_empty() {
                return Err("The deque is empty.".into());
            }
            format!("Removed {} from the front.", next.remove(0))
        }
        "set" if matches!(kind, "fenwick" | "segment" | "array") => {
            number(value)?;
            if index >= next.len() {
                return Err("Update index must be smaller than length.".into());
            }
            next[index] = value.into();
            format!("Set zero-based index {index} to {value}; rebuilt the diagram.")
        }
        _ => return Err("Unsupported operation.".into()),
    };
    validate(kind, &next)?;
    Ok((next, message))
}
