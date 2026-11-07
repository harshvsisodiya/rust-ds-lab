# Scrolling verification for v0.4

A repeatable headless CPU profile compares the released v0.3 source (`d83f341`) with v0.4 on the same Linux execution host using Rust 1.99.1, debug builds and egui 0.31.1. It opens the Binary Search lesson at 1280 × 900, simulates 540 scroll frames (alternating direction every 90 frames), discards 60 warmup frames, and measures UI layout plus tessellation. Reduced motion is enabled in both versions.

| Measurement | v0.3 | v0.4 |
|---|---:|---:|
| Median CPU frame time | 3.347 ms | 2.066 ms |
| 95th percentile CPU frame time | 5.290 ms | 4.135 ms |
| Mean generated vertices per measured frame | 7,061 | 6,381 |

The median in this sample fell by about 38%. This comparison includes changed layout and collapsed detail sections, as well as rendering improvements. It is a narrow development benchmark, not a release-build FPS claim or a GPU/driver test. Timing varies with the host. The deterministic input sequence and profiling code are in the ignored `native_scroll_profile` test:

```sh
cargo test native_scroll_profile -- --ignored --nocapture
```

Changes reducing work include caching computed walkthrough diagrams, cloning scene data only on changes, skipping off-screen diagrams, clipping the dotted grid, using horizontal-only diagram scrollers, and virtualizing long code listings. Regular tests check that idle views stop continuous repainting. Visual review uses real egui meshes and font atlases at 800 × 600 and 1280 × 900, including small-window playback and the new sorting/tree labs. Native window-system and driver behavior should still be assessed on the actual device.
