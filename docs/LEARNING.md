# Learn by predicting

For each structure: describe the problem it solves, draw a tiny example, predict an operation, step through it, and state the rule that remains true. Then explain its time and memory cost. Keep inputs small enough to inspect every change.

## Suggested sequence

1. **Arrays and hashing:** explain direct indexing, collisions, duplicate updates and membership. Compare a count map with a set: what information does the set discard?
2. **Linear structures:** predict stack, queue and deque output. Explain why a bracket parser needs the most recent unmatched opening bracket.
3. **Pointers and windows:** justify each discarded candidate in two-pointer/binary search. Compute a rolling total by removing one value and adding another.
4. **Linked structures:** draw predecessor/successor changes. Distinguish knowing a node from searching for its position. Explain the stopping condition for a circular traversal.
5. **Trees, tries and heaps:** contrast sorted-subtree order with heap order. Insert increasing values into the BST. Compare a word ending with a shared prefix.
6. **Graphs and backtracking:** distinguish a visited set from a recursion path. Trace BFS's queue, then Dijkstra's tentative distances. Explain why Dijkstra's trace requires nonnegative weights.
7. **Dynamic programming and ranges:** state exactly what one table entry represents. Compare a Fenwick prefix with a segment-tree interval. Decide whether touching intervals merge based on the chosen endpoint convention.
8. **Greedy, bits and mathematics:** explain why earliest-finish scheduling leaves room for later jobs. Clear the lowest set bit repeatedly. Explain why Euclid preserves the GCD.

Each roadmap and structure section has 10 worked cases: 3 easy, 4 medium, 3 hard. The cases intentionally repeat a core method with changed inputs so you can separate memorizing an output from understanding the rule. These difficulty labels are local learning labels, not LeetCode ratings.

## A useful hard-case answer

Give the result, a short invariant, a reason each step preserves it, and a termination argument. Derive time from the number of operations and space from the state retained. Name a tempting wrong approach and supply a counterexample. Do not count the visualizer's teaching snapshots as algorithm auxiliary space.

## Beyond this release

Investigate balanced search trees, B-trees, skip lists, radix/compressed tries, monotonic deques, persistent structures and probabilistic filters. Compare CPU cache locality before concluding that fewer big-O operations always means a faster real program. These are follow-on subjects; the current app does not implement them all.
