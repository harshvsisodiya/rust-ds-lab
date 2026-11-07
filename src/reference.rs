//! Standalone Rust examples, embedded as text and also compiled as tests.
pub const STRUCTURES: &[&str] = &[
    include_str!("../examples/reference/array.rs"),
    include_str!("../examples/reference/stack.rs"),
    include_str!("../examples/reference/queue.rs"),
    include_str!("../examples/reference/deque.rs"),
    include_str!("../examples/reference/singly.rs"),
    include_str!("../examples/reference/doubly.rs"),
    include_str!("../examples/reference/circular.rs"),
    include_str!("../examples/reference/hashmap.rs"),
    include_str!("../examples/reference/hashset.rs"),
    include_str!("../examples/reference/bst.rs"),
    include_str!("../examples/reference/heap.rs"),
    include_str!("../examples/reference/trie.rs"),
    include_str!("../examples/reference/graph.rs"),
    include_str!("../examples/reference/dsu.rs"),
    include_str!("../examples/reference/fenwick.rs"),
    include_str!("../examples/reference/segment.rs"),
];
pub const LESSONS: &[&str] = &[
    include_str!("../examples/reference/hashmap.rs"),
    include_str!("../examples/reference/pointers.rs"),
    include_str!("../examples/reference/brackets.rs"),
    include_str!("../examples/reference/binary.rs"),
    include_str!("../examples/reference/window.rs"),
    include_str!("../examples/reference/singly.rs"),
    include_str!("../examples/reference/bst.rs"),
    include_str!("../examples/reference/trie.rs"),
    include_str!("../examples/reference/heap.rs"),
    include_str!("../examples/reference/backtrack.rs"),
    include_str!("../examples/reference/graph.rs"),
    include_str!("../examples/reference/dp1.rs"),
    include_str!("../examples/reference/intervals.rs"),
    include_str!("../examples/reference/greedy.rs"),
    include_str!("../examples/reference/dijkstra.rs"),
    include_str!("../examples/reference/dp2.rs"),
    include_str!("../examples/reference/bits.rs"),
    include_str!("../examples/reference/gcd.rs"),
];
#[cfg(test)]
mod tests {
    mod array {
        include!("../examples/reference/array.rs");
        #[test]
        fn example_runs() {
            main();
        }
    }
    mod stack {
        include!("../examples/reference/stack.rs");
        #[test]
        fn example_runs() {
            main();
        }
    }
    mod queue {
        include!("../examples/reference/queue.rs");
        #[test]
        fn example_runs() {
            main();
        }
    }
    mod deque {
        include!("../examples/reference/deque.rs");
        #[test]
        fn example_runs() {
            main();
        }
    }
    mod singly {
        include!("../examples/reference/singly.rs");
        #[test]
        fn example_runs() {
            main();
        }
    }
    mod doubly {
        include!("../examples/reference/doubly.rs");
        #[test]
        fn example_runs() {
            main();
        }
    }
    mod circular {
        include!("../examples/reference/circular.rs");
        #[test]
        fn example_runs() {
            main();
        }
    }
    mod hashmap {
        include!("../examples/reference/hashmap.rs");
        #[test]
        fn example_runs() {
            main();
        }
    }
    mod hashset {
        include!("../examples/reference/hashset.rs");
        #[test]
        fn example_runs() {
            main();
        }
    }
    mod bst {
        include!("../examples/reference/bst.rs");
        #[test]
        fn example_runs() {
            main();
        }
    }
    mod heap {
        include!("../examples/reference/heap.rs");
        #[test]
        fn example_runs() {
            main();
        }
    }
    mod trie {
        include!("../examples/reference/trie.rs");
        #[test]
        fn example_runs() {
            main();
        }
    }
    mod graph {
        include!("../examples/reference/graph.rs");
        #[test]
        fn example_runs() {
            main();
        }
    }
    mod dsu {
        include!("../examples/reference/dsu.rs");
        #[test]
        fn example_runs() {
            main();
        }
    }
    mod fenwick {
        include!("../examples/reference/fenwick.rs");
        #[test]
        fn example_runs() {
            main();
        }
    }
    mod segment {
        include!("../examples/reference/segment.rs");
        #[test]
        fn example_runs() {
            main();
        }
    }
    mod pointers {
        include!("../examples/reference/pointers.rs");
        #[test]
        fn example_runs() {
            main();
        }
    }
    mod brackets {
        include!("../examples/reference/brackets.rs");
        #[test]
        fn example_runs() {
            main();
        }
    }
    mod binary {
        include!("../examples/reference/binary.rs");
        #[test]
        fn example_runs() {
            main();
        }
    }
    mod window {
        include!("../examples/reference/window.rs");
        #[test]
        fn example_runs() {
            main();
        }
    }
    mod backtrack {
        include!("../examples/reference/backtrack.rs");
        #[test]
        fn example_runs() {
            main();
        }
    }
    mod dp1 {
        include!("../examples/reference/dp1.rs");
        #[test]
        fn example_runs() {
            main();
        }
    }
    mod intervals {
        include!("../examples/reference/intervals.rs");
        #[test]
        fn example_runs() {
            main();
        }
    }
    mod greedy {
        include!("../examples/reference/greedy.rs");
        #[test]
        fn example_runs() {
            main();
        }
    }
    mod dijkstra {
        include!("../examples/reference/dijkstra.rs");
        #[test]
        fn example_runs() {
            main();
        }
    }
    mod dp2 {
        include!("../examples/reference/dp2.rs");
        #[test]
        fn example_runs() {
            main();
        }
    }
    mod bits {
        include!("../examples/reference/bits.rs");
        #[test]
        fn example_runs() {
            main();
        }
    }
    mod gcd {
        include!("../examples/reference/gcd.rs");
        #[test]
        fn example_runs() {
            main();
        }
    }
}
