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
        about: r###"Contiguous indexed slots. Insert or remove at an index to see later elements shift."###,
        cost: r###"Access O(1); search O(n); insert/delete O(n); append amortized O(1)."###,
        invariant: r###"Indices are consecutive, from zero to length minus one."###,
        challenge: r###"Start with [12,7,24,9]. Insert 5 at index 1, then remove index 3. What remains?"###,
        answer: r###"[12,5,7,9]. Insertion shifted 7,24,9 right; removing index 3 removed 24."###,
        python: r###"a = [12, 7, 24, 9]
a.insert(1, 5)  # shift later elements right
removed = a.pop(3)
assert a == [12, 5, 7, 9]
print(a, removed)"###,
    },
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
        python: r###"stack = []
stack.append(4)
stack.append(8)
assert stack.pop() == 8
stack.append(2)
assert stack[-1] == 2
print(stack.pop(), stack.pop())"###,
    },
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
        python: r###"from collections import deque
queue = deque()
for value in [4, 8, 2]:
    queue.append(value)
assert queue.popleft() == 4
assert queue.popleft() == 8
print(queue[0])"###,
    },
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
    },
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
        python: r###"class Node:
    def __init__(self, value, next=None):
        self.value, self.next = value, next

head = Node(4, Node(8))
head = Node(2, head)  # insert at head: O(1)
head.next = head.next.next  # remove node after head
node = head
while node:
    print(node.value)
    node = node.next"###,
    },
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
        python: r###"class Node:
    def __init__(self, value):
        self.value = value
        self.prev = self.next = None

a, b, c = (Node(v) for v in [4, 8, 2])
a.next, b.prev = b, a
b.next, c.prev = c, b
# Remove the middle node b.
a.next, c.prev = c, a
assert a.next.prev is a
print(a.value, a.next.value)"###,
    },
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
        python: r###"class Node:
    def __init__(self, value):
        self.value, self.next = value, None

a, b, c = (Node(v) for v in [4, 8, 2])
a.next, b.next, c.next = b, c, a
node = a
while True:
    print(node.value)
    node = node.next
    if node is a:
        break"###,
    },
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
        python: r###"scores = {}
for key, value in [('cat', 7), ('dog', 12), ('cat', 9)]:
    scores[key] = value
assert len(scores) == 2
assert scores['cat'] == 9
scores.pop('dog')
print(scores)
# Python dict uses its own hash implementation;
# the visualizer uses a simple teaching hash."###,
    },
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
    },
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
        python: r###"class Node:
    def __init__(self, value):
        self.value = value
        self.left = self.right = None

def insert(root, value):
    if root is None:
        return Node(value)
    if value < root.value:
        root.left = insert(root.left, value)
    elif value > root.value:
        root.right = insert(root.right, value)
    return root

root = None
for value in [12, 7, 24, 9, 18]:
    root = insert(root, value)
assert root.left.right.value == 9"###,
    },
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
        python: r###"import heapq
heap = []
for value in [12, 7, 24, 9, 18, 3]:
    heapq.heappush(heap, value)
assert heap[0] == 3
assert heapq.heappop(heap) == 3
print(heap)"###,
    },
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
        python: r###"root = {}
END = '$'
def insert(word):
    node = root
    for letter in word:
        node = node.setdefault(letter, {})
    node[END] = True

def contains(word):
    node = root
    for letter in word:
        if letter not in node:
            return False
        node = node[letter]
    return END in node

for word in ['car', 'cart', 'cat']:
    insert(word)
assert contains('car') and not contains('ca')"###,
    },
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
        python: r###"graph = {}
def add_edge(a, b):
    graph.setdefault(a, set()).add(b)
    graph.setdefault(b, set()).add(a)

for a, b in [('A', 'B'), ('A', 'C'), ('B', 'C')]:
    add_edge(a, b)
assert graph['B'] == {'A', 'C'}
for node in sorted(graph):
    print(node, sorted(graph[node]))"###,
    },
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
        python: r###"parent, size = {}, {}
def find(x):
    parent.setdefault(x, x)
    size.setdefault(x, 1)
    if parent[x] != x:
        parent[x] = find(parent[x])
    return parent[x]

def union(a, b):
    a, b = find(a), find(b)
    if a == b:
        return
    if size[a] < size[b]:
        a, b = b, a
    parent[b] = a
    size[a] += size[b]

for a, b in [('A', 'B'), ('C', 'D'), ('B', 'C')]:
    union(a, b)
assert find('A') == find('D')"###,
    },
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
        python: r###"values = [2, 5, 1, 4]
bit = [0] * (len(values) + 1)
def add(i, delta):
    while i < len(bit):
        bit[i] += delta
        i += i & -i

def prefix(i):
    total = 0
    while i > 0:
        total += bit[i]
        i -= i & -i
    return total

for i, value in enumerate(values, 1):
    add(i, value)
assert prefix(3) == 8
print(bit[1:])"###,
    },
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
        python: r###"values = [2, 5, 1, 4]
n = len(values)
tree = [0] * n + values
for i in range(n - 1, 0, -1):
    tree[i] = tree[2*i] + tree[2*i+1]

def range_sum(left, right):  # half-open [left, right)
    left, right = left + n, right + n
    total = 0
    while left < right:
        if left & 1:
            total += tree[left]; left += 1
        if right & 1:
            right -= 1; total += tree[right]
        left //= 2; right //= 2
    return total

assert range_sum(1, 4) == 10"###,
    },
];

pub fn topic(id: &str) -> &'static Topic {
    TOPICS.iter().find(|t| t.id == id).unwrap_or(&TOPICS[0])
}
