# Rust DS Lab 0.4.0 — clearer, quicker, easier to resume

- Refined creamy theme with embedded Inter typography, stronger contrast, distinct paper surfaces and an explicit error color. Animated sorting bars make swaps and ordering easier to follow.
- Less work while scrolling: cached walkthrough diagrams, off-screen paint culling, visible-region grids and virtualized code rows. Diagrams no longer compete with the page for vertical scrolling. Playback controls stay accessible above lesson content.
- Ctrl/Cmd+K topic search, Left/Right/Space playback shortcuts, Alt+Left/Right page history, text-size control and session resume. Reopening restores the current exercise, custom input, prediction, step, speed and scroll position, paused.
- Six sorting labs: bubble, insertion, selection, merge, quick and heap sort. AVL, left-leaning red-black and minimum-degree-2 B-tree insertion. LRU cache, monotonic stack, Kahn topological sort and KMP string matching. Each has explanations, context, computed traces, ten variations and checkpoints.
- Rust code tabs and 28 standalone examples exercised by tests; Python references remain available for the 16 core structures.
- Safe direct model construction: malformed pairs return an error diagram or a `Result` error instead of panicking. Fonts use embedded asset files. The former monolithic app state is separated into navigation, study, playback, editor, storage and rendering state.

Download one executable matching your OS and CPU: Linux x86_64, Windows x86_64, macOS Apple Silicon or macOS Intel. No browser, Rust installation or Python runtime is required. Linux needs a graphical desktop, OpenGL/xkbcommon and glibc 2.35+. Downloads are unsigned and not Apple-notarized. SHA256SUMS.txt accompanies the four executables.

The 47 learning units contain 470 variations of core methods, not 470 unrelated interview problems. Checkpoints are self-assessed and existing v0.3 progress migrates automatically. Balanced-tree labs cover insertion only. KMP uses bounded ASCII input; the tiny LRU teaching sequence is O(capacity), distinct from a production O(1) hash-map/list cache. Performance changes reduce UI work but are not a guarantee for every graphics driver. See README for precise coverage and input limits.
