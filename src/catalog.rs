//! Learning content is embedded text. Python is never executed by the app.
pub struct Topic {
    pub id: &'static str,
    pub title: &'static str,
    pub group: &'static str,
    pub example: &'static str,
    pub about: &'static str,
    pub cost: &'static str,
    pub invariant: &'static str,
    pub challenge: &'static str,
    pub answer: &'static str,
    pub python: &'static str,
}
pub const TOPICS: &[Topic] = &[
    Topic {
        id: r###"array"###,
        title: r###"Dynamic array"###,
        group: r###"Linear"###,
        example: r###"12,7,24,9,18"###,
        about: r###"Contiguous indexed slots. Insert or remove at an index to see later \
                        elements shift."###,
        cost: r###"Access O(1); search O(n); insert/delete O(n); append amortized O(1)."###,
        invariant: r###"Indices are consecutive, from zero to length minus one."###,
        challenge: r###"Start with [12,7,24,9]. Insert 5 at index 1, then remove index 3. What remains?"###,
        answer: r###"[12,5,7,9]. Insertion shifted 7,24,9 right; removing index 3 removed 24."###,
        python: r###"a = [12, 7, 24, 9]
a.insert(1, 5)  # shift later elements right
removed = a.pop(3)
assert a == [12, 5, 7, 9]
print(a, removed)"###,
    }
,
    Topic {
        id: r###"stack"###,
        title: r###"Stack"###,
        group: r###"Linear"###,
        example: r###"12,7,24,9"###,
        about: r###"Last in, first out. Push adds to the top; pop removes the newest value."###,
        cost: r###"Push amortized O(1); pop O(1); peek O(1)."###,
        invariant: r###"Only the top is used for stack insertion and removal."###,
        challenge: r###"Push 4, push 8, pop, push 2. Predict the top and the next two pops."###,
        answer: r###"Top is 2. The next pops are 2, then 4."###,
        python: "stack = []\nstack.append(4)\nstack.append(8)\nassert stack.pop() == 8\nstack.append(2)\nassert stack[-1] \
                        == 2\nprint(stack.pop(), stack.pop())",

    }
,

    Topic {

        id: r###"queue"###,

        title: r###"Queue"###,

        group: r###"Linear"###,

        example: r###"12,7,24,9"###,

        about: r###"First in, first out. Enqueue at the rear; dequeue from the front."###,

        cost: r###"Enqueue, dequeue, and peek O(1) with a deque."###,

        invariant: r###"The oldest waiting element leaves first."###,

        challenge: r###"Enqueue 4,8,2, then dequeue twice. Which value remains?"###,

        answer: r###"2 remains. Dequeues returned 4 and 8."###,

        python: "from collections import deque\nqueue = deque()\nfor value in [4, 8, 2]:\n    queue.append(value)\nassert \
                        queue.popleft() == 4\nassert queue.popleft() == 8\nprint(queue[0])",

    }
,

    Topic {

        id: r###"deque"###,

        title: r###"Deque"###,

        group: r###"Linear"###,

        example: r###"12,7,24,9"###,

        about: r###"A double-ended queue supports insertion and removal at either end."###,

        cost: r###"Push/pop at either end O(1); middle access O(n) in Python deque."###,

        invariant: r###"Front and back are both valid entry and exit points."###,

        challenge: r###"Start [4,8]. Push 2 at front, push 6 at back, then pop back. Predict the deque."###,

        answer: r###"[2,4,8]. The back pop returns 6."###,

        python: r###"from collections import deque
d = deque([4, 8])
d.appendleft(2)
d.append(6)
assert d.pop() == 6
print(list(d))"###,

    }
,

    Topic {

        id: r###"singly"###,

        title: r###"Singly linked list"###,

        group: r###"Linked"###,

        example: r###"12,7,24,9"###,

        about: r###"Each node links to its successor. The displayed indices identify positions, not memory addresses."###,

        cost: r###"Search/index O(n); insertion after a known node O(1)."###,

        invariant: r###"The tail points to null; every earlier node points to its successor."###,

        challenge: r###"Why is inserting after a known node O(1), but inserting at index k O(n)?"###,

        answer: r###"Rewiring a known link is constant work. Locating the kth node requires walking links."###,

        python: "class Node:\n    def __init__(self, value, next=None):\n        self.value, self.next = value, \
                        next\n\nhead = Node(4, Node(8))\nhead = Node(2, head)  # insert at head: O(1)\nhead.next = \
                        head.next.next  # remove node after head\nnode = head\nwhile node:\n    print(node.value)\n    node = \
                        node.next",

    }
,

    Topic {

        id: r###"doubly"###,

        title: r###"Doubly linked list"###,

        group: r###"Linked"###,

        example: r###"12,7,24,9"###,

        about: r###"Each node has next and previous links, enabling travel in both directions."###,

        cost: r###"Search O(n); insert/delete a known node O(1)."###,

        invariant: r###"If A.next is B, then B.prev is A."###,

        challenge: r###"When removing a middle node X, which neighboring links must change?"###,

        answer: r###"Set X.prev.next = X.next and X.next.prev = X.prev. Handle head/tail separately."###,

        python: "class Node:\n    def __init__(self, value):\n        self.value = value\n        self.prev = self.next = \
                        None\n\na, b, c = (Node(v) for v in [4, 8, 2])\na.next, b.prev = b, a\nb.next, c.prev = c, b\n# Remove \
                        the middle node b.\na.next, c.prev = c, a\nassert a.next.prev is a\nprint(a.value, a.next.value)",

    }
,

    Topic {

        id: r###"circular"###,

        title: r###"Circular linked list"###,

        group: r###"Linked"###,

        example: r###"12,7,24,9"###,

        about: r###"The last node links back to the head. A traversal stops when it returns to its starting node."###,

        cost: r###"Search O(n); insert after a known node O(1)."###,

        invariant: r###"A nonempty list has no null next link."###,

        challenge: r###"Why does a loop that stops only at null fail on a circular list?"###,

        answer: r###"It never reaches null. Stop when the traversal reaches the starting node again."###,

        python: "class Node:\n    def __init__(self, value):\n        self.value, self.next = value, None\n\na, b, c = \
                        (Node(v) for v in [4, 8, 2])\na.next, b.next, c.next = b, c, a\nnode = a\nwhile True:\n    \
                        print(node.value)\n    node = node.next\n    if node is a:\n        break",

    }
,

    Topic {

        id: r###"hashmap"###,

        title: r###"Hash map"###,

        group: r###"Hashing"###,

        example: r###"cat:7,dog:12,ant:4,cat:9"###,

        about: r###"Keys map to values. Repeating a key updates its value. This diagram uses eight buckets and separate chaining."###,

        cost: r###"Average lookup/insert/delete O(1); worst O(n)."###,

        invariant: r###"Each key appears once. Colliding keys coexist in a bucket."###,

        challenge: r###"Insert cat:7, dog:12, cat:9. How many keys exist and what is cat worth?"###,

        answer: r###"Two keys; cat maps to 9. Updating a value does not add a new key."###,

        python: "scores = {}\nfor key, value in [('cat', 7), ('dog', 12), ('cat', 9)]:\n    scores[key] = value\nassert \
                        len(scores) == 2\nassert scores['cat'] == 9\nscores.pop('dog')\nprint(scores)\n# Python dict uses its \
                        own hash implementation;\n# the visualizer uses a simple teaching hash.",

    }
,

    Topic {

        id: r###"hashset"###,

        title: r###"Hash set"###,

        group: r###"Hashing"###,

        example: r###"cat,dog,ant,cat"###,

        about: r###"A collection of unique keys. Duplicate insertion leaves membership unchanged."###,

        cost: r###"Average membership/insert/delete O(1); worst O(n)."###,

        invariant: r###"Each key occurs at most once."###,

        challenge: r###"Insert cat twice and dog once; remove cat once. What remains?"###,

        answer: r###"Only dog. A set tracks membership, not counts."###,

        python: r###"seen = set()
for key in ['cat', 'cat', 'dog']:
    seen.add(key)
assert len(seen) == 2
seen.discard('cat')
print(seen)"###,

    }
,

    Topic {

        id: r###"bst"###,

        title: r###"Binary search tree"###,

        group: r###"Trees"###,

        example: r###"12,7,24,9,18,30,3"###,

        about: r###"Smaller keys go left, larger keys go right. This is an unbalanced BST; duplicate keys are ignored."###,

        cost: r###"Search/insert O(h); average O(log n), worst O(n)."###,

        invariant: r###"Every left-subtree key is smaller and every right-subtree key larger."###,

        challenge: r###"Insert 1,2,3,4,5 in order. What shape forms, and how expensive is finding 5?"###,

        answer: r###"A rightward chain. Finding 5 visits all five nodes; worst-case O(n)."###,

        python: "class Node:\n    def __init__(self, value):\n        self.value = value\n        self.left = self.right \
                        = None\n\ndef insert(root, value):\n    if root is None:\n        return Node(value)\n    if value < \
                        root.value:\n        root.left = insert(root.left, value)\n    elif value > root.value:\n        \
                        root.right = insert(root.right, value)\n    return root\n\nroot = None\nfor value in [12, 7, 24, 9, \
                        18]:\n    root = insert(root, value)\nassert root.left.right.value == 9",

    }
,

    Topic {

        id: r###"heap"###,

        title: r###"Min heap"###,

        group: r###"Trees"###,

        example: r###"12,7,24,9,18,3"###,

        about: r###"A complete binary tree stored in an array. Inserting a value sifts it upward; removing the root sifts its replacement downward."###,

        cost: r###"Peek O(1); push/pop O(log n)."###,

        invariant: r###"Each parent is less than or equal to its children."###,

        challenge: r###"Is a heap array globally sorted? Explain using [2,7,4,9,8]."###,

        answer: r###"No. Each parent only needs to precede its children: 2 <= 7,4 and 7 <= 9,8."###,

        python: "import heapq\nheap = []\nfor value in [12, 7, 24, 9, 18, 3]:\n    heapq.heappush(heap, value)\nassert \
                        heap[0] == 3\nassert heapq.heappop(heap) == 3\nprint(heap)",

    }
,

    Topic {

        id: r###"trie"###,

        title: r###"Trie"###,

        group: r###"Trees"###,

        example: r###"cat,car,cap,dog"###,

        about: r###"Words share prefixes. A filled terminal node marks a complete word, which may also be a prefix of another word."###,

        cost: r###"Insert/search O(L), where L is the word length."###,

        invariant: r###"A root-to-terminal path spells a stored word."###,

        challenge: r###"After inserting car and cart, why must the r node be terminal even though it has a child?"###,

        answer: r###"Because car is itself a word. Children describe longer words; terminal status describes the current prefix."###,

        python: "root = {}\nEND = '$'\ndef insert(word):\n    node = root\n    for letter in word:\n        node = \
                        node.setdefault(letter, {})\n    node[END] = True\n\ndef contains(word):\n    node = root\n    for \
                        letter in word:\n        if letter not in node:\n            return False\n        node = node[letter]\n \
                           return END in node\n\nfor word in ['car', 'cart', 'cat']:\n    insert(word)\nassert contains('car') \
                        and not contains('ca')",

    }
,

    Topic {

        id: r###"graph"###,

        title: r###"Undirected graph"###,

        group: r###"Networks"###,

        example: r###"A-B,A-C,B-D,C-D,D-E"###,

        about: r###"An undirected simple graph, shown as nodes and edges plus an adjacency list. Duplicate edges are ignored."###,

        cost: r###"Adjacency-list storage O(V+E); neighbor iteration O(degree)."###,

        invariant: r###"An undirected edge A-B appears in both neighbor lists."###,

        challenge: r###"Edges A-B, B-C, A-C form what? What is the degree of B?"###,

        answer: r###"A triangle (a cycle of length 3). Vertex B has degree 2."###,

        python: "graph = {}\ndef add_edge(a, b):\n    graph.setdefault(a, set()).add(b)\n    graph.setdefault(b, \
                        set()).add(a)\n\nfor a, b in [('A', 'B'), ('A', 'C'), ('B', 'C')]:\n    add_edge(a, b)\nassert \
                        graph['B'] == {'A', 'C'}\nfor node in sorted(graph):\n    print(node, sorted(graph[node]))",

    }
,

    Topic {

        id: r###"dsu"###,

        title: r###"Disjoint set"###,

        group: r###"Networks"###,

        example: r###"A-B,C-D,B-C,E-F"###,

        about: r###"Union-find stores connected groups as parent trees. Union by size attaches the smaller group under the larger one."###,

        cost: r###"Union/find amortized O(alpha(n)) with path compression and union by size."###,

        invariant: r###"Every root is its own parent; each item has exactly one representative."###,

        challenge: r###"Union A-B, C-D, B-C. Are A and D in the same set?"###,

        answer: r###"Yes. The third union connects the two earlier groups."###,

        python: "parent, size = {}, {}\ndef find(x):\n    parent.setdefault(x, x)\n    size.setdefault(x, 1)\n    if \
                        parent[x] != x:\n        parent[x] = find(parent[x])\n    return parent[x]\n\ndef union(a, b):\n    a, b \
                        = find(a), find(b)\n    if a == b:\n        return\n    if size[a] < size[b]:\n        a, b = b, a\n    \
                        parent[b] = a\n    size[a] += size[b]\n\nfor a, b in [('A', 'B'), ('C', 'D'), ('B', 'C')]:\n    union(a, \
                        b)\nassert find('A') == find('D')",

    }
,

    Topic {

        id: r###"fenwick"###,

        title: r###"Fenwick tree"###,

        group: r###"Range queries"###,

        example: r###"2,5,1,4,3,7,2,6"###,

        about: r###"A binary indexed tree stores partial sums. Cell i covers indices i-lowbit(i)+1 through i, using one-based indices."###,

        cost: r###"Point add and prefix sum O(log n); storage O(n)."###,

        invariant: r###"tree[i] stores the sum of the lowbit(i) elements ending at i."###,

        challenge: r###"For [2,5,1,4], what do cells 2 and 4 store?"###,

        answer: r###"Cell 2 stores 2+5=7. Cell 4 stores 2+5+1+4=12."###,

        python: "values = [2, 5, 1, 4]\nbit = [0] * (len(values) + 1)\ndef add(i, delta):\n    while i < len(bit):\n      \
                          bit[i] += delta\n        i += i & -i\n\ndef prefix(i):\n    total = 0\n    while i > 0:\n        total \
                        += bit[i]\n        i -= i & -i\n    return total\n\nfor i, value in enumerate(values, 1):\n    add(i, \
                        value)\nassert prefix(3) == 8\nprint(bit[1:])",

    }
,

    Topic {

        id: r###"segment"###,

        title: r###"Segment tree"###,

        group: r###"Range queries"###,

        example: r###"2,5,1,4,3,7,2,6"###,

        about: r###"Each node stores a range sum. Its children partition that range. Leaves are individual values."###,

        cost: r###"Point update and range sum O(log n); build/storage O(n)."###,

        invariant: r###"A parent sum equals the sum of its two children."###,

        challenge: r###"For [2,5,1,4], what is the inclusive range sum from index 1 to 3?"###,

        answer: r###"5+1+4=10. Indices here are zero-based."###,

        python: "values = [2, 5, 1, 4]\nn = len(values)\ntree = [0] * n + values\nfor i in range(n - 1, 0, -1):\n    \
                        tree[i] = tree[2*i] + tree[2*i+1]\n\ndef range_sum(left, right):  # half-open [left, right)\n    left, \
                        right = left + n, right + n\n    total = 0\n    while left < right:\n        if left & 1:\n            \
                        total += tree[left]; left += 1\n        if right & 1:\n            right -= 1; total += tree[right]\n    \
                            left //= 2; right //= 2\n    return total\n\nassert range_sum(1, 4) == 10",
    }
,
];
pub fn topic(id: &str) -> &'static Topic {
    TOPICS.iter().find(|t| t.id == id).unwrap_or(&TOPICS[0])
}
