use rust_ds_lab::{catalog::TOPICS, model::*};

#[test]
fn min_heap_matches_sorted_multiset() {
    let values = [9, -4, 2, 2, 0, 9999, -9999, 11, 7, 3];
    let mut heap = Vec::new();
    for v in values {
        heap_push(&mut heap, v);
        for i in 1..heap.len() {
            assert!(heap[(i - 1) / 2] <= heap[i]);
        }
    }
    let mut output = Vec::new();
    while let Some(v) = heap_pop(&mut heap) {
        output.push(v);
        for i in 1..heap.len() {
            assert!(heap[(i - 1) / 2] <= heap[i]);
        }
    }
    let mut expected = values.to_vec();
    expected.sort();
    assert_eq!(output, expected);
    assert_eq!(heap_pop(&mut heap), None);
}
#[test]
fn bst_search_and_duplicate_invariant() {
    let mut root = None;
    for n in [12, 7, 24, 9, 18, 30, 3, 7] {
        Bst::insert(&mut root, n);
    }
    for n in [12, 7, 24, 9, 18, 30, 3] {
        assert!(Bst::contains(&root, n));
    }
    assert!(!Bst::contains(&root, 99));
    let d = build("bst", &tokens("12,7,24,9,18,30,3,7"));
    assert_eq!(d.nodes.len(), 7);
    assert_eq!(d.edges.len(), 6);
    fn check(n: &Option<Box<Bst>>, lo: i64, hi: i64) {
        if let Some(n) = n {
            assert!(lo < n.value && n.value < hi);
            check(&n.left, lo, n.value);
            check(&n.right, n.value, hi);
        }
    }
    check(&root, i64::MIN, i64::MAX);
}
#[test]
fn fenwick_prefixes_match_direct_sums() {
    for n in 0..=16 {
        let values: Vec<i64> = (0..n).map(|i| i as i64 * 3 - 12).collect();
        let bit = fenwick(&values);
        for i in 0..=n {
            assert_eq!(prefix(&bit, i), values[..i].iter().sum());
        }
    }
}
#[test]
fn union_find_merges_and_compresses() {
    let mut d = Dsu::default();
    d.union("a", "b");
    d.union("c", "d");
    d.union("a", "c");
    d.union("x", "y");
    assert_eq!(d.find("a"), d.find("d"));
    assert_ne!(d.find("a"), d.find("x"));
    let root = d.find("a");
    assert_eq!(d.parents["d"], root);
    d.union("a", "d");
    assert_eq!(d.find("d"), root);
}
#[test]
fn all_diagrams_have_valid_edges_and_coordinates() {
    for topic in TOPICS {
        let items = tokens(topic.example);
        validate(topic.id, &items).unwrap();
        for count in 0..=items.len() {
            let d = build(topic.id, &items[..count]);
            for e in &d.edges {
                assert!(e.from < d.nodes.len());
                assert!(e.to < d.nodes.len());
            }
            for n in &d.nodes {
                assert!(n.x >= 0.0 && n.x <= d.width);
                assert!(n.y >= 0.0 && n.y <= d.height);
            }
        }
    }
}
#[test]
fn queue_stack_and_deque_order() {
    let input = tokens("1,2,3");
    assert_eq!(
        mutate("queue", &input, "remove", "", 0).unwrap().0,
        tokens("2,3")
    );
    assert_eq!(
        mutate("stack", &input, "remove", "", 0).unwrap().0,
        tokens("1,2")
    );
    assert_eq!(
        mutate("deque", &input, "front", "4", 0).unwrap().0,
        tokens("4,1,2,3")
    );
    assert_eq!(
        mutate("deque", &input, "popfront", "", 0).unwrap().0,
        tokens("2,3")
    );
}
#[test]
fn indexed_edits_and_errors_are_atomic() {
    let a = tokens("1,2,3");
    assert_eq!(
        mutate("array", &a, "insert", "9", 1).unwrap().0,
        tokens("1,9,2,3")
    );
    assert_eq!(
        mutate("array", &a, "set", "9", 1).unwrap().0,
        tokens("1,9,3")
    );
    assert!(mutate("array", &a, "remove", "", 3).is_err());
    assert!(mutate("array", &a, "insert", "4", 9).is_err());
    assert!(mutate("stack", &[], "remove", "", 0).is_err());
    assert_eq!(a, tokens("1,2,3"));
    assert!(mutate("stack", &vec!["1".into(); 16], "insert", "2", 0).is_err());
}
#[test]
fn hash_update_collision_and_delete() {
    assert_eq!(bucket("a"), bucket("i"));
    let items = tokens("a:1,i:2,a:9");
    let d = build("hashmap", &items);
    assert_eq!(d.nodes.iter().filter(|n| n.note == "entry").count(), 2);
    assert!(d.nodes.iter().any(|n| n.label == "a:9"));
    let next = mutate("hashmap", &items, "remove", "a", 0).unwrap().0;
    assert_eq!(next, tokens("i:2"));
    assert_eq!(build("hashset", &tokens("a,a,i")).nodes.len(), 10);
}
#[test]
fn trie_terminals_distinguish_prefixes() {
    let d = build("trie", &tokens("car,cart,cat,car"));
    assert_eq!(d.nodes.iter().filter(|n| n.active).count(), 3);
    assert_eq!(d.nodes.len(), 6);
}
#[test]
fn graph_deduplicates_reverse_edges() {
    let d = build("graph", &tokens("A-B,B-A,A-C"));
    assert_eq!(d.nodes.len(), 3);
    assert_eq!(d.edges.len(), 2);
    assert!(d.rows.iter().any(|(k, v)| k == "A" && v == "B, C"));
}
#[test]
fn segment_parent_sums_match_children() {
    for len in 1..=16 {
        let input: Vec<_> = (1..=len).map(|i| i.to_string()).collect();
        let d = build("segment", &input);
        assert_eq!(
            d.nodes[0].label.parse::<usize>().unwrap(),
            len * (len + 1) / 2
        );
        for (i, node) in d.nodes.iter().enumerate() {
            let children: Vec<_> = d.edges.iter().filter(|e| e.from == i).collect();
            if !children.is_empty() {
                assert_eq!(children.len(), 2);
                assert_eq!(
                    node.label.parse::<i64>().unwrap(),
                    children
                        .iter()
                        .map(|e| d.nodes[e.to].label.parse::<i64>().unwrap())
                        .sum::<i64>()
                );
            }
        }
    }
}
#[test]
fn malformed_and_oversized_inputs_are_rejected() {
    for (kind, data) in [
        ("array", "99999999"),
        ("graph", "A-A"),
        ("graph", "A-"),
        ("hashmap", "cat"),
        ("trie", "<script>"),
        ("trie", "Cat"),
    ] {
        assert!(validate(kind, &tokens(data)).is_err());
    }
    assert!(validate("array", &vec!["1".into(); 17]).is_err());
}
