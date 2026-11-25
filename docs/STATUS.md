# Direvanity development checkpoint

25 November 2026 (America/Denver) · [canonical repository](https://github.com/Herbertofury/Dreadmarch)

## Product and production boundary

The game is now **Direvanity**. This checkpoint expands the shared Rust RPG and both clients.
It contains two playable chapter loops, six story regions, four side quests and a repeatable
expedition region. It does not contain hundreds of authored hours or finished AAA production art.

[DIREVANITY_PRODUCTION.md](DIREVANITY_PRODUCTION.md) records the accepted 100+ hour campaign,
100+ hour optional-story and 100+ hour endgame direction. The production catalogue defines 12 acts,
36 regional groups, 48 faction arcs and eight endgame disciplines with unvalidated planning budgets.
`validate_content_plan.py` checks those estimates and prevents them being certified as shipped hours.

## Implemented in this iteration

- Direvanity creator, HUD, package and native executable branding; original repository address retained.
- All 19 class resource profiles with named reservoirs, capacities, recovery cadence and kill returns.
- Animated life/class resource presentation, cooldown/affordability hotbars, XP and reduced motion.
- Canonical Rust journal and world atlas shared by the native and browser clients.
- Browser inventory search/sort/comparison, real minimap, overworld-first map and destination actions.
- Native full scrollable inventory, journal/atlas, complete active-skill deck and preserved panel pause.
- Ember Coast, Glasswood and Drowned Archive extend the original three persistent story regions.
- A Crown for the Nameless chapter, permanent shelter/release oath and three additional side quests.
- Shattered Concord expeditions: hunt/seal/purge objectives, wrath/haste/drought modifiers, seeded
  terrain/encounters, rising depth, recoverable drops, one-time rewards and exact return/resume.
- Additive migration of old version-2 saves; existing browser character keys preserved; native legacy
  default-save fallback keeps the original file intact.
- Hotbar node reuse and cached static minimap geometry; no item, class or content availability caps.

## Current observed checks

| Gate | Observed result |
| --- | --- |
| Non-native workspace unit tests | 38 passed: core 6, simulation 27, WASM 4, independent consumer 1 |
| Native unit tests | 7 passed, including final native orb shader correction |
| Format and strict workspace Clippy | Passed on the integration checkpoint |
| All-class real WASM playthroughs | 19 classes, both chapters, all four side quests, 57 expeditions, 255,450 simulated ticks |
| Browser campaign controls against actual WASM | 23 cases passed; DOM/Canvas structural mocks, separate live visual review required |
| Browser legacy controls | 20 cases passed; actual WASM integration also passed |
| Legacy battle/replay/save and independent engine consumer | Passed |
| Planning catalogue | Valid unique IDs, acyclic prerequisites, positive ranges and honest unmeasured-duration labels |

The class-resource identity test binds all 19 profiles by class ID. Additional regression tests cover
legacy migration, invalid extensions, gated-road atomicity, irreversible oaths, distinct crossing
benefits, every point's reachability, scaled saved enemies, expedition resumption and reward uniqueness.
An early hunt reward followed by more combat cannot discard later drops when the next run begins.

The final shared WASM at this checkpoint has SHA-256
`dada0101efb8415504a5c12f04df80c87383c4331934160dfcd453016a26f2dd`.
Actual command-playthrough fixtures preserve completed chapter, active expedition and completed
expedition saves without editing HP, inventory, XP or quest state. Browser controls import each exactly.

## Runtime and publication evidence

The actual native campaign passed 184 frames, ten command checks, exact save restoration and HUD
validation under Mesa Lavapipe. Visual review found rectangular reservoir fills; the corrected embedded
GPU shader passed a fresh render with true circular liquid, glass highlights and no overflow.
The final-shader legacy skirmish also passed 120 frames and three commands. All three native captures
were reviewed at 1440 × 900. These UI-candidate results precede the subsequent extreme-save hardening;
the final optimized package is verified separately. Software Vulkan establishes
functional rendering, not dedicated-GPU frame rates or Windows interactive runtime proof.

Final optimized native rendering, public browser review, source commit, CI, platform package hashes and
verified delivery identities are recorded at the final publication checkpoint. These gates remain active
until their exact results are appended; a successful structural test is not visual acceptance.

## Next production milestone

Finish one production-quality region: original rigged characters and equipment, authored environment
art, typed attack timelines and real projectile/chain delivery, expressive movement/reactions, sound,
NPC dialogue choices, differentiated enemy ecology and a multi-phase boss. Extend the new class
resource foundation into signature generators and behaviorally different builds. Prove controller,
rebindable input and Windows/Linux hardware frame pacing before scaling authoring across the 12 acts.

The exact preceding release and its independent evidence remain in [HISTORY_ALPHA2.md](HISTORY_ALPHA2.md).

The final challenge also fixed numeric-boundary XP/skill-point saves, nonfinite content-plan targets,
and repetitive expedition modifiers. Dedicated command tests completed hunt/wrath, seals/haste and
purge/drought from an actual earned character save. Existing active expedition saves retain their
stored seed and modifier. The final full class sweep is rerun by CI on the release source.
