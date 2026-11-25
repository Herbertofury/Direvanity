# Verified checkpoint

Project: Dreadmarch · Repository: https://github.com/Herbertofury/Dreadmarch · Branch: main

## First playable foundation: v0.1.0-alpha.1

Founded 24 November 2026. This is a playable engineering foundation toward the full production vision.
Release verification updated 25 November 2026 UTC.

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
  Separately, the live browser playtest was visually inspected and exercised through actual controls:
  launch, pause/resume, recruitment, commander/army selection, right-click orders, Soulfire targeting,
  field manual, restart and save/load. Saving at 13 units/30 essence, recruiting to 14/5, and loading
  restored 13/30. The launch screen, battlefield, objectives, minimap and action bar were readable.
- Live browser playtest: https://herbertofury.github.io/Dreadmarch/. Deployment run `45566607509`
  passed the compiled ABI and control checks before publication.
- A fresh extraction of the earlier `780eec8` checkpoint verified all 51 file hashes and executable mode,
  then independently passed the 120-frame native GPU smoke with the identical final checksum.
- The independent warehouse-routing sample uses only the generic engine, with no game-rule imports.
- Cross-platform CI run `45565948891` passed all three jobs: verification `139040657046`,
  Linux native `139040657306` and Windows native `139040657395`. This includes simulation,
  full battle, replay, independent-engine and browser checks, plus strict Clippy, all three native
  regression tests and native builds on both operating systems.
- Optimized Windows x64 and portable Linux x64 artifacts were built. The exact downloaded portable
  Linux artifact independently passed its checksum, executable-mode, glibc and renderer/gameplay gates.
- Spatial-query comparison: 20,000 units, 1,000 queries, five rounds. Same 14,054 returned pairs and
  fingerprint `5e6ad574a282da93`. Median naive 27.4937 ms; indexed 4.1730 ms including index build.
  Observed 6.588x speedup for this CPU microbenchmark only, not a rendered unit-count or FPS claim.
- Checkpoint packaging records every file's SHA-256 and source revision, preserves executable
  permissions, rejects collisions and requires clean source by default.

### Native release evidence

The release artifacts have distinct build revisions. The portable Linux revision adds the explicit
Ubuntu 24.04 baseline and packaging gate; the Windows executable comes from the earlier release build.
Release receipts preserve those exact revisions rather than assigning a later documentation commit
to an existing binary.

| Artifact | Build source | Verified result |
| --- | --- | --- |
| Linux x64 optimized | `93c51dc4ede21580c7bc90ba92d98b48cb282844` | Downloaded artifact, archive integrity, fresh extraction, glibc requirement and actual offscreen renderer/gameplay all passed |
| Windows x64 optimized | `780eec867be4bfd1b910cfb11f091c0475e73ce8` | Executable built and packaged; Windows CI build, strict Clippy and native regression tests passed; Windows GPU runtime remains unverified |

**Portable Linux.** Workflow run `45567826892`, job `139046854131`, artifact `15001077210`
(`portable-linux-x64`) produced the verified package. The provider artifact digest matched the
download, and the inner ZIP matched both its bundled SHA file and build receipt. Independent ELF
inspection found a maximum glibc symbol requirement of **2.35**, within the enforced **2.39** ceiling.
The exact freshly extracted 112,299,952-byte executable ran on Debian 13 / glibc 2.41 with scoped
Mesa software Vulkan and exited 0:

```text
DREADMARCH_RENDER_SMOKE_OK frames=120 tick=120 units=12 checksum=4d08544a95f6c631 screenshot_captured=true commands_ok=3 save_roundtrip=true hud_validated=true
```

The new 1440 × 900 screenshot was visually inspected: full 3D scene, squads, relics, lighting,
top counters, bottom action bar and instructions were readable. Its state checksum matches the CI
smoke. This verifies the exact optimized artifact, in addition to the earlier development build.

- Linux ZIP SHA-256: `30dbc908368558ca63d93fd3c29d85d36b38c80fc91ffdea01eee339c317ba16`.
- Linux executable SHA-256: `c89dd59542129e18efef6423e09e655afa9862723deb5510734ff28c0fba5465`.
- Provider artifact ZIP SHA-256: `e8896107e9e617a9d3f8f00582eeec16db693f7bb11ccce1e2e0fffb506b4197`.
- Extracted executable screenshot SHA-256: `f5f4a5f92e1d0fc6e97ccb36e65cf573e0d16a68c3b1b76ba603e0003c8852f1`.
- Local verification receipt, log and capture: `dist/portable-verified/receipt.json`,
  `dist/portable-verified/native-smoke.log`, `dist/portable-verified/native-smoke.png`.

The earlier Ubuntu-latest Linux executable required `GLIBC_2.43` and failed before application
initialization on Debian 13. It is excluded from this success claim; see [RECOVERY.md](RECOVERY.md).

**Windows.** The optimized PE executable is 76,246,528 bytes and imports `VCRUNTIME140.dll`.
Install the [official Microsoft Visual C++ x64 Redistributable](https://aka.ms/vc14/vc_redist.x64.exe)
if that runtime is missing; the [Microsoft runtime guide](https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist/)
documents supported versions. Microsoft runtime DLLs are not bundled.

- Windows ZIP SHA-256: `f2c4c947bd3e97078b023180db9b34f80b26915cfab72a98e9755083b9e09883`.
- Windows executable SHA-256: `81894e9ff5549508f405975a7ea6d7b0977dcaef26b65dd6fcdf36c2db239470`.

### Current boundaries

The scene uses original procedural geometry. Production character art, animation, audio, loot,
deep build systems, campaign, networking, crowd collision avoidance, fog of war and visual authoring
remain open milestones. Fixed simulation runs at 30 Hz in this playtest. No target-hardware FPS
claim has been made. Generic core reuse is demonstrated by a separate routing sample, not a full
second game or production editor. Native windowed mouse/keyboard end-to-end play and Windows GPU
runtime remain separate verification gates; automated native UI activation is covered.

### Exact next action

Begin **M1 combat and navigation**: add unit footprints, local avoidance and congestion regressions;
introduce data-driven ability effects, damage/defense/status rules and combat diagnostics; then connect
authored animation and the first boss encounter to those rules. Capture Windows target-hardware input
and frame times during the vertical slice. Release publication and checkpoint distribution close out
M0 and do not require another foundation-design phase. See [ROADMAP.md](ROADMAP.md) and the public
issue tracker for the full ambition.
