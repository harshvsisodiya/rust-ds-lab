# Rust DS Lab

**Understand the rule. Predict the next state. Watch it happen.**

An offline native desktop learning app written in Rust. Follow the 18-topic NeetCode-style roadmap, experiment with 16 core data structures, and explore 13 additional algorithms and balanced trees with computed walkthroughs. The app uses egui/eframe with an OpenGL renderer: no browser, web server, HTML, CSS, JavaScript, Python runtime, or downloaded assets at runtime. Python examples remain embedded, readable reference text.

## Download one executable

Open [Releases](https://github.com/harshvsisodiya/rust-ds-lab/releases) and download **one asset matching your OS and CPU**. Repository access is required while the repository is private. Builds appear only after the native build workflow succeeds.

| Computer | File |
|---|---|
| Linux desktop, Intel/AMD 64-bit | `rust-ds-lab-linux-x86_64` |
| Windows, Intel/AMD 64-bit | `rust-ds-lab-windows-x86_64.exe` |
| macOS, Apple Silicon | `rust-ds-lab-macos-arm64` |
| macOS, Intel | `rust-ds-lab-macos-x86_64` |

Windows: open the `.exe`. Linux/macOS: grant executable permission, then launch the downloaded file, for example:

```sh
chmod +x rust-ds-lab-linux-x86_64
./rust-ds-lab-linux-x86_64
```

A single native executable cannot run unchanged across Windows, Linux and macOS. Each download contains the same Rust application compiled for that platform. No Rust installation is needed to run it. Normal OS windowing and graphics libraries are still required: Linux needs a graphical X11/Wayland session, OpenGL, xkbcommon and glibc 2.35 or newer. Builds use Ubuntu 22.04, Windows Server 2022 and the listed macOS runners; this is not a promise of support for every OS version, GPU or CPU architecture. Mobile, 32-bit and Linux/Windows ARM builds are not included. Downloads are not commercially signed or Apple-notarized; the OS may show its usual downloaded-app warning. `SHA256SUMS.txt` accompanies releases for integrity checking.

## A calmer learning workspace

The native interface uses the supplied cream, beige, taupe, sage and pale-blue palette. Embedded Inter typography, clearer cream/paper contrast, compact lesson sections, curved roadmap connections and clear labels and brief 220 ms state transitions help explain changes. **Reduce motion** disables transitions. The first launch opens a four-step tour; **Take the tour** reopens it anytime.

1. **Start here** offers your next unfinished roadmap lesson and your most recently completed topic.
2. **Lesson library** groups guided lessons, core structures, and algorithms/balanced trees. **Ctrl/Cmd+K** searches across all 47 topics from anywhere.
3. **Understand** explains why, how, when, industry uses and historical context. Guided lessons include three computed snapshots; structure lessons show their actual layout, invariant and challenge.
4. **Visual walkthrough / Playground** offers Back, Next, Play, Reset and adjustable playback speed. Structure labs also accept custom inputs and edits.
5. **Practice** offers 10 worked variations per topic. Write a prediction, reveal the explanation, step through the solution, and compare the calculated answer. Rust examples are the default in Code tabs; core structures retain a Python reference option. The 28 standalone Rust examples are executed by the test suite. Extra algorithm tabs show their actual project implementation, including trace helpers.
6. Mark **Understand**, **Visualize** and **Practice** checkpoints. Three checked stages finish a topic. Roadmap cards and library cards show each stage. These are self-assessed checkpoints, not automated grading or a claim that every exercise was solved.

Checkpoints exist separately for all **47 topics**: 18 roadmap lessons, 16 core structure labs and 13 extra algorithms/balanced trees. Existing v0.3 checkpoints are preserved; added topics begin unchecked. The home page suggests the next incomplete roadmap topic; a library search reaches any partially completed structure lab. Progress, the most recent completion, tour completion and reduced-motion preference persist locally. The current page, tab, exercise, custom input, prediction, playback position/speed, scroll position and text size also persist. Reopening resumes paused. Saves are debounced and flushed on normal exit; abrupt termination can lose the last unsaved fraction of a second. Back/forward page history lasts for the current app session. There is no account, telemetry or cloud synchronization.

Local progress file: Linux `$XDG_CONFIG_HOME/rust-ds-lab/progress.txt` (or `~/.config/rust-ds-lab/progress.txt`), macOS `~/Library/Application Support/rust-ds-lab/progress.txt`, Windows `%APPDATA%/rust-ds-lab/progress.txt`. A failed save produces an in-app message. Version 0.2 progress was session-only and cannot be recovered after closing that version.

Palette: black `#000000`, charcoal `#73787C`, gray `#C5C6C7`, pale blue `#D7E5F0`, beige `#C9AD93`, taupe `#554940`, sage `#879A77`. Derived cream `#EFEBE2`, paper `#FFFEFB`, muted text `#505559`, and error accent `#D97B5E` with darker error text improve readability. Blue marks active nodes; labels and explanatory notes communicate meaning alongside color. Illustrative complexity curves compare common growth rates and are not benchmark measurements.

## Coverage

The graph follows all **18 topics in the supplied NeetCode roadmap**: Arrays & Hashing, Two Pointers, Stack, Binary Search, Sliding Window, Linked List, Trees, Tries, Heap / Priority Queue, Backtracking, Graphs, 1-D Dynamic Programming, Intervals, Greedy, Advanced Graphs, 2-D Dynamic Programming, Bit Manipulation, and Math & Geometry. Several are algorithm techniques rather than data structures. [NeetCode](https://neetcode.io/roadmap) inspired the prerequisite graph; lesson prose, implementations and exercise inputs here are original and are not a copy of its paid question bank.

| Structure lab | Operations and visible relationships |
|---|---|
| Dynamic array | Insert/remove/set; indexed slots |
| Stack | Push/pop; top |
| Queue | Enqueue/dequeue; front/back |
| Deque | Push/pop both ends |
| Singly linked list | Positional edits; successor links |
| Doubly linked list | Positional edits; both link directions |
| Circular linked list | Positional edits; tail-to-head link |
| Hash map | Insert/update/remove; buckets and collisions |
| Hash set | Add/remove; duplicate membership |
| Binary search tree | Insert; ordered subtrees, ignored duplicates |
| Min heap | Insert/extract minimum; parent rule, backing array |
| Trie | Insert/remove words; shared prefixes, terminal markers |
| Undirected graph | Add edges; adjacency relationships |
| Disjoint set | Union; representatives and parent forest |
| Fenwick tree | Set values; update links, partial and prefix sums |
| Segment tree | Set values; interval hierarchy and subtree sums |

“All data structures” has no finite universal list. This release implements the 16 core structures, all 18 requested roadmap categories and the 13 additions below. Skip lists, persistent structures and probabilistic filters are not implemented. The current Math & Geometry trace covers Euclid's GCD; it is not a geometry curriculum. Advanced Graphs demonstrates nonnegative-weight Dijkstra; it is not a complete advanced graph algorithms course.

| Additional lab | Computed states |
|---|---|
| Bubble, insertion, selection, merge, quick and heap sort | Comparisons, swaps, partitions or merges, with animated bars |
| AVL tree | Insertion, heights, balance factors and rotations |
| Red-black tree | Left-leaning insertion, rotations, color flips and root recoloring |
| B-tree | Minimum-degree-2 insertion, child splits and promoted keys |
| LRU cache | Capacity-three accesses, hits, eviction and recency order |
| Monotonic stack | Strict next-greater values, pushes and pops |
| Topological sort | Kahn's ready queue, indegrees, order and cycle detection |
| KMP string matching | Prefix table, fallback and all matches including overlaps |

Balanced-tree labs cover **insertion**, not deletion. Rotation/split snapshots may show the affected subtree; explanatory text labels those local states. LRU uses a tiny sequence to make recency visible, so its teaching operations cost O(capacity); production hash-map/list implementations can achieve expected O(1). KMP accepts visible ASCII text up to 48 characters and a pattern of 1–16 characters using `text|pattern`. Directed edges use `A-B,B-C`; self-loop inputs are rejected, and multi-node cycles are demonstrated.

## Navigation and resume

| Shortcut | Action |
|---|---|
| Ctrl/Cmd+K | Search all topics |
| Left / Right | Previous / next visualization step |
| Space | Play / pause |
| Ctrl/Cmd+R | Reset playback |
| Alt+Left / Alt+Right | Back / forward through pages |
| Escape | Close search and pause |

Playback shortcuts leave text fields alone. Controls stay above the scrolling lesson content, and Text size adjusts the entire interface. The four-step tour can be reopened from the sidebar. Scroll-wheel movement uses one vertical page scroller; large diagrams pan horizontally.

## Questions and visualization detail

There are **10 worked cases per roadmap topic** (180), **10 per structure** (160), and **10 per additional lab** (130), with 3 easy, 4 medium and 3 hard cases in each set. These are structured variations on a core algorithm or operation sequence, **not 470 different interview problems**. Difficulty is relative to the lesson; harder cases add longer/mixed inputs, duplicate reasoning, invariant proofs and cost analysis.

Roadmap traces compute frequency counts, two-pointer searches, bracket stacks, binary search intervals, rolling sums, list reversal, BST/heap insertions, trie paths, subset recursion, BFS, stairs DP, interval merging, greedy scheduling, Dijkstra, grid-path DP, bit counting and Euclid's algorithm. Answers are calculated by Rust, not prerecorded pictures. Tree/structure exercises preserve their native geometry.

**Granularity:** structure replay shows each complete insertion or operation. It does not animate every heap swap or allocation. Algorithm traces show the selected algorithm's comparisons, queue visits, recursive states or DP updates. The explanations label that boundary. A mutation during partial replay branches from the visible prefix and discards its later items.

## Efficiency and model boundaries

- Native vector shapes and embedded fonts; no video, browser engine or network fetching.
- Event-driven repainting. While idle, the app does not request continuous redraws. Timed repainting runs during playback and brief state transitions; window-system events and UI animations may still cause frames.
- Only the selected exercise is generated and retained. Walkthrough diagrams are precomputed when an exercise changes, and structure diagrams are cached until their input or replay step changes. Off-screen diagrams skip painting; code listings virtualize their visible rows. The dotted background is drawn only in the visible region. Inputs and teaching trace sizes are bounded.
- Release builds use size optimization, thin LTO, one codegen unit and stripped symbols. No multisampling. This reduces work but is not a measured CPU/GPU guarantee on every computer.
- Linked diagrams model logical links with indexed sequences, not raw memory addresses. Hashing uses eight teaching buckets, not Rust's or Python's production layout. Editing range structures rebuilds a small teaching diagram; listed costs describe standard data structure operations, not rendering or model reconstruction.

Numbers accept -9999..9999. Inputs use comma-separated numbers, `cat:7,dog:12` for a map, `cat,car` for a trie, and `A-B,B-C` for a graph/disjoint set. Blank input is valid. Words are 1–8 lowercase ASCII letters; other labels are 1–12 ASCII letters/digits/underscores. Empty comma fields are ignored. Duplicate entries count toward the 16-input limit even if the logical structure deduplicates them.

See [PERFORMANCE.md](docs/PERFORMANCE.md) for the reproducible scrolling profile and its limits.

## Build from source

Install stable [Rust](https://rustup.rs) and the normal compiler/linker for your OS. On Debian/Ubuntu, install `build-essential pkg-config libxkbcommon-dev libgl1-mesa-dev`. Windows uses the Visual Studio C++ build tools; macOS uses Xcode command-line tools.

```sh
git clone https://github.com/harshvsisodiya/rust-ds-lab.git
cd rust-ds-lab
cargo run --release --locked
```

The first source build downloads the crates in `Cargo.lock`. Subsequent builds can use `--offline` after dependencies have been fetched. The executable itself runs offline. App source and logic are Rust; eframe uses platform APIs and the OS graphics driver, so “Rust application” does not mean every dependency in the OS is implemented in Rust.

```sh
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
cargo build --release --locked
```

Tests cover data structure invariants and edits, malformed/oversized input, trace bounds, independently known answers, checkpoint persistence/replacement, actual button navigation, idle repaint behavior, and rendering each native view at two viewport sizes without a display. Native builds run those checks for each target. These tests do not replace a manual GPU/window-system compatibility test on every supported computer.

## Source map

- `model.rs`: validation, deterministic structures, operations and layout geometry.
- `catalog.rs` / `background.rs`: structure references, examples and context.
- `curriculum.rs` / `guides.rs`: roadmap lessons, analogies, numbered procedures and prerequisite edges.
- `practice.rs`: executable worked traces and calculated answers.
- `desktop.rs` / `desktop/`: page composition plus separate navigation, playback, study, editor, storage and scene state; drawing helpers and native regression tests.
- `theme.rs` / `main.rs`: palette, embedded Inter fonts and window entry point.
- `extras/`: sorting, balanced-tree and sequence/graph algorithms with computed traces.
- `reference.rs` / `examples/reference/`: standalone, tested Rust examples.
- `progress.rs`: local checkpoints and versioned persistence.
- `.github/workflows/ci.yml`: four native builds, checksums and versioned GitHub releases. Existing version assets are preserved; bump `Cargo.toml` to publish a new release.

See [LEARNING.md](docs/LEARNING.md) for drills and [RELEASE.md](docs/RELEASE.md) for this release's boundaries. MIT licensed; see [LICENSE](LICENSE).

Inter is embedded under the SIL Open Font License; see [INTER-LICENSE.txt](docs/INTER-LICENSE.txt). The navigation-symbol font is a DejaVu Sans subset; see [FONT-LICENSE.txt](docs/FONT-LICENSE.txt). Both use `include_bytes!`, not byte arrays in source. All fonts and shapes are embedded, with no runtime downloads.
