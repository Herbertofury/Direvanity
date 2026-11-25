# Direvanity: the ambition

An original, massive, character-led single-player action RPG and a reusable Rust engine.
Founded 24 November 2026; RPG direction accepted 25 November 2026 (America/Denver).
These are production targets, not claims of achieved quality.

## The promise

Begin with a person you created, not a predetermined army. Choose a recognizable class identity, then
shape it through skills, equipment, discoveries and decisions. Explore an interconnected world whose
people, secrets, routes and consequences matter. Combat should be expressive enough for a brilliant
moment-to-moment action game; the world should be deep enough to remember after the loot is gone.

The entire main campaign must work offline, be completable solo, support pause, and preserve the world
through local saves. No multiplayer service, daily attendance or competitive economy is required to
complete the story or develop a satisfying build. Optional future modes must respect that foundation.

## Great inspirations

| Inspiration | What we study | Original Direvanity direction |
| --- | --- | --- |
| Divine Divinity (2002) | A continuous, explorable role-playing world, secrets, quests and freedom across skill Ways | Authored towns and wilderness, reactive quests, useful noncombat talents and persistent discoveries |
| Diablo I / Hellfire | Atmosphere, danger, readable archetypes and shared magical knowledge | Distinct legacy identities, purposeful exploration and consequential encounters |
| Diablo II / Resurrected | Character trees, class silhouettes, item interactions and build identity | Branch prerequisites and legible build choices without collapsing related classes |
| Diablo III / Reaper of Souls | Strong skill variants, class fantasy, impact and readability | Original skill transformations that change decisions and behavior |
| Diablo IV | Grounded dark-fantasy materials, animation and world cohesion | Original high-fidelity characters, regional identity and coherent audiovisual direction |
| Diablo Immortal | Additional class identities and transformation mechanics | Adapt mechanics into an offline solo progression model |
| Lost Ark | Responsive isometric combat, class identity systems and encounter choreography | Expressive actions and solo-readable bosses without MMO progression gates |
| Path of Exile | Skill composition, itemization, passive structure and experimentation | Typed ability modifiers and deep equipment interactions with explainable rules |
| Path of Exile II | Deliberate action, encounter execution and animation commitment | Clear anticipation, counterplay and deliberate build tradeoffs |
| Later Divinity games | Surfaces, hybrid skills, dialogue consequences and systemic interactions | Select compatible ideas while preserving the real-time single-player action focus |

The [research atlases](research/) record versions, exact inventories and uncertain coverage. These
credits do not imply feature parity or reuse of proprietary code, narrative or assets.

## Product pillars

1. **A real role-playing world.** Exploration, NPCs, secrets, readable navigation, dialogue decisions,
   quest consequences and authored dungeons are core production work.
2. **Every class has an identity.** Preserve all 19 researched Diablo identities, including Warrior,
   Bard, Witch Doctor and the distinct Sorcerer/Wizard and Paladin/Crusader traditions.
3. **Combat with weight and agency.** Immediate input acknowledgement, deliberate commitments,
   reliable targeting, convincing reactions, layered VFX/audio and readable enemy intent.
4. **Builds change how you play.** Active skills, variants, passives, resources and equipment interact
   through explicit typed rules. Different names on identical damage circles are insufficient.
5. **A character you own.** A substantial creator with unrestricted class/body combinations, persistent
   appearance, equipment silhouettes, accessible controls, multiple saves and recoverable progress.
6. **A reusable engine.** Generic math, navigation, scheduling, assets, rendering, input, persistence,
   authoring and profiling remain separable from this world and its rules.
7. **Quality and scale together.** Optimization must preserve actual content and simulation behavior.
   Measure frame pacing, memory, loading, input response and worst-case navigation; do not hide work
   by silently dropping entities, skills, quests or catalogue rows.

## Extremely high quality bar

“AAAA” expresses our ambition. Surpassing the inspirations requires comparable finished encounters,
art, animation, audio, usability, build diversity, stability and measured player experience.

| Dimension | Production target | Required evidence |
| --- | --- | --- |
| Combat | Distinct class resources, skill delivery and tactical decisions; authored bosses | Blind playtests, encounter review, input/animation traces |
| Campaign | Complete multi-act narrative, large persistent world, optional quests and secrets | End-to-end save-compatible playthroughs and content coverage audit |
| Character art | Original high-fidelity bodies, faces, hair, armor and animation | Rig/LOD/material/animation review across the creator matrix |
| Rendering | Cohesive PBR, lighting, shadows, atmospheric effects and exceptional VFX | Identical-scene comparisons, cross-vendor review, hardware captures |
| Frame pacing | Explicit 60/120/144 Hz targets plus uncapped mode on suitable hardware | Representative shipping scenes, p50/p95/p99 and exact settings |
| Streaming | Prepared shaders and smooth traversal across authored regions | Cold/warm traces, I/O, memory and VRAM measurements |
| Reliability | Validated/migrated saves, deterministic game rules, crash recovery | Fault injection, long campaigns, cross-platform and corruption tests |
| Accessibility | Rebinding, controller support, UI scale, color-independent cues, reduced effects | Input/readability/contrast testing and user review |
| Reuse | A second game can use engine services without Direvanity content | Separate package, build, tests and runtime proof |

The current three-region procedural prologue establishes a real loop. Production models, advanced
animation, extensive narrative content, deep itemization and the measured quality targets remain work.
