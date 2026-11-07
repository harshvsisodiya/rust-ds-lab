//! Additional computed lessons. Each trace records real algorithm state.
use crate::{
    model,
    practice::{Exercise, Frame},
};
mod sequences;
mod sorting;
mod trees;
pub struct Topic {
    pub title: &'static str,
    pub group: &'static str,
    pub example: &'static str,
    pub why: &'static str,
    pub how: &'static str,
    pub when: &'static str,
    pub history: &'static str,
    pub source: &'static str,
    pub cost: &'static str,
}
pub const TOPICS: &[Topic] = &[
    Topic {
        title: "Bubble sort",
        group: "Sorting",
        example: "8,3,7,1,5,2",
        why: "Repeated adjacent comparisons give a simple way to see how ordering emerges.",
        how: concat!(
            "Compare neighbors, swap an inverted pair, and repeat passes. The ",
            "largest unsettled value reaches the end on each pass."
        ),
        when: concat!(
            "Useful for learning swaps and invariants. Quadratic work makes it ",
            "a poor default for large production inputs."
        ),
        history: concat!(
            "Adjacent-exchange sorting has a long history; this lesson does ",
            "not assign an exclusive inventor."
        ),
        source: "https://xlinux.nist.gov/dads/HTML/bubblesort.html",
        cost: "O(n²) average/worst time; O(n) best with early exit; O(1) auxiliary space.",
    },
    Topic {
        title: "Insertion sort",
        group: "Sorting",
        example: "8,3,7,1,5,2",
        why: "Maintain a sorted prefix while receiving one new value at a time.",
        how: concat!(
            "Grow a sorted prefix by swapping each new value left past larger ",
            "neighbors. This stable adjacent-swap variant makes each move ",
            "visible."
        ),
        when: concat!(
            "Useful for small or nearly sorted sequences and as a small-input ",
            "component of hybrid sorting algorithms."
        ),
        history: concat!(
            "The procedure mirrors hand sorting of cards and is a foundational ",
            "comparison method; no unique invention date is claimed."
        ),
        source: "https://xlinux.nist.gov/dads/HTML/insertionSort.html",
        cost: "O(n²) worst time; O(n) nearly sorted/best; O(1) auxiliary space.",
    },
    Topic {
        title: "Selection sort",
        group: "Sorting",
        example: "8,3,7,1,5,2",
        why: concat!(
            "Choose the next smallest value explicitly, making the ",
            "sorted-prefix invariant easy to inspect."
        ),
        how: concat!(
            "Find the minimum of the remaining suffix, swap it into the next ",
            "output position, then advance the boundary."
        ),
        when: concat!(
            "Useful for teaching selection and minimizing swap count. It still ",
            "scans quadratically and is generally not a fast general-purpose ",
            "sort."
        ),
        history: concat!(
            "Selection sorting is a foundational family of comparison methods ",
            "without a unique inventor asserted here."
        ),
        source: "https://xlinux.nist.gov/dads/HTML/selectionSort.html",
        cost: "O(n²) comparisons in all cases; O(n) swaps; O(1) auxiliary space.",
    },
    Topic {
        title: "Merge sort",
        group: "Sorting",
        example: "8,3,7,1,5,2",
        why: concat!(
            "Combine small ordered sequences into a large ordered sequence ",
            "with predictable comparison growth."
        ),
        how: concat!(
            "Recursively divide into halves, sort each half, then merge by ",
            "choosing the smaller front value. Equal values come from the left ",
            "first for stability."
        ),
        when: concat!(
            "Useful for stable sorting, linked sequences and merging external ",
            "runs that are too large for memory."
        ),
        history: concat!(
            "John von Neumann's 1945 work is widely associated with early ",
            "merge sorting. This is a historical milestone, not a claim about ",
            "every merging technique."
        ),
        source: "https://xlinux.nist.gov/dads/HTML/mergesort.html",
        cost: "O(n log n) time; O(n) temporary array space plus O(log n) recursion.",
    },
    Topic {
        title: "Quicksort",
        group: "Sorting",
        example: "8,3,7,1,5,2",
        why: "Partition around one value so two smaller regions can be sorted independently.",
        how: concat!(
            "This example uses the last value as pivot. Move values no larger ",
            "than it left, place the pivot, and recurse on the two sides."
        ),
        when: concat!(
            "Partitioning is a core technique in sorting and selection. A ",
            "fixed last-element pivot can degenerate on ordered input; ",
            "production implementations use additional safeguards."
        ),
        history: concat!(
            "Tony Hoare published his detailed Quicksort paper in 1962. ",
            "Pivot and partition variants have evolved since then."
        ),
        source: "https://doi.org/10.1093/comjnl/5.1.10",
        cost: "O(n log n) average time, O(n²) worst; recursion O(log n) average / O(n) worst.",
    },
    Topic {
        title: "Heapsort",
        group: "Sorting",
        example: "8,3,7,1,5,2",
        why: concat!(
            "Use a heap to repeatedly select the largest remaining value ",
            "without a separate output array."
        ),
        how: concat!(
            "Build a max heap, move the root to the last unsettled position, ",
            "shrink the heap, and restore its order by sifting down."
        ),
        when: concat!(
            "Useful when a comparison sort needs O(n log n) worst-case work ",
            "and constant auxiliary array space. It is not stable."
        ),
        history: concat!(
            "J. W. J. Williams published heapsort in 1964; later refinements ",
            "improved heap construction."
        ),
        source: "https://xlinux.nist.gov/dads/HTML/heapSort.html",
        cost: concat!(
            "O(n log n) worst-case time; O(1) auxiliary space for the ",
            "iterative sift implementation."
        ),
    },
    Topic {
        title: "AVL tree",
        group: "Balanced trees",
        example: "30,20,10,25,28,40,50",
        why: concat!(
            "Prevent a search tree from becoming a long chain after ",
            "unfortunate insertion order."
        ),
        how: concat!(
            "Track subtree heights. After insertion, use single or double ",
            "rotations whenever a node's height difference exceeds one."
        ),
        when: concat!(
            "Useful for ordered lookup where strict height balance matters. ",
            "This lab visualizes insertion and rotations; deletion is not ",
            "included."
        ),
        history: concat!(
            "Georgy Adelson-Velsky and Evgenii Landis introduced the AVL ",
            "balance condition in 1962."
        ),
        source: "https://xlinux.nist.gov/dads/HTML/avltree.html",
        cost: concat!(
            "O(log n) lookup/insertion; O(n) stored nodes and O(log n) ",
            "recursive insertion stack."
        ),
    },
    Topic {
        title: "Red-black tree",
        group: "Balanced trees",
        example: "30,20,10,25,28,40,50",
        why: concat!(
            "Keep ordered operations logarithmic using color rules with a less ",
            "rigid balance condition than AVL."
        ),
        how: concat!(
            "This insertion-only left-leaning variant rotates right-leaning ",
            "red links, fixes consecutive red links, splits temporary 4-nodes ",
            "by color flips, and makes the root black."
        ),
        when: concat!(
            "Red-black families support ordered maps and sets. This lab shows ",
            "left-leaning insertion, not all production variants or deletion."
        ),
        history: concat!(
            "Rudolf Bayer's symmetric binary B-tree work preceded the ",
            "red-black formulation. Guibas and Sedgewick described the ",
            "red-black framework in 1978; left-leaning formulations came later."
        ),
        source: "https://sedgewick.io/wp-content/uploads/2022/03/2008-09LLRB.pdf",
        cost: "O(log n) search/insertion; O(n) nodes and O(log n) recursive stack.",
    },
    Topic {
        title: "B-tree",
        group: "Balanced trees",
        example: "10,20,5,6,12,30,7,17",
        why: concat!(
            "Store multiple keys per node so a search crosses fewer levels and ",
            "can work efficiently with page-sized storage."
        ),
        how: concat!(
            "This minimum-degree-2 B-tree keeps up to three keys per node. ",
            "Split full children before descending; promote a median to the ",
            "parent. All leaves remain at one depth."
        ),
        when: concat!(
            "B-tree families are used in storage indexes. This small in-memory ",
            "example explains insertion and splitting; it does not simulate ",
            "disk I/O or B+ tree leaf layouts."
        ),
        history: "Rudolf Bayer and Edward McCreight developed B-trees around 1970–1972.",
        source: "https://xlinux.nist.gov/dads/HTML/btree.html",
        cost: concat!(
            "O(log n) levels for fixed degree; O(n) storage. Node comparisons ",
            "depend on degree."
        ),
    },
    Topic {
        title: "LRU cache",
        group: "Patterns",
        example: "1,2,3,1,4,2,5,1",
        why: concat!(
            "Keep a bounded set of recently used entries and evict the one ",
            "that has waited longest without an access."
        ),
        how: concat!(
            "With capacity three, move every accessed key to the most-recent ",
            "end. Insert a missing key and evict the least recent key if ",
            "capacity is exceeded."
        ),
        when: concat!(
            "Useful for bounded reuse caches when recent access predicts ",
            "future use. Scan-heavy workloads can defeat this assumption."
        ),
        history: concat!(
            "Least-recently-used replacement predates modern software caches ",
            "and appears in memory-management literature; no unique inventor ",
            "is assigned here."
        ),
        source: "https://xlinux.nist.gov/dads/HTML/leastRecentlyUsed.html",
        cost: concat!(
            "Production hash-map + linked-list designs: expected O(1) access. ",
            "This bounded Vec teaching model uses O(capacity) moves."
        ),
    },
    Topic {
        title: "Monotonic stack",
        group: "Patterns",
        example: "2,1,5,3,4,7",
        why: concat!(
            "Find a next greater value without scanning the remaining suffix ",
            "for every position."
        ),
        how: concat!(
            "Store unresolved indices in decreasing-value order. The next ",
            "larger value resolves and pops smaller values on top; then push ",
            "its own index."
        ),
        when: concat!(
            "Useful for next-greater-element queries, span problems and ",
            "histogram reasoning. Each index enters and leaves the stack at ",
            "most once."
        ),
        history: concat!(
            "Monotonic stacks describe a technique family rather than one ",
            "structure with an agreed invention date."
        ),
        source: "https://xlinux.nist.gov/dads/HTML/stack.html",
        cost: concat!(
            "O(n) time, O(n) auxiliary space; equal values remain unresolved ",
            "until a strictly greater value arrives."
        ),
    },
    Topic {
        title: "Topological sort",
        group: "Graphs",
        example: "A-B,A-C,B-D,C-D",
        why: "Order tasks so every dependency appears before the task that needs it.",
        how: concat!(
            "Kahn's algorithm repeatedly removes a vertex with zero incoming ",
            "edges and decreases its successors' indegrees. If vertices remain ",
            "but none are ready, the graph has a cycle."
        ),
        when: concat!(
            "Useful for build dependencies, course prerequisites and ",
            "scheduling a directed acyclic graph. A cyclic dependency has no ",
            "topological order."
        ),
        history: "Arthur Kahn published a topological-sorting algorithm in 1962.",
        source: "https://xlinux.nist.gov/dads/HTML/topologicalSort.html",
        cost: concat!(
            "O(V + E) traversal after adjacency is built; O(V + E) ",
            "representation. Ordered teaching maps add lookup overhead."
        ),
    },
    Topic {
        title: "KMP string matching",
        group: "Strings",
        example: "ababcabcabababd|ababd",
        why: concat!(
            "Reuse information from a matched prefix instead of restarting ",
            "every comparison after a mismatch."
        ),
        how: concat!(
            "Build an LPS table: each entry is the longest proper prefix that ",
            "is also a suffix. On mismatch, jump the pattern index through ",
            "this table while the text index stays put."
        ),
        when: concat!(
            "Useful for exact matching of a pattern in a long stream. This ",
            "visual lesson uses ASCII characters and reports all match starts, ",
            "including overlaps."
        ),
        history: concat!(
            "Knuth, Morris and Pratt published their linear-time ",
            "string-matching work in 1977, building on earlier independent ",
            "insights."
        ),
        source: "https://xlinux.nist.gov/dads/HTML/knuthMorrisPratt.html",
        cost: "O(text length + pattern length) time; O(pattern length) auxiliary space.",
    },
];
pub fn trace(topic: usize, input: &str, case: usize) -> Result<Exercise, String> {
    let t = TOPICS.get(topic).ok_or("Unknown lesson")?;
    let mut e = Exercise {
        title: t.title.into(),
        difficulty: if case < 3 {
            "Easy"
        } else if case < 7 {
            "Medium"
        } else {
            "Hard"
        },
        prompt: format!(
            "Trace {} for {input}. Predict the final result and explain the rule that stays true.",
            t.title
        ),
        explanation: t.how.into(),
        cost: t.cost,
        frames: vec![],
        answer: String::new(),
    };
    if topic == 12 {
        sequences::kmp(input, &mut e)?;
    } else if topic == 11 {
        sequences::topological(input, &mut e)?;
    } else {
        let items = model::tokens(input);
        model::validate("array", &items)?;
        if items.is_empty() {
            return Err("Enter at least one number.".into());
        }
        let a: Vec<i64> = items
            .iter()
            .map(|s| s.parse::<i64>().expect("validated integer"))
            .collect();
        match topic {
            0..=5 => sorting::run(topic, a, &mut e),
            6..=8 => trees::run(topic, a, &mut e),
            9 => sequences::lru(a, &mut e),
            10 => sequences::monotonic(a, &mut e),
            _ => unreachable!(),
        }
    }
    Ok(e)
}
pub fn example(topic: usize, case: usize) -> String {
    if case == 0 {
        return TOPICS[topic].example.into();
    }
    if topic == 12 {
        return [
            "aaaaa|aa",
            "abcdef|gh",
            "abcabcabc|abc",
            "abababa|aba",
            "mississippi|issi",
            "a|a",
            "abc|abcd",
            "ababacababac|ababac",
            "abcababcab|abcab",
        ][case.min(9) - 1]
            .into();
    }
    if topic == 11 {
        return [
            "A-B,B-C,C-A",
            "A-B,C-D",
            "A-B,A-C,A-D",
            "A-C,B-C,C-D,D-E",
            "A-B,B-A",
            "A-B,B-C,C-D,D-E",
            "A-B,A-B,B-C",
            "A-C,B-C,C-D,B-E",
            "A-B,A-C,B-D,C-D,D-E,E-F",
        ][case.min(9) - 1]
            .into();
    }
    let cases = [
        vec![1, 2, 3, 4, 5, 6],
        vec![6, 5, 4, 3, 2, 1],
        vec![3, 1, 3, 2, 1],
        vec![-3, 0, 7, -1, 5],
        vec![9],
        vec![5, 5, 5, 5],
        vec![12, 3, 8, -5, 0, 7, 2, 11],
        vec![0, -1, -2, -3, 4, 5, 6],
        vec![8, 7, 6, 5, 4, 3, 2, 1, 0, -1],
    ];
    cases[case.min(9) - 1]
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(",")
}
pub(super) fn cells(e: &mut Exercise, values: Vec<String>, active: Vec<usize>, note: String) {
    e.frames.push(Frame {
        cells: values,
        active,
        note,
        columns: 6,
        edges: vec![],
        diagram: None,
    });
}
pub(super) fn diagram(e: &mut Exercise, d: model::Diagram, note: String) {
    e.frames.push(Frame {
        cells: d.nodes.iter().map(|n| n.label.clone()).collect(),
        active: d
            .nodes
            .iter()
            .enumerate()
            .filter(|(_, n)| n.active)
            .map(|(i, _)| i)
            .collect(),
        note,
        columns: 6,
        edges: vec![],
        diagram: Some(d),
    });
}
pub fn rust_source(topic: usize) -> &'static str {
    match topic {
        0..=5 => include_str!("sorting.rs"),
        6..=8 => include_str!("trees.rs"),
        _ => include_str!("sequences.rs"),
    }
}
