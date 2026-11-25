# Technology strategy

Dependency selection checkpoint: **2026-11-24**, America/Denver. Product direction updated: **2026-11-25**, America/Denver.

## Decision

Build Direvanity as an original, offline single-player action RPG backed by a reusable Rust engine SDK. The primary experience is a personally created character, distinct classes and progression, responsive combat, exploration, quests and a large authored campaign. Keep the authoritative simulation independent of graphics, windowing, audio, UI and the first game's content. Use modern upstream components where they provide a better verified result than a new implementation.

The selected native technology target is **Rust 1.100.0, edition 2024; Bevy 0.20.1; and the wgpu 30 dependency family used by Bevy**. Published metadata verifies that Bevy 0.20.1 requires at least Rust 1.97.1. Upstream wgpu 30.0.2 is current at this checkpoint. [Rust releases](https://blog.rust-lang.org/releases/), [Bevy manifest](https://docs.rs/crate/bevy/latest/source/Cargo.toml.orig), [Bevy renderer dependencies](https://docs.rs/crate/bevy_render/latest), [wgpu releases](https://github.com/gfx-rs/wgpu/releases).

This is an upstream selection, not a claim that all proposed subsystems are already implemented. The checked-in manifest, lockfile and validation record are authoritative for the build actually delivered. Optional packages enter the dependency graph only when their feature is implemented and tested.

The delivered native Bevy and browser campaign clients use the shared Rust simulation: a character creator, 19 class identities with original playable skills, progression, equipment, quests, three connected prologue regions and separate local character saves. Its 30 Hz simulation, procedural presentation and limited campaign content are prototype capabilities. A massive campaign, production character art and animation, mature ability delivery, differentiated class resources and production-quality native presentation remain production work. The earlier army skirmish remains a legacy simulation/presentation fixture; it does not define the primary game.

## Why this foundation

Bevy supplies Rust ECS, application scheduling, assets, rendering and input without requiring the engine SDK to become a fork of its renderer. Fyrox remains an explicit challenger for an integrated editor workflow. A custom renderer remains possible through a presentation adapter, but must demonstrate a concrete improvement that outweighs maintenance and content-tooling costs.

The upstream landscape and source-level reuse assessment are recorded in [RESEARCH.md](RESEARCH.md). D2Gamma is a useful example of an authoritative simulation separated from a Bevy client. The Bevy-independent navigation and fixed-point candidates in ferrets remain available for a measured comparison if character movement, enemy crowds or companion navigation expose a concrete limitation. Navigation work follows the campaign's requirements and measured workloads.

## Engine and game boundary

| Layer | Owns | Must remain independent of |
| --- | --- | --- |
| Simulation | Ticks, entity identities, commands, movement, ability timing/delivery, combat resolution, deterministic randomness, authoritative state | Render frame rate, wall-clock timing, GPU state, device input APIs |
| Engine SDK | Shared simulation services, scenario/configuration interfaces, event/snapshot APIs, persistence boundaries | Direvanity faction names, campaign scripts and specific item identities |
| Game rules/content | Player characters, classes, resource rules, skills, enemies, progression, items, quests, encounters and campaign content | A single renderer implementation |
| Presentation | Camera, materials, animation, particles, audio, targeting feedback, character creation and UI | Ownership of authoritative outcomes |
| Tools | Content validation, profiling, replay inspection, asset processing and editing | Hidden changes to gameplay rules during a verified run |
| Platform | Window/input devices, storage adapters and native packaging; optional future transport | Game-specific balance and item calculations |

An engine reusability claim needs a second consumer. The existing `engine-lab` navigation consumer proves a focused dependency boundary. A separate scenario runner or differently themed game harness must also exercise reusable combat/content services without copying the first game's rules into engine code before those services can claim the same proof. A folder named engine is not sufficient proof.

## Simulation contract

1. Commands enter at explicit ticks with stable sequence and player/unit identity. Input ordering is defined rather than dependent on container iteration or ECS scheduling.
2. The fixed simulation step is independent of rendering. Presentation may interpolate between snapshots without changing outcomes.
3. Authoritative calculations use a specified integer/fixed-point model wherever cross-platform determinism is promised. Define overflow, division rounding and coordinate range explicitly.
4. Randomness has a recorded seed and stable algorithm. Game events cannot accidentally consume randomness according to render order.
5. State hashing includes every authoritative value necessary to continue a run. A matching partial checksum is not full replay proof.
6. Saving and loading preserve rules/content versions, simulation version, seed/state and tick. Version changes have explicit migrations or clear compatibility rejection.
7. A replay records the initial scenario plus commands and periodic full-state checkpoints or hashes. Distinguish deterministic local replay from proven cross-platform/network lockstep.
8. The campaign runs and saves locally without a network service. Optional cooperative play, online authority and transport are deferred product and architecture decisions; they must preserve the offline single-player experience.

The initial implementation should prove this contract with the gameplay it actually contains. Tests for imaginary future features provide no substitute for exercising the delivered combat loop.

## Rendering and content direction

The target visual identity is dark fantasy with strong art direction: readable silhouettes, deliberate palette and material contrast, coherent lighting, expressive animation and effects that communicate mechanics. High fidelity includes animation, sound, input response and visual clarity as well as pixel detail.

Bevy 0.20 introduces WESL shader integration, mesh-shader work, richer editor widgets and improvements to Solari and DLSS Ray Reconstruction. These are candidates for measured adoption, not a requirement to enable every feature immediately. [Official release notes](https://bevy.org/news/bevy-0-20/).

Maintain a capable raster rendering path. Evaluate advanced lighting against representative dense RPG encounters, close character views, camera motion and low-light readability. Select quality features according to measured image quality, latency, GPU time and memory on the same scene. A vendor-specific reconstruction path cannot be the only playable renderer.

Authoring formats should be portable and documented: meshes, materials and animation through glTF where applicable; explicitly versioned game definitions; reproducible texture/audio conversion; and provenance tracked independently for source code and media. Assets have their own licenses and attribution even when their source repository has a permissive code license.

No generated imagery is required for this checkpoint. Procedural original geometry, native UI and properly sourced assets can establish the interaction and rendering pipeline while the art direction matures.

## Optional subsystem candidates

| Need | Candidate | Verified compatibility | Integration gate |
| --- | --- | --- | --- |
| GPU particles | Hanabi 0.20.0 | Bevy 0.20, wgpu 30 | Native compile, reproducible effect scene, visual readability and particle/GPU profiling |
| General physics | Avian3D 0.8.0 | Bevy 0.20 | Defined authority boundary, collision fixtures and no divergence in deterministic game state |
| Optional multiplayer, deferred | Lightyear 0.31.0 | Bevy app/ECS 0.20 | A separately accepted multiplayer scope, protocol choice, connection/reconnection behavior, simulated loss/latency and authority tests; no dependency on online services for the local campaign |
| Character/enemy navigation | ferrets geometry/nav-grid/pathfinder leaf crates | Leaf manifests are Bevy-independent; root uses Bevy 0.18 | A measured need, adapter to our world schema, passage/footprint fixtures, dense encounter and companion workloads, and replay stability |
| Integrated editor alternative | Fyrox 1.0.0 | Separate engine/editor | A representative authoring task and rendered scene compared with the Bevy workflow |

Sources: [Hanabi](https://docs.rs/crate/bevy_hanabi/latest), [Avian](https://docs.rs/crate/avian3d/latest), [Lightyear](https://docs.rs/crate/lightyear/latest), [ferrets](https://github.com/valeriyr/ferrets), [Fyrox](https://fyrox.rs/blog/post/fyrox-game-engine-1-0-0/). Version compatibility in published metadata still requires a successful target build.

## Quality and performance ambition

The ambition is **AAAA production quality that can match and surpass the named ARPG inspirations**: the Diablo series, Lost Ark, the original Divine Divinity and broader Divinity series, and Path of Exile I and II. It is a project objective with evidence gates, not a label attached to the initial prototype. The design combines compatible mechanical ideas through original content and implementations; research coverage does not mean those source abilities are already playable.

### Proposed engineering targets

These are starting budgets for later measured milestones. They are not benchmark results or limits on player content.

| Dimension | Initial high-ambition target | How to evaluate |
| --- | --- | --- |
| Gameplay response | Commands accepted at the next eligible fixed tick; immediate visual acknowledgment | Instrument device input, command acceptance, simulation effect and displayed feedback |
| Simulation cadence | Evaluate 60 Hz authoritative simulation beyond the delivered 30 Hz prototype | Tick-time distribution, ability timing and replay/save compatibility under normal combat and deliberately dense encounters |
| Native rendering | Uncapped presentation with a 144 Hz flagship quality target, subject to a fixed scene/resolution/device profile | CPU/GPU frame time, p95/p99, capture quality and consistent scene settings |
| Encounter scaling | Increasing fixed workloads of enemies, summons, companions, projectiles and persistent effects without silently dropping simulated or visible entities | Identical state/command counts, correctness hashes, ability outcomes, path completion and memory scaling |
| Frame stability | No avoidable shader compilation or synchronous asset-load spikes during warmed combat | Cold/warm capture comparison, pipeline cache evidence and frame-time traces |
| Reliability | Recoverable saves, reproducible bugs, deterministic fixture results and explicit failure states | Save/replay round trips, targeted error cases and real runtime tests |
| Reusability | A second SDK consumer with different rules/content | Build and run both consumers using the same engine interfaces |

Do not claim those targets from a browser mockup, headless benchmark, debug build or a different machine. Record CPU, GPU, OS, driver, resolution, build profile, content revision, warmup, workload and samples beside results. Simulation performance and renderer performance are separate measurements.

Optimization preserves the complete requested workload. Do not introduce entity caps, hidden simulation skips, missing units or weaker content to make a chart look better. Any rendering optimization must preserve visible results and is subject to the user's no-loss constraints. Compare equivalent work and retain both correctness and performance evidence.

## Development and release gates

- Pin the intended Rust toolchain, commit Cargo.lock, and use locked builds for release validation.
- Keep lint/format errors distinct from gameplay correctness. Exercise character creation and class choice, movement, targeting, ability/resource rules, damage, death/recovery, progression, quest rewards, region travel and character save slots where implemented.
- Run the headless simulation and the real presentation client. Neither proves the other's behavior.
- Capture a coherent checkpoint before a long native/CI build. Preserve exact source revision and job identity.
- Validate native targets independently; a Linux build is not a verified Windows executable.
- Publish playable milestones with precise implemented/planned labels, instructions, known limitations and source lineage.
- Refresh the upstream comparison only when a dependency change or concrete missing capability makes it useful.

## Next material architecture experiment

Extend the running campaign with a production-oriented **ability timeline, delivery model and class resource system** in the reusable Rust core. Keep generic timing, targeting, delivery and resource operations separate from Direvanity's authored class definitions. Animation, sound and effects should follow authoritative events so that native and WASM presentations cannot produce different combat outcomes.

The experiment must make the following decisions concrete in a playable encounter:

1. Define tick-based preparation, activation, recovery, interruption and channeling semantics. Specify when a cast commits its cost and cooldown, and what cancellation refunds or preserves.
2. Exercise distinct delivery paths, including a melee strike, a traveling projectile and a placed or persistent area effect. Define hit timing, target identity, collision, lifetime and repeated-hit rules in simulation state.
3. Replace the shared prototype focus model where appropriate with authored class resource definitions. Model gain, spending, caps and regeneration explicitly; demonstrate at least two meaningfully different resource behaviors with original class skills.
4. Preserve active ability timelines, deliveries and resource state through save/load and deterministic replay. Validate rejected commands before mutation, and test interruption, death and region travel while an ability is active.
5. Measure input-to-feedback latency, simulation cost and visual readability on the same encounter. Record which behavior is implemented, which remains proposed, and the evidence for any claimed improvement.

Character and enemy navigation remain part of that encounter measurement. Compare ferrets leaf crates with the current pathfinder only if a concrete passage, crowd, footprint or companion problem warrants it, using identical workloads and preserved determinism. Optional networking is deferred. Repeating a broad engine search or changing the verified dependency pins is not the next step.
