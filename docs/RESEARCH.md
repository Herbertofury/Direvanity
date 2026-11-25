# Research and adoption decisions

Research date: **2026-11-24**, America/Denver. Working title: **Direvanity**.

## Direction

Direvanity is now an original dark-fantasy, offline single-player action RPG and the first consumer of a reusable Rust engine. **Original Divine Divinity (2002), the complete Diablo series, Diablo IV, Lost Ark, Path of Exile, and Path of Exile II are explicit creative and quality inspirations.** Their names describe the intended standard of combat, atmosphere, encounters and systems. The project intends to match and surpass those benchmarks through original execution. No current prototype is described as having achieved that standard.

The combination is deliberate: commander-level action and character expression inside battles that also reward squad control, positioning and strategic decisions. The engine must remain usable for other games with different content and rules.

## Inspiration register

The source column records developer or publisher material. The design interpretation and acceptance evidence are Direvanity decisions, rather than claims made by those sources.

| Inspiration | Source-grounded characteristics | Direvanity interpretation | Evidence required from our implementation |
| --- | --- | --- | --- |
| **Diablo IV** | Blizzard describes a dark-fantasy world, distinct classes, powerful abilities, gear and character progression. [Official game overview](https://diablo4.blizzard.com/en-us) | Weighty, tactile combat; coherent dark materials and atmosphere; strong silhouettes; attacks whose sound, timing and visual impact agree. | Captured gameplay with impact timing, hit confirmation, enemy reactions, controlled exposure and legible units in a crowded battle. |
| **Lost Ark** | Its official overview emphasizes advanced classes, ability customization, large bosses and multiphase raids with complex mechanics. [Official overview](https://www.playlostark.com/en-us/game/about) | Spectacular abilities and encounters that remain understandable at RTS camera distance; coordinated mechanics for a commander and army. | Boss mechanics are telegraphed clearly; players can identify danger, safe space and their own units during the largest effects. |
| **Path of Exile** | Grinding Gear Games documents support-gem interactions, shared passive-tree choices and a deep item system. [Official game overview](https://www.pathofexile.com/game), [official passive tree](https://www.pathofexile.com/passive-skill-tree) | Composable commander, army and equipment builds that alter decisions and behavior, with understandable explanations and reliable calculations. | Different builds produce different tactics; modifier ordering is tested; tooltips explain outcomes; save and replay results reproduce the same state. |
| **Path of Exile II** | GGG's store description emphasizes boss-specific mechanics, skill combinations and character customization; its developer feedback post directly discusses dodge collision and movement control. [Official site](https://pathofexile2.com/), [GGG's Steam page](https://store.steampowered.com/app/2694490/Path_of_Exile_2/), [developer movement discussion](https://www.pathofexile.com/forum/view-thread/3611705) | Deliberate movement, committed attacks, expressive animation and memorable bosses, while preserving responsive army commands. | Players can anticipate and counter attacks; movement and targeting are reliable; animation and collision timing agree. |

The 2024 PoE II developer post is used as historical design evidence, not as a statement of current patch balance. No class counts, launch content totals or current-season mechanics are required by these pillars.

## Current technology evidence

| Item | Verified published state | Primary source |
| --- | --- | --- |
| Rust | **1.100.0**, released 2026-11-12 | [Rust release index](https://blog.rust-lang.org/releases/) |
| Bevy | **0.20.1**, published 2026-10-28; Rust edition 2024; MSRV 1.97.1; MIT OR Apache-2.0 | [Crate documentation](https://docs.rs/bevy/latest/bevy/), [published manifest](https://docs.rs/crate/bevy/latest/source/Cargo.toml.orig) |
| Bevy renderer dependency | `wgpu ^30` in `bevy_render 0.20.1` | [Published dependency metadata](https://docs.rs/crate/bevy_render/latest) |
| wgpu | **30.0.2**, released 2026-11-12 | [Official releases](https://github.com/gfx-rs/wgpu/releases) |
| Fyrox | **1.0.0** stable; integrated scene editor; MIT | [Release notes](https://fyrox.rs/blog/post/fyrox-game-engine-1-0-0/), [repository](https://github.com/FyroxEngine/Fyrox) |
| Hanabi | **0.20.0**; Bevy `^0.20`, wgpu `^30` | [Published metadata](https://docs.rs/crate/bevy_hanabi/latest) |
| Avian3D | **0.8.0**; Bevy `^0.20` | [Published metadata](https://docs.rs/crate/avian3d/latest) |
| Lightyear | **0.31.0**; Bevy app/ECS `^0.20` | [Published metadata](https://docs.rs/crate/lightyear/latest) |

These are verified upstream versions, not a claim that every candidate has been integrated or successfully built here. The repository's lockfile and validation record determine the delivered dependency graph. At inspection, docs.rs reported a failed documentation build for the latest Hanabi and Lightyear packages; that is a reason to test the intended native configuration, not evidence that native runtime use fails.

## Existing projects and reuse decisions

| Project | Actual implementation and provenance | Fit and current disposition |
| --- | --- | --- |
| [D2Gamma](https://github.com/Nugwa628/d2gamma) | Rust/Bevy Diablo II: Lord of Destruction reimplementation; MIT; author labels it early beta. Manifest pins Bevy `=0.19.1` and MSRV 1.95. | Highest-priority Diablo-specific architecture and subsystem candidate. Audit isolated code before importing. Its Diablo-specific data and behavior assumptions need an explicit adapter and do not define Direvanity's original world or reusable core. |
| [ferrets](https://github.com/valeriyr/ferrets) | Rust RTS engine; MIT OR Apache-2.0. Fixed-point simulation, pathfinding, lockstep and replay facilities. Root manifest currently uses Bevy 0.18. | Highest-priority navigation and determinism challenger. Several leaf crates have no Bevy dependency and can be evaluated separately. See bounded audit below. |
| [Digital Extinction](https://github.com/DigitalExtinction/Game) | Rust/Bevy 3D RTS. Code AGPL-3.0; assets generally CC BY-SA 4.0 with per-directory exceptions. Archived 2026-02-17; README identifies early development. | Historical reference for RTS controls, architecture and multiplayer packaging. Not selected as a maintained engine foundation. Any code or asset integration needs its own recorded license disposition. |
| [Oxidator](https://github.com/Ruddle/oxidator) | MIT Rust/WebGPU RTS game/engine. README shows a 35,000-unit demonstration and names large-scale RTS influences. | Reference for army-scale experiments. The demonstration is an upstream claim, not a Direvanity benchmark or a verified current-hardware result. |
| [Veloren](https://gitlab.com/veloren/veloren/) | GPL-3.0 Rust multiplayer voxel RPG; [official GitHub mirror](https://github.com/veloren/veloren). | Reference for streaming, networking, world simulation and content pipeline. Different world representation and art goals make a wholesale fork a poor initial fit. |
| [Fish Folk: Jumpy](https://github.com/fishfolk/jumpy) | Rust/Bevy tactical 2D shooter. Source MIT OR Apache-2.0; media CC BY-NC under its [license file](https://github.com/fishfolk/jumpy/blob/main/LICENSE). | Reference for content architecture and local/online play. Code and media have different provenance. |
| [OpenDiablo2](https://github.com/OpenDiablo2/OpenDiablo2) | Go Diablo II reimplementation; GPL-3.0; archived 2021-12-20. | Historical architecture reference, not a Rust engine base. Its README points to the Abyss Engine split. |
| [DevilutionX](https://github.com/diasurgical/DevilutionX) | C++ modern Diablo I port associated with the Devilution decompilation lineage. The current [LICENSE.md](https://github.com/diasurgical/DevilutionX/blob/master/LICENSE.md) identifies Sustainable Use License 1.0. | Behavioral/reference candidate. Old search snippets describing Unlicense are not authoritative for the current revision. |
| [rend3](https://github.com/BVE-Reborn/rend3) | Rust/wgpu rendering library; archived 2025-06-07; repository says maintenance mode. | Not selected as the primary renderer for a new project. |

### Reimplementation is not recompilation

D2Gamma and OpenDiablo2 describe rewritten engines that interpret game data. DevilutionX descends from a recovered Diablo engine. These are different from a generic binary recompiler, and none is a ready-made Diablo IV, Lost Ark or Path of Exile engine. Direvanity uses original game content and a reusable engine boundary; asset-format compatibility is a separate possible tooling feature.

## Bounded ferrets source audit

Read through the GitHub file API on the research date:

| File | Observed content | Git blob SHA |
| --- | --- | --- |
| [`ferrets-math/Cargo.toml`](https://github.com/valeriyr/ferrets/blob/main/crates/ferrets-math/Cargo.toml) | `fixed 1.31` with serde plus serde; no Bevy dependency | `00eec0ab65915a1dcc7f1307a5f21850d41d9f32` |
| [`ferrets-math/src/lib.rs`](https://github.com/valeriyr/ferrets/blob/main/crates/ferrets-math/src/lib.rs) | Signed I32F32 and unsigned U32F32 scalar aliases; vector, rectangle and facing modules | `5e55182bf7f70e4d51c4f6e0a925d40333b690d6` |
| [`ferrets-math/src/fixed_vec2.rs`](https://github.com/valeriyr/ferrets/blob/main/crates/ferrets-math/src/fixed_vec2.rs) | Serializable vector, dot product, squared distance and arithmetic | `29dd3d6193df571dd9523f21f559df7707018fe9` |
| [`ferrets-pathfinder/Cargo.toml`](https://github.com/valeriyr/ferrets/blob/main/crates/ferrets-pathfinder/Cargo.toml) | `pathfinding 4.15`, ferrets-geometry and ferrets-nav-grid; no direct Bevy dependency | `2a21376621f97dcdfa893322bd276f732e0f0a09` |
| [`ferrets-pathfinder/src/astar.rs`](https://github.com/valeriyr/ferrets/blob/main/crates/ferrets-pathfinder/src/astar.rs) | Fixed neighbor order, mover footprints, stop distance and blocked diagonal-corner checks | `6fd845e9fa35b878d66279afdc42e155c2b20f55` |
| [`ferrets-pathfinder/src/search.rs`](https://github.com/valeriyr/ferrets/blob/main/crates/ferrets-pathfinder/src/search.rs) | Deterministic placement ordering and reachable/free-position searches | `0cad611027d835fe0bc2bd25ed3f220db30345dd` |
| [`ferrets-pathfinder/src/hpa.rs`](https://github.com/valeriyr/ferrets/blob/main/crates/ferrets-pathfinder/src/hpa.rs) | Hierarchical route corridors, unreachable-goal repair, progressive refinement and corridor reuse for group orders | `36d47955289b66087db38fdfbea5570bd232cfd8` |

The manifests and math files were read in full; navigation implementation inspection covered the first 180 lines of each named source file. This establishes useful interfaces and candidate behavior, not an exhaustive correctness audit or a proven performance advantage.

**Adoption decision:** prioritize a navigation adapter experiment when obstacle-rich maps enter the slice. The hierarchy, multi-cell footprint and shared-corridor mechanisms are materially more capable than straight-line steering. They also assume ferrets grid, geometry and shape types, so inserting them into a different world representation is an integration task. Importing the entire Bevy 0.18 engine would unnecessarily couple unrelated parts of Direvanity.

The next comparison must include narrow passages, disconnected goals, differing unit sizes, diagonal walls, dynamic obstruction and large formation orders. Compare path validity, full-group completion, deterministic hashes, memory and time against the current implementation using identical requests. Preserve notices and pin a full upstream commit before integration. Never describe an unrun comparison as an optimization.

The math crate can be evaluated without renderer migration. A switch from an existing integer-coordinate format must specify scale, range, rounding, overflow and save migration. Using Rust or fixed-point types alone does not establish deterministic simulation.

The initial engine comparison below retains its 24 November 2026 snapshot. The expanded RPG research is in [Diablo](research/DIABLO_CLASS_ATLAS.md), [Divinity](research/DIVINITY_ATLAS.md), and [Lost Ark / Path of Exile](research/ARPG_SYSTEMS.md).

## Bounded D2Gamma architecture audit

The [architecture document](https://github.com/Nugwa628/d2gamma/blob/main/docs/ARCHITECTURE.md) separates data readers, native mod layers, simulation, protocol, transport, authoritative server and Bevy client. It describes the same simulation host for offline and online play, with an in-memory transport for a local game. Data-format and simulation crates stay independent of Bevy.

**Adoption decision:** use this as evidence for a clean simulation/client boundary and a transport-neutral command protocol. The codebase is more directly useful for future format tools, item-system study and save-validation patterns than for immediate RTS gameplay. Its simulation depends on Diablo-specific tables and rules, and its renderer is a Bevy generation behind the selected target. A wholesale fork would add content assumptions before the original game has proved its own loop. No Diablo assets are needed to implement these architectural boundaries.

No D2Gamma code was imported by this research task. The project's advertised full-game features were not independently played or verified.

## Selection and follow-up

- **Adopt now:** Rust, a reusable headless simulation boundary, deterministic commands, reproducible tests and a separate presentation layer.
- **Selected native renderer target:** Bevy 0.20.1 and its compatible wgpu 30 dependency family.
- **First integration challenger:** ferrets navigation leaf crates, evaluated behind an adapter with comparative fixtures.
- **Further candidates:** Hanabi for visual effects, Avian for optional presentation/physics interactions, Lightyear for a chosen network protocol, and Fyrox as the editor/workflow challenger.
- **Reference only at this checkpoint:** Diablo reimplementations, Veloren, Digital Extinction, Oxidator and Jumpy unless a later commit explicitly records an import.

Recheck freshness when changing a dependency or revisiting a candidate. A search miss, unavailable source page or failed documentation build is recorded as unresolved evidence, never converted into proof that a project or feature does not exist.
