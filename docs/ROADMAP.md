# Production roadmap

The complete ambition remains open. Checked items require implementation and observed evidence
in STATUS.md. “Supported by Bevy” does not mean “integrated in Dreadmarch.”

## M0: playable engine foundation

- [ ] Public source repository and current pinned Rust/Bevy toolchain
- [ ] Renderer-independent integer simulation, deterministic commands and checksums
- [ ] Generic navigation and spatial-query engine crate, independent consumer example
- [ ] Commander, two squad types, enemy types, combat, recruitment, three capture sites
- [ ] Wave pressure, victory, defeat, restart, selection and movement
- [ ] Native 3D Bevy client running the actual simulation
- [ ] Browser playtest compiled from the same Rust simulation
- [ ] Versioned validated save/load and deterministic replay verification
- [ ] Reproducible build/test workflow and downloadable checkpoint

## M1: combat and navigation vertical slice

- [ ] Replace geometric units with original rigged production models and animation sets
- [ ] Data-driven ability graph: targeting, costs, cooldowns, effects, interrupts and tags
- [ ] Damage/defense/status model with authored tooltips and combat diagnostics
- [ ] Animation anticipation, impact, recovery, hit reactions and audio layers
- [ ] Shared-goal path fields or hierarchical paths, unit footprints, avoidance and congestion tests
- [ ] Controller/direct hero control, control groups, order queues, attack/hold/patrol commands
- [ ] One handcrafted boss with readable telegraphs, counterplay and squad-aware mechanics
- [ ] Item drops, inventory, equipment, affixes and save migration
- [ ] Windows target-hardware capture with input and frame-time instrumentation

## M2: production rendering and tools

- [ ] Asset manifest, import/cook pipeline, provenance, dependency hot reload and validation
- [ ] GLTF materials/animation, terrain layers, decals, atmospheric lighting and shadows
- [ ] GPU particles with bounded visual budgets and zero gameplay truncation
- [ ] Compare current raster path against Solari and other advanced paths on identical scenes
- [ ] Shader warmup, pipeline caching, streaming, device-loss recovery and fallback tests
- [ ] Material/ability/unit/encounter authoring tools with undo/redo and validation
- [ ] Replay viewer, profiler overlays, automated visual captures and regression review
- [ ] Audio middleware evaluation, dynamic score and spatial mixing

## M3: multiplayer and reusable SDK

- [ ] Decide authoritative server vs deterministic lockstep from measured game requirements
- [ ] Tick-addressed commands, versioned protocol, desync diagnostics and replay compatibility
- [ ] Transport evaluation using current Lightyear/QUIC candidates and impairment tests
- [ ] Reconnection, late join, save compatibility, abuse resistance and server observability
- [ ] Second distinct game sample with different content/rules using the same engine SDK
- [ ] Stable engine API boundaries, documentation, examples and migration policy

## M4: flagship content and production quality

- [ ] Multiple distinctive factions and commanders with orthogonal combat identities
- [ ] Deep progression, itemization, army doctrines and meaningful build interactions
- [ ] Campaign, authored regions, repeatable encounters, bosses and endgame progression
- [ ] Accessibility, localization, rebinds, UI scale, color-independent feedback and reduced motion
- [ ] Comprehensive settings, save recovery, crash reporting with user consent and diagnostics
- [ ] Scale tiers, long-duration soak, cross-vendor GPUs, cold/warm loading and frame-time gates
- [ ] Repeated blind playtests and professional art/audio/animation review
- [ ] Demonstrated advantage over selected inspiration benchmarks before “surpass” claims
