# Technology strategy

Decision date: **2026-11-24**, America/Denver.

## Decision

Build Dreadmarch as an original action RTS backed by a reusable Rust engine SDK. Keep the authoritative simulation independent of graphics, windowing, audio, UI and the first game's content. Use modern upstream components where they provide a better verified result than a new implementation.

The selected native technology target is **Rust 1.100.0, edition 2024; Bevy 0.20.1; and the wgpu 30 dependency family used by Bevy**. Published metadata verifies that Bevy 0.20.1 requires at least Rust 1.97.1. Upstream wgpu 30.0.2 is current at this checkpoint. [Rust releases](https://blog.rust-lang.org/releases/), [Bevy manifest](https://docs.rs/crate/bevy/latest/source/Cargo.toml.orig), [Bevy renderer dependencies](https://docs.rs/crate/bevy_render/latest), [wgpu releases](https://github.com/gfx-rs/wgpu/releases).

This is an upstream selection, not a claim that all proposed subsystems are already implemented. The checked-in manifest, lockfile and validation record are authoritative for the build actually delivered. Optional packages enter the dependency graph only when their feature is implemented and tested.

## Why this foundation

Bevy supplies Rust ECS, application scheduling, assets, rendering and input without requiring the engine SDK to become a fork of its renderer. Fyrox remains an explicit challenger for an integrated editor workflow. A custom renderer remains possible through a presentation adapter, but must demonstrate a concrete improvement that outweighs maintenance and content-tooling costs.

The upstream landscape and source-level reuse assessment are recorded in [RESEARCH.md](RESEARCH.md). In particular, ferrets provides Bevy-independent navigation and fixed-point candidates worth testing before implementing a large RTS navigation stack from scratch. D2Gamma is a useful example of an authoritative simulation separated from a Bevy client.

## Engine and game boundary

| Layer | Owns | Must remain independent of |
| --- | --- | --- |
| Simulation | Ticks, unit identities, commands, movement, combat resolution, deterministic randomness, authoritative state | Render frame rate, wall-clock timing, GPU state, device input APIs |
| Engine SDK | Shared simulation services, scenario/configuration interfaces, event/snapshot APIs, persistence boundaries | Dreadmarch faction names, campaign scripts and specific item identities |
| Game rules/content | Commanders, armies, enemies, progression, encounters, scenarios and audiovisual references | A single renderer implementation |
| Presentation | Camera, materials, animation, particles, audio, selection feedback and UI | Ownership of authoritative outcomes |
| Tools | Content validation, profiling, replay inspection, asset processing and editing | Hidden changes to gameplay rules during a verified run |
| Platform | Window/input devices, storage adapters, native packaging and eventual transport | Game-specific balance and item calculations |

An engine reusability claim needs a second consumer. A small separate scenario runner, headless server or differently themed game harness must exercise the SDK without copying the first game's rules into engine code. A folder named engine is not sufficient proof.

## Simulation contract

1. Commands enter at explicit ticks with stable sequence and player/unit identity. Input ordering is defined rather than dependent on container iteration or ECS scheduling.
2. The fixed simulation step is independent of rendering. Presentation may interpolate between snapshots without changing outcomes.
3. Authoritative calculations use a specified integer/fixed-point model wherever cross-platform determinism is promised. Define overflow, division rounding and coordinate range explicitly.
4. Randomness has a recorded seed and stable algorithm. Game events cannot accidentally consume randomness according to render order.
5. State hashing includes every authoritative value necessary to continue a run. A matching partial checksum is not full replay proof.
6. Saving and loading preserve rules/content versions, simulation version, seed/state and tick. Version changes have explicit migrations or clear compatibility rejection.
7. A replay records the initial scenario plus commands and periodic full-state checkpoints or hashes. Distinguish deterministic local replay from proven cross-platform/network lockstep.
8. Online authority and transport are separate choices. A local game must remain testable without a network service.

The initial implementation should prove this contract with the gameplay it actually contains. Tests for imaginary future features provide no substitute for exercising the delivered combat loop.

## Rendering and content direction

The target visual identity is dark fantasy with strong art direction: readable silhouettes, deliberate palette and material contrast, coherent lighting, expressive animation and effects that communicate mechanics. High fidelity includes animation, sound, input response and visual clarity as well as pixel detail.

Bevy 0.20 introduces WESL shader integration, mesh-shader work, richer editor widgets and improvements to Solari and DLSS Ray Reconstruction. These are candidates for measured adoption, not a requirement to enable every feature immediately. [Official release notes](https://bevy.org/news/bevy-0-20/).

Maintain a capable raster rendering path. Evaluate advanced lighting against representative crowded battles, camera motion and low-light readability. Select quality features according to measured image quality, latency, GPU time and memory on the same scene. A vendor-specific reconstruction path cannot be the only playable renderer.

Authoring formats should be portable and documented: meshes, materials and animation through glTF where applicable; explicitly versioned game definitions; reproducible texture/audio conversion; and provenance tracked independently for source code and media. Assets have their own licenses and attribution even when their source repository has a permissive code license.

No generated imagery is required for this checkpoint. Procedural original geometry, native UI and properly sourced assets can establish the interaction and rendering pipeline while the art direction matures.

## Optional subsystem candidates

| Need | Candidate | Verified compatibility | Integration gate |
| --- | --- | --- | --- |
| GPU particles | Hanabi 0.20.0 | Bevy 0.20, wgpu 30 | Native compile, reproducible effect scene, visual readability and particle/GPU profiling |
| General physics | Avian3D 0.8.0 | Bevy 0.20 | Defined authority boundary, collision fixtures and no divergence in deterministic game state |
| Multiplayer library | Lightyear 0.31.0 | Bevy app/ECS 0.20 | Protocol choice, connection/reconnection behavior, simulated loss/latency and authority tests |
| RTS navigation | ferrets geometry/nav-grid/pathfinder leaf crates | Leaf manifests are Bevy-independent; root uses Bevy 0.18 | Adapter to our world schema, passage/footprint fixtures, group throughput and replay stability |
| Integrated editor alternative | Fyrox 1.0.0 | Separate engine/editor | A representative authoring task and rendered scene compared with the Bevy workflow |

Sources: [Hanabi](https://docs.rs/crate/bevy_hanabi/latest), [Avian](https://docs.rs/crate/avian3d/latest), [Lightyear](https://docs.rs/crate/lightyear/latest), [ferrets](https://github.com/valeriyr/ferrets), [Fyrox](https://fyrox.rs/blog/post/fyrox-game-engine-1-0-0/). Version compatibility in published metadata still requires a successful target build.

## Quality and performance ambition

The ambition is **AAAA production quality that can match and surpass the four named inspirations**. It is a project objective with evidence gates, not a label attached to the initial prototype.

### Proposed engineering targets

These are starting budgets for later measured milestones. They are not benchmark results or limits on player content.

| Dimension | Initial high-ambition target | How to evaluate |
| --- | --- | --- |
| Gameplay response | Commands accepted at the next eligible fixed tick; immediate visual acknowledgment | Instrument device input, command acceptance, simulation effect and displayed feedback |
| Simulation cadence | 60 Hz authoritative simulation as a candidate default | Tick-time distribution under normal combat and deliberately dense encounters |
| Native rendering | Uncapped presentation with a 144 Hz flagship quality target, subject to a fixed scene/resolution/device profile | CPU/GPU frame time, p95/p99, capture quality and consistent scene settings |
| Army scaling | Increasing fixed workloads, including large formations, without silently dropping simulated or visible entities | Identical state/command counts, correctness hashes, path completion and memory scaling |
| Frame stability | No avoidable shader compilation or synchronous asset-load spikes during warmed combat | Cold/warm capture comparison, pipeline cache evidence and frame-time traces |
| Reliability | Recoverable saves, reproducible bugs, deterministic fixture results and explicit failure states | Save/replay round trips, targeted error cases and real runtime tests |
| Reusability | A second SDK consumer with different rules/content | Build and run both consumers using the same engine interfaces |

Do not claim those targets from a browser mockup, headless benchmark, debug build or a different machine. Record CPU, GPU, OS, driver, resolution, build profile, content revision, warmup, workload and samples beside results. Simulation performance and renderer performance are separate measurements.

Optimization preserves the complete requested workload. Do not introduce entity caps, hidden simulation skips, missing units or weaker content to make a chart look better. Any rendering optimization must preserve visible results and is subject to the user's no-loss constraints. Compare equivalent work and retain both correctness and performance evidence.

## Development and release gates

- Pin the intended Rust toolchain, commit Cargo.lock, and use locked builds for release validation.
- Keep lint/format errors distinct from gameplay correctness. Exercise selection, movement, attacks, damage, death, restart and scenario completion where implemented.
- Run the headless simulation and the real presentation client. Neither proves the other's behavior.
- Capture a coherent checkpoint before a long native/CI build. Preserve exact source revision and job identity.
- Validate native targets independently; a Linux build is not a verified Windows executable.
- Publish playable milestones with precise implemented/planned labels, instructions, known limitations and source lineage.
- Refresh the upstream comparison only when a dependency change or concrete missing capability makes it useful.

## Next material architecture experiment

After the initial combat slice is running, evaluate the ferrets navigation leaf crates against the existing movement path on identical obstacle and formation fixtures. A useful result is an integrated better pathfinder with preserved determinism, or a recorded concrete incompatibility that explains why the current approach remains preferable. Repeating a broad engine search is not the next step.
