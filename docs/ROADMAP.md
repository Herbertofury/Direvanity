# Production roadmap

Primary product: an offline single-player action RPG. Checked items require implementation and evidence
in [STATUS.md](STATUS.md). Supported by a dependency does not mean integrated into Dreadmarch.

## R0: authoritative RPG foundation

- [x] Public repository, pinned Rust/Bevy toolchain, renderer-independent core and second consumer
- [x] Shared native/WASM integer simulation, commands, validated saves and deterministic legacy replay
- [x] All 19 researched Diablo class identities in the original gameplay catalogue
- [x] Browser creator with name, silhouette, hair, colors and stature
- [x] 174 original prototype skill nodes with prerequisites, active effects and mastery passives
- [x] Single hero, skill points, XP, levels, focus/cooldowns, loot and equipped damage
- [x] Three connected persistent regions, dialogue, complete main quest and side quest
- [x] Independent browser character slots, save/resume, export and import
- [x] Every class completes both quests through actual public commands
- [x] Native creator/campaign integration, release build and actual runtime evidence
- [ ] Actual browser visual/input review and complete published RPG artifact

## R1: one production-quality region

- [ ] Original rigged bodies, faces, hair and equipment with creator persistence and appearance migration
- [ ] Direct movement, click-to-move and controller schemes with rebinding and accessible camera
- [ ] Typed ability execution: targeting, cost, windup, delivery, hit, effects, recovery and interrupts
- [ ] Distinct per-class resources and signature mechanics; behaviorally different active variants
- [ ] Real projectiles, melee sweeps, chains, summons, traps, surfaces, status interactions and dispels
- [ ] Damage/defense/resistance model, item affixes, tooltips and inspectable combat provenance
- [ ] One authored town/wilderness/dungeon region with branching dialogue, choices and consequences
- [ ] Enemy ecology, encounter composition, elite modifiers and a handcrafted multi-phase boss
- [ ] Original animation, hit reactions, audio layers, VFX and environmental storytelling
- [ ] Stable full-region saves, corruption recovery, checkpoints and death/recovery design
- [ ] Windows/Linux hardware input and frame-time evidence; controller and accessibility review

## R2: production engine and content tools

- [ ] Import/cook manifest with provenance, dependencies, hot reload and validation
- [ ] GLTF/PBR/material/animation pipeline, terrain layers, decals, lighting and shadows
- [ ] Evaluate advanced Bevy rendering paths against measured raster baselines on identical scenes
- [ ] Shader warmup, pipeline caching, asset streaming, device-loss recovery and fallbacks
- [ ] Quest/dialogue/ability/item/encounter authoring tools with undo/redo and reference checks
- [ ] Persistent object IDs, region transitions, quest event contracts and versioned save migrations
- [ ] Profiling, replay viewer, combat diagnostics, automated visual capture and content coverage tools
- [ ] Hierarchical navigation, footprints, avoidance and congestion tests without dropped entities
- [ ] Source-backed ability research reconciled against exact owned game-data editions where needed

## R3: the massive campaign

The proposed planning floor in [ARPG_SYSTEMS.md](research/ARPG_SYSTEMS.md) is a budgetable production
scope, not delivered content or a playtime promise. Authoring throughput and player testing must support it.

- [ ] Six acts across twelve major regions with distinct towns, wilderness and dungeon families
- [ ] Main narrative, class-informed dialogue, optional factions, meaningful choices and persistent outcomes
- [ ] Regional enemy families, extensive boss roster, secrets, puzzles and alternate routes
- [ ] All 19 class identities with full behaviorally distinct trees, item support and complete playthroughs
- [ ] Broad shared adventuring talents inspired by original Divine Divinity's cross-class freedom
- [ ] Extensive itemization and crafting with clear tradeoffs and solo-accessible progression
- [ ] Endgame exploration/build challenges that fit the single-player world
- [ ] Localization, accessibility, settings, recovery and long-duration complete-campaign testing

## R4: prove the quality and the reusable engine

- [ ] Finished scene/encounter benchmark suite against the inspiration quality goals
- [ ] Art, animation and audio review, blind playtests and measurable usability outcomes
- [ ] Cross-vendor GPUs, multiple hardware tiers, cold/warm loading and p95/p99 frame pacing
- [ ] Second distinct game using the engine SDK without Dreadmarch content or rules
- [ ] Stable documented APIs, examples, content tooling and migration policy
- [ ] Optional cooperation/networking only after the offline solo campaign requirements remain intact

The original RTS skirmish stays available as an engine regression and optional experiment. Its army
systems no longer determine the campaign's required progression or design priorities.
