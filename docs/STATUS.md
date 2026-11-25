# Verified checkpoint

Project: Dreadmarch · Repository: https://github.com/Herbertofury/Dreadmarch · Branch: main

## First playable foundation

Founded 24 November 2026. This is a playable engineering foundation toward the full production vision.

### Verified

- Public repository created and visibility read back as public.
- Rust 1.100.0, Bevy 0.20.1 and wgpu 30.0.2 are the actual locked and built dependency graph.
- All 24 Rust tests pass: 21 across core, simulation, WASM bridge and independent engine consumer;
  three native client regressions exercise command rejection, surviving selections and actual UI activation.
- Strict Clippy passes for all packages and native targets; formatting is checked.
- Full battle through public commands reaches Victory at tick 4,392: all five waves, all three relics,
  80 kills, 43 allied survivors, and successful mid-battle save/load.
- Full-battle checksum: `43701d5395d1a649`.
- Native client built and ran the actual Vulkan renderer for 120 frames/ticks. Three real commands
  (move, recruit, cast) succeeded, and save/load preserved checksum `4d08544a95f6c631`.
- The native GPU readback is a 1440 × 900 PNG. The original 3D scene, lighting/shadows, units,
  title, live HUD, action costs and instructions were visually inspected. HUD camera assignment
  and nonzero layout are also runtime assertions.
- Native runtime: Mesa 25.0.7 llvmpipe / LLVM 19.1.7, offscreen. This establishes rendering
  correctness on software Vulkan; it does not establish gaming-GPU FPS or windowed input behavior.
- Compiled WASM passes 510-tick, two-instance full-state equality, opaque 64-bit-safe save/load,
  invalid-save atomicity, recruitment, readable errors and memory-growth integration checks.
- The offline browser file embeds the compiled Rust simulation and is 589,020 bytes.
- 20 packaged browser control regressions pass against the real WASM with mocked DOM/Canvas.
  This is structural/simulation evidence; browser visual rendering has not been independently captured.
- The independent warehouse-routing sample uses only the generic engine, with no game-rule imports.
- Public CI has passed simulation, full battle, replay, independent-engine and browser jobs.
  Earlier native lint findings were corrected and strict Clippy passed locally. Final cross-platform
  CI and optimized Windows/Linux release builds are the publication gates.
- Spatial-query comparison: 20,000 units, 1,000 queries, five rounds. Same 14,054 returned pairs and
  fingerprint `5e6ad574a282da93`. Median naive 27.4937 ms; indexed 4.1730 ms including index build.
  Observed 6.588x speedup for this CPU microbenchmark only, not a rendered unit-count or FPS claim.
- Checkpoint packaging records every file's SHA-256 and source revision, preserves executable
  permissions, rejects collisions and requires clean source by default.

### Current boundaries

The scene uses original procedural geometry. Production character art, animation, audio, loot,
deep build systems, campaign, networking, crowd collision avoidance, fog of war and visual authoring
remain open milestones. Fixed simulation runs at 30 Hz in this playtest. No target-hardware FPS
claim has been made. Generic core reuse is demonstrated by a separate routing sample, not a full
second game or production editor. Native windowed mouse/keyboard end-to-end play and Windows GPU
runtime remain separate verification gates; automated native UI activation is covered.

### Exact next action

Publish the optimized Windows/Linux prerelease and complete source/runtime checkpoint, then begin
M1 combat/navigation and authored assets. See ROADMAP.md and the public issue tracker for the full ambition.
