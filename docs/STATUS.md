# Verified checkpoint

Project: Dreadmarch · Repository: https://github.com/Herbertofury/Dreadmarch · Branch: main

## 24 November 2026: first implementation checkpoint

The project is under active implementation. This is a playable foundation toward the full vision.

### Verified

- Public repository created and visibility read back as public.
- Rust 1.100.0 installed; current Bevy 0.20.1 dependency graph resolved.
- 16 core/simulation tests pass, with strict Clippy for both crates.
- Full battle through public commands reaches Victory at tick 4,392: all five waves, all three relics,
  80 kills, 43 allied survivors, and successful mid-battle save/load.
- Final full-battle checksum after the fixed-point edge-case correction: `43701d5395d1a649`.
- Native client passes current Bevy compilation checking; full binary and GPU smoke are underway.
- Rust simulation compiled to WebAssembly; 21 lightweight Rust tests pass across the core,
  simulation, WASM bridge and independent engine consumer.
- Compiled WASM passes 510-tick, two-instance full-state equality, opaque 64-bit-safe save/load,
  invalid-save atomicity, recruitment, readable errors and memory-growth integration checks.
- 20 packaged browser control regressions pass against the real WASM with mocked DOM/Canvas.
  This is structural/simulation evidence; it is not browser rendering proof.
- The first public CI run passed simulation, full battle, replay, independent-engine and browser
  compilation/ABI jobs. Native checks continue independently.
- Spatial-query comparison: 20,000 units, 1,000 queries, five rounds. Same 14,054 returned pairs and
  result fingerprint `5e6ad574a282da93`. Median naive 27.4063 ms; indexed 4.2453 ms including build.
  Observed 6.456x speedup for this microbenchmark only.

### Current boundaries

The scene uses original procedural geometry. Production character art, animation, audio, loot,
deep build systems, campaign, networking, crowd collision avoidance, fog of war and visual authoring
remain open milestones. Fixed simulation runs at 30 Hz in this playtest. No target-hardware FPS
claim has been made. Generic core reuse is demonstrated by a separate routing sample, not a full
second game or production editor.

### Exact next action

Finish the native binary and offscreen GPU smoke, run the browser ABI and actual control tests,
publish complete source and runnable artifacts, then begin M1 combat/navigation and authored assets.

See ROADMAP.md for the complete production ambition. Later verification receipts will supersede
the in-progress build statements here without rewriting the original goals.
