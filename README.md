# Rust DS Lab

**Understand the rule. Predict the next state. Watch it happen.**

An offline native desktop learning app written in Rust. Follow the 18-topic NeetCode-style roadmap, experiment with 16 core data structures, and step through worked solutions. The app uses egui/eframe with an OpenGL renderer: no browser, web server, HTML, CSS, JavaScript, Python runtime, or downloaded assets at runtime. Python examples remain embedded, readable reference text.

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

The native interface uses the supplied cream, beige, taupe, sage and pale-blue palette. Spacious cards, curved roadmap connections, clear labels and brief 280 ms state transitions help explain changes. **Reduce motion** disables transitions. The first launch opens a four-step tour; **Take the tour** reopens it anytime.

1. **Start here** offers your next unfinished roadmap lesson and your most recently completed topic.
2. **Lesson library** searches all guided topics and structure labs by name, concept or use.
3. **Understand** explains why, how, when, industry uses and historical context. Guided lessons include three computed snapshots; structure lessons show their actual layout, invariant and challenge.
4. **Visual walkthrough / Playground** offers Back, Next, Play, Reset and adjustable playback speed. Structure labs also accept custom inputs and edits.
5. **Practice** offers 10 worked variations per topic. Write a prediction, reveal the explanation, step through the solution, and compare the calculated answer. Python reference examples live in their own structure tab and can be copied.
6. Mark **Understand**, **Visualize** and **Practice** checkpoints. Three checked stages finish a topic. Roadmap cards and library cards show each stage. These are self-assessed checkpoints, not automated grading or a claim that every exercise was solved.

Checkpoints exist separately for all 18 roadmap topics and all 16 structure labs. The home page suggests the next incomplete roadmap topic; a library search reaches any partially completed structure lab. Progress, the most recent completion, tour completion and reduced-motion preference persist locally. Exercise inputs and predictions remain session-only. There is no account, telemetry or cloud synchronization.

Local progress file: Linux `$XDG_CONFIG_HOME/rust-ds-lab/progress.txt` (or `~/.config/rust-ds-lab/progress.txt`), macOS `~/Library/Application Support/rust-ds-lab/progress.txt`, Windows `%APPDATA%/rust-ds-lab/progress.txt`. A failed save produces an in-app message. Version 0.2 progress was session-only and cannot be recovered after closing that version.

Palette: black `#000000`, charcoal `#73787C`, gray `#C5C6C7`, pale blue `#D7E5F0`, beige `#C9AD93`, taupe `#554940`, sage `#879A77`. Derived cream `#F7F3EC`, paper `#FFFDF9` and darker text improve readability. Blue marks active nodes; labels and explanatory notes communicate meaning alongside color. Illustrative complexity curves compare common growth rates and are not benchmark measurements.

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

“All data structures” has no finite universal list. This release implements these 16 core structures and all 18 requested roadmap categories. Specialized structures such as AVL/red-black/B-trees, skip lists and persistent structures are **not implemented**. The current Math & Geometry trace covers Euclid's GCD; it is not a geometry curriculum. Advanced Graphs demonstrates nonnegative-weight Dijkstra; it is not a complete advanced graph algorithms course.

## Questions and visualization detail

There are **10 worked cases per roadmap topic** (180) and **10 per structure** (160), with 3 easy, 4 medium and 3 hard cases in each set. These are structured variations on a core algorithm or operation sequence, **not 340 different interview problems**. Difficulty is relative to the lesson; harder cases add longer/mixed inputs, duplicate reasoning, invariant proofs and cost analysis.

Roadmap traces compute frequency counts, two-pointer searches, bracket stacks, binary search intervals, rolling sums, list reversal, BST/heap insertions, trie paths, subset recursion, BFS, stairs DP, interval merging, greedy scheduling, Dijkstra, grid-path DP, bit counting and Euclid's algorithm. Answers are calculated by Rust, not prerecorded pictures. Tree/structure exercises preserve their native geometry.

**Granularity:** structure replay shows each complete insertion or operation. It does not animate every heap swap or allocation. Algorithm traces show the selected algorithm's comparisons, queue visits, recursive states or DP updates. The explanations label that boundary. A mutation during partial replay branches from the visible prefix and discards its later items.

## Efficiency and model boundaries

- Native vector shapes and embedded fonts; no video, browser engine or network fetching.
- Event-driven repainting. While idle, the app does not request continuous redraws. Timed repainting runs during playback and brief state transitions; window-system events and UI animations may still cause frames.
- Only the selected exercise is generated and retained. Structure diagrams are cached until their input or replay step changes. Inputs and teaching trace sizes are bounded.
- Release builds use size optimization, thin LTO, one codegen unit and stripped symbols. No multisampling. This reduces work but is not a measured CPU/GPU guarantee on every computer.
- Linked diagrams model logical links with indexed sequences, not raw memory addresses. Hashing uses eight teaching buckets, not Rust's or Python's production layout. Editing range structures rebuilds a small teaching diagram; listed costs describe standard data structure operations, not rendering or model reconstruction.

Numbers accept -9999..9999. Inputs use comma-separated numbers, `cat:7,dog:12` for a map, `cat,car` for a trie, and `A-B,B-C` for a graph/disjoint set. Blank input is valid. Words are 1–8 lowercase ASCII letters; other labels are 1–12 ASCII letters/digits/underscores. Empty comma fields are ignored. Duplicate entries count toward the 16-input limit even if the logical structure deduplicates them.

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
- `desktop.rs` / `theme.rs` / `main.rs`: native UI, palette and window entry point.
- `progress.rs`: local checkpoints and versioned persistence.
- `.github/workflows/ci.yml`: four native builds, checksums and versioned GitHub releases. Existing version assets are preserved; bump `Cargo.toml` to publish a new release.

See [LEARNING.md](docs/LEARNING.md) for drills and [RELEASE.md](docs/RELEASE.md) for this release's boundaries. MIT licensed; see [LICENSE](LICENSE).

The tiny embedded navigation-symbol font is a DejaVu Sans subset; see [FONT-LICENSE.txt](docs/FONT-LICENSE.txt). All fonts and shapes are embedded, with no runtime downloads.
