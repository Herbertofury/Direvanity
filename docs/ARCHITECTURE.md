# Architecture

## Dependency direction

| Package | Owns | Must not own |
| --- | --- | --- |
| dread-core | Integer world coordinates, generic grid navigation and spatial indexing | Dreadmarch classes, graphics, browser state |
| dread-sim | Game rules, unit state, combat, objectives, economy, commands, saves | Camera, UI, frame rate, nondeterministic wall-clock decisions |
| dreadmarch-client | Bevy3D presentation, input mapping, selection, camera and HUD | An independent combat/navigation implementation |
| dread-wasm | Thin C ABI and JSON boundary over the same Rust simulation | JavaScript gameplay rules |
| dread-headless | Replays, deterministic verification and CPU timing | GPU/FPS claims |
| engine-lab | Independent consumer of the generic navigation API | Imports from dread-sim |

The initial engine is a focused reusable layer on Bevy, not a new graphics API implementation.
Bevy supplies ECS, renderer, windows, assets and UI. The project supplies game-specific orchestration
and a separate deterministic world. Keep the renderer adapter replaceable; do not fork Bevy unless
an observed upstream limitation demands a maintained patch.

## Simulation contract

The simulation advances at 30 ticks per second in this first playtest. Coordinates use integer
milliworld units; conversion to floating point occurs only at the presentation boundary. Commands
are applied in explicit order. State contains its own tick, seed/random state, units, objective
progress and cooldowns. Rendering frames may interpolate but may not alter authority.

Events describe presentation opportunities; the clients derive their visuals from real outcomes.
No cosmetic effect is evidence that damage occurred. Save loading must validate schema, topology,
identities, numerical ranges and route data before replacing live state.

## Reuse versus proof

The core consumer example exercises navigation without importing this game's simulation. This
demonstrates a useful boundary, not a complete general-purpose editor or a shipped SDK. Advanced
asset cooking, visual authoring, networking and modular content remain roadmap work.

## Rendering progression

Start with a stable raster path and original procedural geometry to prove camera, selection,
animation timing and battlefield readability. Upgrade to authored PBR assets, layered animation,
particles, terrain and lighting in reviewable increments. Compare advanced GPU pipelines on the
same scene and hardware. Never treat “latest” as a benchmark result.

## Performance discipline

Nearest-target acquisition should use spatial indexing while preserving results against brute
force. Pathfinding must respect blocked cells and deterministic tie ordering. Reuse allocations
where measured. Keep all units in the authoritative simulation. Any future rendering visibility
optimization must preserve full world state and exact visible results; it cannot become a content
or population limit.

Headless CPU timings and software-renderer smoke tests answer different questions. Record both,
with seed, unit count, ticks, exact artifact, hardware/runtime, settings and checksums. A container
software renderer cannot establish the user's RTX 4090 frame rate.

## Campaign rules and content boundary

`dread-sim::rpg` owns character validation, class lookup, original skills, progression, equipment,
interactions and region memory. `data/gameplay/classes.json` is compiled into the simulation, so native
and WASM clients cannot diverge by maintaining separate skill rules. Research catalogues are not
compiled as playable abilities. They retain editions/provenance for design and coverage work.

Campaign save version 2 stores appearance, learned skills, cooldowns, inventory/equipment, quest flags,
visited regions, enemies and uncollected loot. Legacy skirmish version 1 remains loadable with its
original deterministic behavior. Saves are validated before replacing the running world, including
canonical region maps, traversal paths, identity uniqueness, prerequisites and point accounting.
Fallible reward and encounter allocation is preflighted before a one-time quest/cache/travel change.

The browser's creator, skill/inventory/journal/world panels and save slots are presentation. Character
slots store exact Rust-produced UTF-8 plus a separate small metadata index. Import validates through
Rust before restoration; 64-bit RNG state is never reserialized through JavaScript Number. Modal
panels pause the simulation, contain keyboard focus and restore the prior pause state.
