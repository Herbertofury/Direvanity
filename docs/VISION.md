# Dreadmarch: the ambition

Working title. An original dark-fantasy action RTS, built on an engine that can power other games.
Founded 24 November 2026. This document defines targets, not achieved production quality.

## The promise

Command an army with the clarity and agency of a great RTS. Fight through its front line as a
commander with the impact, responsiveness, build depth, and audiovisual ambition of an exceptional
action RPG. Every ability should change a tactical decision. Every army should express a build.
The engine should make a second game faster to create without importing Dreadmarch's identity.

## Great inspirations

| Inspiration | What we want to learn | What Dreadmarch should make its own |
| --- | --- | --- |
| Diablo IV | Grounded dark-fantasy atmosphere, impactful attacks, cohesive material and lighting language | Commanders whose strikes visibly matter inside a readable army battle |
| Lost Ark | Isometric spectacle, expressive skill animation, readable encounter choreography | Large encounters where squad positioning and commander execution interact |
| Path of Exile | Interlocking skills, itemization, build experimentation, long-lived progression | Army doctrines, commander skills, equipment, and faction synergies that create different play styles |
| Path of Exile II | Deliberate combat, expressive animation, boss anticipation and execution | Bosses designed for both direct hero play and tactical orders |

These are inspirations, not feature-parity or asset-reuse claims. Primary links and candidate
research are in [RESEARCH.md](RESEARCH.md).

## Product pillars

1. **Combat with weight and precision.** Fast input acknowledgment, legible hit timing, controllable
   animation commitments, directional reactions, layered VFX and audio, satisfying death responses.
2. **Strategy that survives the spectacle.** Formation movement, terrain, flanking, threat, counters,
   readable enemy intent, economic decisions, and victory conditions beyond damage output.
3. **Builds at commander and army scale.** Skills, item affixes, talents, faction doctrines, squad
   upgrades, and encounter rules use a shared data-driven effect model with inspectable provenance.
4. **A world with an artistic identity.** Original architecture, factions, silhouettes, color scripts,
   animation, orchestration, environmental storytelling, and consistent art direction.
5. **A reusable engine.** Generic math, navigation, scheduling, assets, rendering, editor, input,
   persistence, profiling, and networking interfaces. Game content stays outside the generic core.
6. **Performance and quality together.** Instrument frame-time percentiles, input latency, memory,
   streaming, loading, determinism, and simulation cost. No unit-count caps or dropped simulation
   content disguised as optimizations.

## Extremely high quality bar

“AAAA” expresses the ambition. It is not a certification or a status label for this prototype.
Surpassing the inspirations must be demonstrated by comparable finished encounters, art, animation,
audio, usability, build diversity, stability, and measured player experience.

| Dimension | Production target | Proof required before the claim |
| --- | --- | --- |
| Frame pacing | 144 Hz class high-end target, with explicit 60/120/144 Hz settings and uncapped option | Representative shipping scene traces, p50/p95/p99, target hardware and exact settings |
| Simulation | Stable fixed-step workload under large armies with no silent omissions | Same command stream, same entity count, same final checksum before/after optimization |
| Command response | Immediate UI acknowledgment and minimal local command-to-simulation delay | Input timestamps through simulation and displayed response |
| Presentation | High-fidelity PBR, directional lighting, expressive animation, world-class VFX and audio | Art review and playtest evidence against a curated reference scene suite |
| Scale | Thousands of units as an engineering target, not a shipped claim or limit | Several population tiers plus pathological clustering and navigation cases |
| Loading | Seamless region transitions and prepared shaders in gameplay | Cold/warm runs, hitch traces, memory/VRAM and I/O budgets |
| Reliability | Reproducible replays, validated saves, recoverable corruption, graceful device loss | Fuzz/property tests, cross-platform replay fixtures, fault injection |
| Accessibility | Rebindable inputs, scalable interface, color-independent telegraphs, reduced flashes/motion | Keyboard/controller review, contrast and readability playtests |
| Extensibility | A second distinct sample using engine crates without game rules | Separate consumer package, build, tests, and runtime |

Targets will be specified per hardware tier after measurement. No FPS figure in this table is a
measured result from the first build.

## First encounter: The Broken Concord

The player leads a commander, vanguard, and ranged squads through a ruined battlefield. Capture
three relic sites while surviving five enemy waves. Captured ground supports the economy. The
commander's area ability can break an engagement, but its cooldown demands timing. Recruitment,
positioning, target selection, and preserving the commander decide the outcome.

The initial geometric environment is a functional gameplay and rendering test. Production characters,
animation, sound design, loot systems, campaign, matchmaking, and advanced rendering remain explicit
milestones. They must not be presented as implemented merely because a dependency supports them.

## Engine independence

`dread-core` must compile without Bevy and without Dreadmarch factions or classes. `dread-sim`
contains this game's rules and is shared by native and browser clients. Rendering never becomes
authoritative game state. A separate pathfinding sample demonstrates independent core consumption.

## Scope expansion order

Protect a complete play loop first; then deepen it. Add crafted animation and audio; robust
navigation/collision; data-driven abilities and loot; content tooling; network architecture;
then larger world systems and production breadth. Work can run in parallel where boundaries are
stable. No arbitrary release date substitutes for the quality gates.
