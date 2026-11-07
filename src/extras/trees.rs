use super::*;
#[derive(Clone, Debug)]
struct Node {
    key: i64,
    height: i32,
    red: bool,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}
impl Node {
    fn new(key: i64) -> Box<Self> {
        Box::new(Self {
            key,
            height: 1,
            red: true,
            left: None,
            right: None,
        })
    }
}
fn height(n: &Option<Box<Node>>) -> i32 {
    n.as_ref().map_or(0, |n| n.height)
}
fn update(n: &mut Node) {
    n.height = 1 + height(&n.left).max(height(&n.right));
}
fn red(n: &Option<Box<Node>>) -> bool {
    n.as_ref().is_some_and(|n| n.red)
}
fn left(mut n: Box<Node>) -> Box<Node> {
    let Some(mut child) = n.right.take() else {
        return n;
    };
    n.right = child.left.take();
    update(&mut n);
    child.left = Some(n);
    update(&mut child);
    child
}
fn right(mut n: Box<Node>) -> Box<Node> {
    let Some(mut child) = n.left.take() else {
        return n;
    };
    n.left = child.right.take();
    update(&mut n);
    child.right = Some(n);
    update(&mut child);
    child
}
fn avl(node: Option<Box<Node>>, key: i64, events: &mut Vec<(Box<Node>, String)>) -> Box<Node> {
    let Some(mut n) = node else {
        return Node::new(key);
    };
    if key < n.key {
        n.left = Some(avl(n.left.take(), key, events));
    } else if key > n.key {
        n.right = Some(avl(n.right.take(), key, events));
    } else {
        return n;
    }
    update(&mut n);
    let balance = height(&n.left) - height(&n.right);
    if balance > 1 {
        if n.left.as_ref().is_some_and(|c| key > c.key) {
            if let Some(child) = n.left.take() {
                let child = left(child);
                events.push((
                    child.clone(),
                    "Local subtree: left rotation prepares the left-right case.".into(),
                ));
                n.left = Some(child);
            }
        }
        n = right(n);
        events.push((
            n.clone(),
            "Local subtree: right rotation restores the AVL height rule.".into(),
        ));
    } else if balance < -1 {
        if n.right.as_ref().is_some_and(|c| key < c.key) {
            if let Some(child) = n.right.take() {
                let child = right(child);
                events.push((
                    child.clone(),
                    "Local subtree: right rotation prepares the right-left case.".into(),
                ));
                n.right = Some(child);
            }
        }
        n = left(n);
        events.push((
            n.clone(),
            "Local subtree: left rotation restores the AVL height rule.".into(),
        ));
    }
    update(&mut n);
    n
}
fn llrb(node: Option<Box<Node>>, key: i64, events: &mut Vec<(Box<Node>, String)>) -> Box<Node> {
    let Some(mut n) = node else {
        return Node::new(key);
    };
    if key < n.key {
        n.left = Some(llrb(n.left.take(), key, events));
    } else if key > n.key {
        n.right = Some(llrb(n.right.take(), key, events));
    }
    if red(&n.right) && !red(&n.left) {
        let color = n.red;
        n = left(n);
        n.red = color;
        if let Some(c) = &mut n.left {
            c.red = true;
        }
        events.push((
            n.clone(),
            "Local subtree: rotate left to make the red link lean left.".into(),
        ));
    }
    if red(&n.left) && n.left.as_ref().is_some_and(|c| red(&c.left)) {
        let color = n.red;
        n = right(n);
        n.red = color;
        if let Some(c) = &mut n.right {
            c.red = true;
        }
        events.push((
            n.clone(),
            "Local subtree: rotate right to remove consecutive left-red links.".into(),
        ));
    }
    if red(&n.left) && red(&n.right) {
        n.red = !n.red;
        if let Some(c) = &mut n.left {
            c.red = !c.red;
        }
        if let Some(c) = &mut n.right {
            c.red = !c.red;
        }
        events.push((
            n.clone(),
            concat!(
                "Local subtree: flip colors to split a temporary 4-node. Root ",
                "blackening follows insertion."
            )
            .into(),
        ));
    }
    update(&mut n);
    n
}
fn binary_diagram(root: &Node, is_red_black: bool, active: i64) -> model::Diagram {
    fn visit(
        n: &Node,
        d: &mut model::Diagram,
        depth: usize,
        x: &mut usize,
        rb: bool,
        active: i64,
    ) -> usize {
        let id = d.nodes.len();
        d.nodes.push(model::Node {
            label: n.key.to_string(),
            note: if rb {
                if n.red {
                    "red node".into()
                } else {
                    "black node".into()
                }
            } else {
                format!(
                    "h={} · balance {}",
                    n.height,
                    height(&n.left) - height(&n.right)
                )
            },
            x: 0.0,
            y: 55.0 + depth as f64 * 110.0,
            active: n.key == active,
        });
        if let Some(c) = &n.left {
            let child = visit(c, d, depth + 1, x, rb, active);
            d.edges.push(model::Edge {
                from: id,
                to: child,
                directed: true,
                both: false,
            });
        }
        d.nodes[id].x = 70.0 + *x as f64 * 115.0;
        *x += 1;
        if let Some(c) = &n.right {
            let child = visit(c, d, depth + 1, x, rb, active);
            d.edges.push(model::Edge {
                from: id,
                to: child,
                directed: true,
                both: false,
            });
        }
        id
    }
    let mut d = model::Diagram::default();
    let mut x = 0;
    visit(root, &mut d, 0, &mut x, is_red_black, active);
    d.width = x as f64 * 115.0 + 50.0;
    d.height = root.height as f64 * 110.0 + 50.0;
    d
}
#[derive(Clone, Debug, Default)]
struct BNode {
    keys: Vec<i64>,
    children: Vec<BNode>,
}
impl BNode {
    fn contains(&self, key: i64) -> bool {
        match self.keys.binary_search(&key) {
            Ok(_) => true,
            Err(i) => self.children.get(i).is_some_and(|c| c.contains(key)),
        }
    }
    fn split(&mut self, index: usize) {
        let mut child = self.children.remove(index);
        let right_keys = child.keys.split_off(2);
        let median = child.keys.pop().expect("full child has a median");
        let right_children = if child.children.is_empty() {
            vec![]
        } else {
            child.children.split_off(2)
        };
        self.keys.insert(index, median);
        self.children.insert(index, child);
        self.children.insert(
            index + 1,
            BNode {
                keys: right_keys,
                children: right_children,
            },
        );
    }
    fn insert_nonfull(&mut self, key: i64, events: &mut Vec<(BNode, String)>) {
        let mut i = self.keys.partition_point(|v| *v < key);
        if self.children.is_empty() {
            self.keys.insert(i, key);
            return;
        }
        if self.children[i].keys.len() == 3 {
            self.split(i);
            events.push((
                self.clone(),
                "Local subtree: split a full child and promote its median key.".into(),
            ));
            if key > self.keys[i] {
                i += 1;
            }
        }
        self.children[i].insert_nonfull(key, events);
    }
    fn insert(&mut self, key: i64, events: &mut Vec<(BNode, String)>) {
        if self.contains(key) {
            return;
        }
        if self.keys.len() == 3 {
            let old = std::mem::take(self);
            self.children.push(old);
            self.split(0);
            events.push((
                self.clone(),
                "Split the full root. A new root adds one level while all leaves stay aligned."
                    .into(),
            ));
        }
        self.insert_nonfull(key, events);
    }
}
fn btree_diagram(root: &BNode, active: i64) -> model::Diagram {
    fn visit(n: &BNode, depth: usize, x: &mut usize, d: &mut model::Diagram, active: i64) -> usize {
        let id = d.nodes.len();
        d.nodes.push(model::Node {
            label: n
                .keys
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(" | "),
            note: if n.children.is_empty() {
                "leaf".into()
            } else {
                "routing keys".into()
            },
            x: 0.0,
            y: 60.0 + depth as f64 * 120.0,
            active: n.keys.contains(&active),
        });
        if n.children.is_empty() {
            d.nodes[id].x = 85.0 + *x as f64 * 170.0;
            *x += 1;
        } else {
            let children: Vec<_> = n
                .children
                .iter()
                .map(|c| visit(c, depth + 1, x, d, active))
                .collect();
            d.nodes[id].x = (d.nodes[children[0]].x
                + d.nodes[*children.last().expect("internal node has children")].x)
                / 2.0;
            for child in children {
                d.edges.push(model::Edge {
                    from: id,
                    to: child,
                    directed: true,
                    both: false,
                });
            }
        }
        id
    }
    let mut d = model::Diagram::default();
    let mut x = 0;
    visit(root, 0, &mut x, &mut d, active);
    d.width = x as f64 * 170.0 + 80.0;
    d.height = d.nodes.iter().map(|n| n.y).fold(0.0, f64::max) + 80.0;
    d
}
pub(super) fn run(kind: usize, values: Vec<i64>, e: &mut Exercise) {
    let mut unique = std::collections::BTreeSet::new();
    if kind == 8 {
        let mut root = BNode::default();
        for value in values {
            let mut events = vec![];
            let fresh = unique.insert(value);
            root.insert(value, &mut events);
            for (tree, note) in events {
                diagram(e, btree_diagram(&tree, value), note);
            }
            diagram(e,
btree_diagram(&root,
value),
format!("Full tree after {} {value}. Keys are sorted within nodes; leaves share a depth.",
if fresh {
"inserting"}
 else {
"ignoring duplicate"}
));
        }
    } else {
        let mut root = None;
        for value in values {
            let fresh = unique.insert(value);
            let mut events = vec![];
            let mut next = if kind == 6 {
                avl(root.take(), value, &mut events)
            } else {
                llrb(root.take(), value, &mut events)
            };
            if kind == 7 {
                next.red = false;
            }
            for (tree, note) in events {
                diagram(e, binary_diagram(&tree, kind == 7, value), note);
            }
            diagram(
                e,
                binary_diagram(&next, kind == 7, value),
                format!(
                    "Full tree after {} {value}. {}",
                    if fresh {
                        "inserting"
                    } else {
                        "ignoring duplicate"
                    },
                    if kind == 6 {
                        "Every balance factor is -1, 0 or 1."
                    } else {
                        "The root is black; every root-to-null path has equal black height."
                    }
                ),
            );
            root = Some(next);
        }
    }
    e.answer = format!("Ordered keys: {:?}", unique.into_iter().collect::<Vec<_>>());
}
#[cfg(test)]
mod tests {
    use super::*;
    fn check(n: &Option<Box<Node>>, lo: i64, hi: i64, rb: bool) -> (i32, usize) {
        let Some(n) = n else {
            return (0, 1);
        };
        assert!(lo < n.key && n.key < hi);
        let (lh, lb) = check(&n.left, lo, n.key, rb);
        let (rh, rbh) = check(&n.right, n.key, hi, rb);
        assert_eq!(n.height, 1 + lh.max(rh));
        if rb {
            assert_eq!(lb, rbh);
            assert!(!red(&n.right));
            if n.red {
                assert!(!red(&n.left) && !red(&n.right));
            }
        } else {
            assert!((lh - rh).abs() <= 1);
        }
        (n.height, lb + usize::from(!n.red))
    }
    #[test]
    fn rotations_preserve_order_balance_and_black_height() {
        for rb in [false, true] {
            for descending in [false, true] {
                let mut root = None;
                for i in 0..100 {
                    let value = if descending { 100 - i } else { i };
                    let mut next = if rb {
                        llrb(root.take(), value, &mut vec![])
                    } else {
                        avl(root.take(), value, &mut vec![])
                    };
                    next.red = false;
                    root = Some(next);
                    check(&root, i64::MIN, i64::MAX, rb);
                }
            }
        }
    }
    fn bcheck(
        n: &BNode,
        lo: i64,
        hi: i64,
        depth: usize,
        leaf: &mut Option<usize>,
        ordered: &mut Vec<i64>,
    ) {
        assert!((1..=3).contains(&n.keys.len()));
        assert!(n.keys.windows(2).all(|w| w[0] < w[1]));
        assert!(n.keys.iter().all(|k| lo < *k && *k < hi));
        if n.children.is_empty() {
            if let Some(d) = leaf {
                assert_eq!(*d, depth);
            } else {
                *leaf = Some(depth);
            }
            ordered.extend(&n.keys);
        } else {
            assert_eq!(n.children.len(), n.keys.len() + 1);
            for i in 0..=n.keys.len() {
                bcheck(
                    &n.children[i],
                    if i == 0 { lo } else { n.keys[i - 1] },
                    if i == n.keys.len() { hi } else { n.keys[i] },
                    depth + 1,
                    leaf,
                    ordered,
                );
                if i < n.keys.len() {
                    ordered.push(n.keys[i]);
                }
            }
        }
    }
    #[test]
    fn btree_splits_keep_order_and_equal_leaf_depth() {
        for reverse in [false, true] {
            let mut root = BNode::default();
            for i in 0..100 {
                let value = if reverse { 99 - i } else { i };
                root.insert(value, &mut vec![]);
                root.insert(value, &mut vec![]);
                let mut ordered = vec![];
                bcheck(&root, i64::MIN, i64::MAX, 0, &mut None, &mut ordered);
                assert_eq!(ordered.len(), i as usize + 1);
                assert!(ordered.windows(2).all(|w| w[0] < w[1]));
            }
        }
    }
}
