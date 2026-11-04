# Your data structure practice plan

Use this loop: **predict → perform → explain → implement**. Write an answer before revealing it. A correct guess without an explanation is not mastery.

| Session | Structures | What you should be able to explain |
|---|---|---|
| 1 | Array, stack, queue | Indexed access, shifting, LIFO versus FIFO |
| 2 | Deque and three linked lists | Both ends, pointer relationships, termination |
| 3 | Hash map and hash set | Keys, updates, duplicates, collisions |
| 4 | BST and min heap | Search ordering versus parent ordering |
| 5 | Trie | Shared prefixes, complete-word markers |
| 6 | Graph and disjoint set | Adjacency, connectivity, representatives |
| 7 | Fenwick and segment trees | Partial sums, ranges, update/query costs |

## The daily 20-minute drill

- **5 minutes:** rebuild yesterday's example without looking at the code.
- **10 minutes:** predict three operation sequences; check each in the visualizer.
- **5 minutes:** state the invariant and complexity aloud. Explain which implementation assumption makes that complexity true.

Then implement one structure yourself in Rust. The Python example is a conceptual comparison, not a requirement to write Python.

## Deeper challenges

1. **Dynamic array:** why is append amortized O(1) even though a resize copies O(n) elements? Distinguish worst-case from amortized cost.
2. **Stack:** explain how a browser history or function-call stack differs from a queue.
3. **Queue:** implement a queue with two stacks, then explain the amortized cost.
4. **Deque:** write a ring buffer. Distinguish logical positions from physical array indices.
5. **Singly linked list:** implement a real owned Rust chain using `Option<Box<Node>>`; compare it to this app's diagram model.
6. **Doubly linked list:** explain why safe Rust ownership becomes more complicated with backward links. Investigate arena indices before reaching for unsafe pointers.
7. **Circular list:** handle empty and one-node cases; show why following links until null cannot terminate.
8. **Hash map:** deliberately insert `a:1,i:2,q:3` in the lab. All collide under its teaching hash. Explain separate chaining and load factor.
9. **Hash set:** distinguish a set from a multiset. Predict what removing a duplicate should mean in each.
10. **BST:** compare insertion orders `1,2,3,4,5` and `3,1,5,2,4`. Explain why balancing is useful.
11. **Heap:** map parent and child array indices. Explain why binary searching the heap array is invalid.
12. **Trie:** insert `car,cart,cat`; remove `car`. Predict which terminal markers and shared prefixes remain.
13. **Graph:** compare adjacency lists and matrices for sparse and dense graphs. Explain why an undirected edge is stored twice in an adjacency list.
14. **Disjoint set:** implement find with path compression and union by size. Explain why this does not enumerate graph paths.
15. **Fenwick:** derive `i & -i` using binary arithmetic. Compute a range sum as the difference of two prefix sums.
16. **Segment tree:** implement a logarithmic range query and point update. Compare its flexibility with a Fenwick tree.

## Self-assessment

For each structure, record:

| Date | Structure | Prediction | Actual result | Why it differed | Repeat tomorrow? |
|---|---|---|---|---|---|
| | | | | | |

You understand a structure when you can: draw it from memory, state its invariant, predict a sequence with duplicates/empty inputs, explain the cost, and implement its core operations.

## Further reading

- Rust ownership: <https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html>
- Rust collections: <https://doc.rust-lang.org/std/collections/>
- Python data structures: <https://docs.python.org/3/tutorial/datastructures.html>
- Python deque: <https://docs.python.org/3/library/collections.html#collections.deque>
- Python heapq: <https://docs.python.org/3/library/heapq.html>

Next extensions could include AVL/red-black trees, B-trees, sparse matrices, skip lists, and persistent structures. Add them only when their invariants and behavior can be tested clearly.
