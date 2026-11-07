pub struct Lesson {
    pub id: &'static str,
    pub title: &'static str,
    pub x: f32,
    pub y: f32,
    pub why: &'static str,
    pub how: &'static str,
    pub when: &'static str,
    pub industry: &'static str,
    pub history: &'static str,
    pub structures: &'static str,
    pub source: &'static str,
}
pub const LESSONS: &[Lesson] = &[
    Lesson {
        id: r###"arrays"###,
        title: r###"Arrays & Hashing"###,
        x: 0.0,
        y: 0.0,
        why: r###"Store many values and retrieve them by position or key."###,
        how: r###"An array keeps adjacent slots; a hash table routes a key to a bucket, \
                        then resolves collisions."###,
        when: concat!(
            "Use arrays for dense sequences and maps for keyed lookup. Arrays ",
            "avoid pointer chasing; hashing trades extra memory for average ",
            "constant lookup."
        ),
        industry: r###"Sensor samples, inventory keyed by SKU, counting events in a log."###,
        history: concat!(
            "Arrays are foundational indexed memory. Hashing developed in ",
            "early computing; there is no single inventor asserted here."
        ),
        structures: r###"array,hashmap,hashset"###,
        source: r###"https://xlinux.nist.gov/dads/HTML/hashtab.html"###,
    },
    Lesson {
        id: r###"pointers"###,
        title: r###"Two Pointers"###,
        x: -1.0,
        y: 1.0,
        why: r###"Avoid rechecking every pair in a sorted sequence."###,
        how: concat!(
            "Maintain two positions. Move one only when the ordering tells you ",
            "which candidates can be ruled out."
        ),
        when: concat!(
            "Use sorted pair search, in-place partitioning and sequence ",
            "comparison. The monotonic movement gives a linear scan."
        ),
        industry: r###"Comparing sorted event streams or merging ordered records."###,
        history: concat!(
            "This is a family of traversal techniques, not one data structure ",
            "with a universally agreed invention date."
        ),
        structures: r###"array"###,
        source: r###"https://xlinux.nist.gov/dads/HTML/merge.html"###,
    },
    Lesson {
        id: r###"stack"###,
        title: r###"Stack"###,
        x: 1.0,
        y: 1.0,
        why: r###"Remember unfinished work in reverse order."###,
        how: concat!(
            "Push at the top; pop the most recently added item. Matching ",
            "brackets uses the stack to remember opening symbols."
        ),
        when: r###"Use nesting, undo, expression evaluation and depth-first worklists."###,
        industry: r###"Function calls, syntax parsers and undo history."###,
        history: concat!(
            "Stacks became central to language execution in early computing. ",
            "Do not confuse one early implementation with invention of the ",
            "abstract LIFO idea."
        ),
        structures: r###"stack"###,
        source: r###"https://xlinux.nist.gov/dads/HTML/stack.html"###,
    },
    Lesson {
        id: r###"binary"###,
        title: r###"Binary Search"###,
        x: -2.0,
        y: 2.0,
        why: r###"Find an ordered value without scanning every entry."###,
        how: r###"Compare the midpoint and keep only the half that can still contain the target."###,
        when: concat!(
            "Use sorted arrays or monotonic predicates. Random access is ",
            "important; linked-list midpoint access is costly."
        ),
        industry: concat!(
            "Lookup in sorted indexes and finding a threshold in a monotonic ",
            "configuration space."
        ),
        history: concat!(
            "Halving a search interval is an old mathematical idea; no unique ",
            "inventor is claimed for the general method."
        ),
        structures: r###"array"###,
        source: r###"https://xlinux.nist.gov/dads/HTML/binarySearch.html"###,
    },
    Lesson {
        id: r###"window"###,
        title: r###"Sliding Window"###,
        x: 0.0,
        y: 2.0,
        why: r###"Reuse work when neighboring ranges overlap."###,
        how: concat!(
            "Add the entering value and remove the leaving value. Variable ",
            "windows also move the left boundary when a constraint breaks."
        ),
        when: concat!(
            "Use contiguous stream statistics; ensure the maintained summary ",
            "can be updated correctly."
        ),
        industry: r###"Moving averages, recent-event counts and network monitoring."###,
        history: concat!(
            "Sliding windows describe several streaming techniques, not a ",
            "single invention with one date."
        ),
        structures: r###"queue,deque"###,
        source: r###"https://docs.python.org/3/library/collections.html#collections.deque"###,
    },
    Lesson {
        id: r###"linked"###,
        title: r###"Linked List"###,
        x: 2.0,
        y: 2.0,
        why: r###"Insert or remove nodes without shifting every later element."###,
        how: concat!(
            "Nodes store values and links. Rewire neighboring links; searching ",
            "for a position still takes time."
        ),
        when: concat!(
            "Use when stable node identities and known-node edits matter. ",
            "Arrays often win for sequential scans due to locality."
        ),
        industry: r###"Free lists, intrusive OS lists and the recency chain in an LRU cache."###,
        history: concat!(
            "Newell, Shaw and Simon developed influential list-processing work ",
            "in 1955–1956 at RAND and Carnegie. This is a historical ",
            "milestone, not a claim about every list variant."
        ),
        structures: r###"singly,doubly,circular"###,
        source: r###"https://sova.si.edu/record/nmah.ac.0580"###,
    },
    Lesson {
        id: r###"trees"###,
        title: r###"Trees"###,
        x: 0.0,
        y: 3.0,
        why: r###"Represent hierarchy and narrow searches through branches."###,
        how: concat!(
            "A root leads to children. A BST additionally orders keys: smaller ",
            "left, larger right."
        ),
        when: concat!(
            "Use hierarchy and ordered lookup. Unbalanced BSTs can become ",
            "chains; balanced variants keep height small."
        ),
        industry: r###"Directories, document syntax trees and ordered indexes."###,
        history: concat!(
            "Trees have mathematical roots predating computers. BSTs and ",
            "balanced variants evolved separately; there is no single ",
            "invention date for all trees."
        ),
        structures: r###"bst"###,
        source: r###"https://xlinux.nist.gov/dads/HTML/binarySearchTree.html"###,
    },
    Lesson {
        id: r###"tries"###,
        title: r###"Tries"###,
        x: -2.0,
        y: 4.0,
        why: concat!(
            "Share repeated prefixes instead of storing and scanning every ",
            "full word independently."
        ),
        how: concat!(
            "Follow one character per edge; a terminal flag distinguishes a ",
            "complete word from a mere prefix."
        ),
        when: concat!(
            "Use prefix lookup, dictionaries and routing keys. Sparse children ",
            "can save memory compared with a fixed full alphabet at every node."
        ),
        industry: r###"Autocomplete dictionaries and prefix-based lookup."###,
        history: concat!(
            "Trie is a distinct prefix-tree family; its name comes from ",
            "retrieval. See the linked reference for definitions and ",
            "historical references."
        ),
        structures: r###"trie"###,
        source: r###"https://xlinux.nist.gov/dads/HTML/trie.html"###,
    },
    Lesson {
        id: r###"heap"###,
        title: r###"Heap / Priority Queue"###,
        x: -1.0,
        y: 5.0,
        why: r###"Repeatedly obtain the most urgent or smallest item."###,
        how: concat!(
            "A complete binary tree fits in an array. A min-heap keeps each ",
            "parent no larger than its children."
        ),
        when: concat!(
            "Use scheduling and top-k streams. A heap is partially ordered; it ",
            "is not a globally sorted array."
        ),
        industry: r###"Job schedulers, event simulation and shortest-path frontiers."###,
        history: concat!(
            "J. W. J. Williams published heapsort in 1964, an important ",
            "binary-heap milestone."
        ),
        structures: r###"heap"###,
        source: r###"https://xlinux.nist.gov/dads/HTML/heapSort.html"###,
    },
    Lesson {
        id: r###"backtrack"###,
        title: r###"Backtracking"###,
        x: 2.0,
        y: 4.0,
        why: r###"Explore choices while discarding branches that cannot produce a valid answer."###,
        how: r###"Choose, recurse, undo. Every return restores the state expected by the parent."###,
        when: concat!(
            "Use small combinatorial spaces and constraint search. Output ",
            "itself can be exponential, so bound demonstrations."
        ),
        industry: r###"Puzzle solvers, configuration search and constraint engines."###,
        history: concat!(
            "Systematic search predates computing. Backtracking is a broad ",
            "technique; no sole inventor is claimed here."
        ),
        structures: r###"stack"###,
        source: r###"https://xlinux.nist.gov/dads/HTML/backtrack.html"###,
    },
    Lesson {
        id: r###"graphs"###,
        title: r###"Graphs"###,
        x: 1.0,
        y: 5.0,
        why: r###"Model relationships that do not fit one parent-child hierarchy."###,
        how: concat!(
            "Vertices represent entities and edges represent relationships. ",
            "BFS uses a queue; DFS uses a stack or recursion."
        ),
        when: concat!(
            "Use adjacency lists for sparse graphs; matrices may suit dense ",
            "graphs. Mark visited nodes to avoid cycles."
        ),
        industry: r###"Road networks, dependencies, social links and connectivity."###,
        history: concat!(
            "Graph theory predates electronic computers. Individual traversal ",
            "and optimization methods have separate histories."
        ),
        structures: r###"graph,queue"###,
        source: r###"https://xlinux.nist.gov/dads/HTML/graph.html"###,
    },
    Lesson {
        id: r###"dp1"###,
        title: r###"1-D Dynamic Programming"###,
        x: 3.0,
        y: 5.0,
        why: r###"Avoid repeatedly solving the same smaller problem."###,
        how: concat!(
            "Define the state, derive a recurrence, initialize base cases and ",
            "evaluate dependencies first."
        ),
        when: concat!(
            "Use overlapping subproblems with reusable answers. Rolling state ",
            "can reduce memory when old states are no longer needed."
        ),
        industry: r###"Planning, counting valid sequences and selecting compatible rewards."###,
        history: concat!(
            "Richard Bellman developed dynamic programming in the 1950s; a ",
            "1952 paper and his 1957 book are landmarks."
        ),
        structures: r###"array"###,
        source: r###"https://pubmed.ncbi.nlm.nih.gov/16589462/"###,
    },
    Lesson {
        id: r###"intervals"###,
        title: r###"Intervals"###,
        x: -3.0,
        y: 6.0,
        why: r###"Reason about spans of time or positions."###,
        how: concat!(
            "Sort endpoints or intervals; compare a current span with the next ",
            "one and merge or select as required."
        ),
        when: concat!(
            "Use calendars, reservations and range consolidation. Decide ",
            "explicitly whether touching endpoints overlap."
        ),
        industry: r###"Calendar availability, reservation conflicts and address ranges."###,
        history: concat!(
            "Intervals are mathematical objects. Merging and scheduling are ",
            "separate algorithmic problems with no single shared invention date."
        ),
        structures: r###"array"###,
        source: r###"https://xlinux.nist.gov/dads/HTML/interval.html"###,
    },
    Lesson {
        id: r###"greedy"###,
        title: r###"Greedy"###,
        x: -2.0,
        y: 7.0,
        why: r###"Make a locally best choice when a proof shows it preserves a global optimum."###,
        how: concat!(
            "Choose according to a rule, commit, and repeat. Explain an ",
            "exchange argument or another correctness proof."
        ),
        when: concat!(
            "Use interval scheduling and suitable graph algorithms. A ",
            "plausible local rule alone is insufficient."
        ),
        industry: r###"Selecting nonoverlapping appointments and resource scheduling."###,
        history: concat!(
            "Greedy algorithms are a family with separate histories. The ",
            "interval scheduling rule used here is a standard example."
        ),
        structures: r###"heap"###,
        source: r###"https://xlinux.nist.gov/dads/HTML/greedyalgo.html"###,
    },
    Lesson {
        id: r###"advanced"###,
        title: r###"Advanced Graphs"###,
        x: 0.0,
        y: 7.0,
        why: r###"Handle weighted paths and connectivity efficiently."###,
        how: concat!(
            "Dijkstra repeatedly finalizes the smallest tentative distance for ",
            "nonnegative weights. Union-find merges components."
        ),
        when: concat!(
            "Use nonnegative shortest paths and dynamic connectivity. Negative ",
            "edges require another shortest-path method."
        ),
        industry: r###"Routing, dependency analysis and building minimum-cost networks."###,
        history: concat!(
            "Dijkstra published his shortest-path algorithm in 1959. ",
            "Union-find has its own development history."
        ),
        structures: r###"dsu,graph,heap"###,
        source: r###"https://doi.org/10.1007/BF01386390"###,
    },
    Lesson {
        id: r###"dp2"###,
        title: r###"2-D Dynamic Programming"###,
        x: 2.0,
        y: 7.0,
        why: r###"Track problems whose state depends on two coordinates."###,
        how: r###"Fill a table so each cell reads only already-computed predecessor states."###,
        when: concat!(
            "Use grid paths and comparing sequences. Show base cases and ",
            "transitions before optimizing memory."
        ),
        industry: r###"Sequence alignment, text comparison and grid-based planning."###,
        history: concat!(
            "Two-dimensional DP uses Bellman’s general framework; it is not a ",
            "separate invention."
        ),
        structures: r###"array"###,
        source: r###"https://pubmed.ncbi.nlm.nih.gov/16589462/"###,
    },
    Lesson {
        id: r###"bits"###,
        title: r###"Bit Manipulation"###,
        x: 4.0,
        y: 7.0,
        why: r###"Represent many independent boolean states compactly."###,
        how: r###"Use AND to test, OR to set, XOR to toggle, and shifts to move bit positions."###,
        when: concat!(
            "Use flags and compact subsets. Fixed-width integer rules and ",
            "overflow behavior matter."
        ),
        industry: r###"Permission masks, protocol flags and compact state sets."###,
        history: concat!(
            "Bit operations grow from binary arithmetic and Boolean algebra; ",
            "no single computer-era invention date applies."
        ),
        structures: r###"array"###,
        source: r###"https://doc.rust-lang.org/std/primitive.u32.html"###,
    },
    Lesson {
        id: r###"math"###,
        title: r###"Math & Geometry"###,
        x: 3.0,
        y: 8.0,
        why: r###"Turn numerical and spatial questions into precise invariants."###,
        how: concat!(
            "Use arithmetic identities, coordinates and geometric ",
            "relationships. Euclid repeatedly replaces (a,b) by (b,a mod b)."
        ),
        when: concat!(
            "Use exact integers where possible. Geometry using floating point ",
            "needs explicit tolerance policies."
        ),
        industry: r###"Grid coordinates, graphics, simulation and reducing ratios."###,
        history: concat!(
            "Euclid’s greatest-common-divisor method dates to ancient ",
            "mathematics. Modern computational geometry contains many later ",
            "methods."
        ),
        structures: r###"array"###,
        source: r###"https://xlinux.nist.gov/dads/HTML/euclidGcd.html"###,
    },
];
pub const LINKS: &[(usize, usize)] = &[
    (0, 1),
    (0, 2),
    (1, 3),
    (1, 4),
    (1, 5),
    (3, 6),
    (5, 6),
    (6, 7),
    (6, 8),
    (6, 9),
    (9, 10),
    (9, 11),
    (8, 12),
    (8, 13),
    (8, 14),
    (10, 14),
    (10, 15),
    (11, 15),
    (11, 16),
    (15, 17),
    (16, 17),
];
