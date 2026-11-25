# Direvanity: verified development checkpoint

25 November 2026 (America/Denver) · [canonical repository](https://github.com/Herbertofury/Dreadmarch) · [play the deployed campaign](https://herbertofury.github.io/Dreadmarch/)

## Published source and playable site

[PR #4](https://github.com/Herbertofury/Dreadmarch/pull/4) is merged. The implementation was verified at
commit [`40deec9f9108f3494703e3f0e5c994ef63a191e0`](https://github.com/Herbertofury/Dreadmarch/commit/40deec9f9108f3494703e3f0e5c994ef63a191e0).
Merge commit [`c7a76ab62cb6c0adfd00703bd80217ea60dd5b3d`](https://github.com/Herbertofury/Dreadmarch/commit/c7a76ab62cb6c0adfd00703bd80217ea60dd5b3d)
has the same source tree, `a5591ad20f1131d87fe48a26efb5b3a9349411a8`.

[CI run 45708059241](https://github.com/Herbertofury/Dreadmarch/actions/runs/45708059241) passed all three
jobs. [Pages run 45708530418](https://github.com/Herbertofury/Dreadmarch/actions/runs/45708530418) built
and deployed the merge revision successfully. The public page displays Direvanity and was exercised
through its actual browser controls.

[Optimized release build 45708000312](https://github.com/Herbertofury/Dreadmarch/actions/runs/45708000312)
also passed. Both native packages have been downloaded and verified, and the freshly extracted Linux
binary has passed independent renderer/runtime checks. The
[`v0.2.0-alpha.1` prerelease](https://github.com/Herbertofury/Dreadmarch/releases/tag/v0.2.0-alpha.1)
is published as release `458757949`, with its tag pointing to implementation commit
`40deec9f9108f3494703e3f0e5c994ef63a191e0`. It is public, marked prerelease, and not a draft. All six
uploaded asset sizes and provider SHA-256 digests match the verified local bytes; the exact publication
receipt is `dist/qa/release-publication.json`.

## Playable content and production boundary

The game is now **Direvanity**. The GitHub repository address remains `Herbertofury/Dreadmarch`.
The current implementation contains two playable chapter loops, six fixed story regions, four side
quests and one repeatable expedition region. All 19 classes and 174 original prototype skill nodes
remain available. The legacy skirmish remains a separate regression scenario.

- Named class resources have actual capacities, recovery cadences and kill returns. Animated life
  and class-resource reservoirs, cooldowns, affordability, XP and skill points read the shared Rust rules.
- The browser creator, journal, quest tracker, inventory search/sort/comparisons, minimap and
  overworld-first atlas are functional. Native presentation includes GPU-shader orbs, a complete
  active-skill deck and full scrollable inventory. Both clients preserve pause around panels.
- **A Crown for the Nameless** continues the Ashen Concord story through Ember Coast, Glasswood and
  Drowned Archive. A permanent shelter/release oath gives protection or replenishes class resource
  when entering hostile regions. The choice also changes dialogue.
- The Last Ferry, A Garden of Voices and Names Beneath the Water join the original lost-satchel
  quest. Their progress and rewards persist.
- Shattered Concord expeditions rotate hunt/seal/purge objectives and wrath/haste/drought modifiers,
  vary seeded layouts/encounters, increase depth, preserve unfinished runs and protect unique rewards.
  Unclaimed drops remain recoverable, including drops earned after an early hunt reward.
- Existing version-2 saves migrate additively. Browser character storage keys remain compatible;
  native saves fall back to legacy filenames without overwriting the old file. Active expedition
  saves retain their stored seed and modifier. Reduced motion persists across browser reloads.

The production catalogue defines **12 acts, 36 regional groups, 48 faction arcs, eight factions and
eight endgame disciplines**. Its 134–182 campaign hours, 108–168 optional-story hours and 144–246
first-mastery endgame hours are **unvalidated planning estimates**. They are not delivered or measured
playtime. Procedural repetition and simulation tick counts do not substantiate authored-hour claims.
Finished AAA art, animation, audio, encounters and narrative scale remain production work. See
[DIREVANITY_PRODUCTION.md](DIREVANITY_PRODUCTION.md) for the authoring and quality gates.

## Automated verification

| Gate | Observed result |
| --- | --- |
| Workspace Rust tests | 45 passed: core 6, simulation 27, WASM 4, independent engine consumer 1, native 7 |
| Formatting and strict workspace/all-target Clippy | Passed |
| Full real-WASM campaign sweep | All 19 classes completed the prologue and additional chapter |
| Side-quest coverage | 76 completions, all four quests for each class |
| Expedition coverage | 57 completions; each of three objectives and three modifiers completed 19 times |
| Oath branches | Shelter/ward 10 characters; release 9 characters |
| Progression and travel | Every class finished at level 17 and visited seven regions |
| Aggregate campaign execution | 259,620 ticks, already including 52,320 prologue ticks |
| Packaged browser campaign controls | 23 cases passed against real WASM with DOM/Canvas mocks |
| Packaged legacy browser controls | 20 cases passed; legacy WASM ABI also passed |
| Supplemental modifier combat | Three separate earned-save Paladin runs exercised wrath, haste and drought |
| Numeric-boundary saves | Valid extreme progression survives commands; malformed progression rejects atomically |
| Production catalogue | Unique IDs, acyclic prerequisites, finite positive budgets and honest unmeasured labels; three validator tests passed |
| Legacy battle/replay/save and independent engine consumer | Passed |

These results include class-resource identity, save migration, gated-road atomicity, irreversible
oaths, crossing benefits, reachability, scaled saved enemies, expedition resumption, reward uniqueness
and late-drop recovery. The final challenge repaired numeric-boundary XP/skill-point saves,
nonfinite planning targets and repetitive modifier selection.

The completed-chapter, active-expedition and completed-expedition fixture saves were earned through
public commands. Their exact import behavior is included in the 23-case structural suite. The
numeric-boundary test is separately identified because it deliberately imports a valid synthetic
extreme-value fixture.

Build-specific hashes are retained separately: the final local validation WASM SHA-256 is
`dada0101efb8415504a5c12f04df80c87383c4331934160dfcd453016a26f2dd`; release-CI WASM SHA-256 is
`a77b2c868d80b5d8c22fdd079cb8111153666f496eec163d63659370217d6026`. Each belongs to its own build receipt.

## Live browser review

The deployed browser build passed hands-on checks of class selection and its Fury/Mana resource
presentation; actual casting reduced Witch Doctor Mana from 130 to 121 while the cooldown remained
visible. A newly learned Wicker Companion appeared in the world. Manual pause survived opening and
closing the skill panel; the map displayed the overworld before its real local terrain and destinations.
Map navigation reached a cache, its weapon was recovered and equipped, and inventory search, clearing
that search and sorting behaved correctly. The journal displayed the canonical campaign, side and
endgame entries.

Saving and reloading restored the learned skill, equipped weapon and reduced-motion setting. Existing
characters remained available. Captures cover the creator, depleted-resource HUD, skill tree, inventory,
overworld and local map at `dist/qa/direvanity-*.jpg`.

**Remaining visual boundary:** the browser file-chooser operation for importing the late-campaign
fixture stalled and was interrupted. The completed-chapter and expedition imports therefore have
structural real-WASM proof, but their live-browser visual checks are not complete. Do not claim those
late-game screenshots or an entire manual campaign playthrough for this revision.

## Native packages and exact runtime proof

The packages below were built from implementation commit `40deec9f9108f3494703e3f0e5c994ef63a191e0`.
Both complete payloads, provider ZIP digests, inner ZIP digests and 25 bundled project data/documentation files
were verified. The packaged source payloads match across platforms. A later documentation closeout
does not change their binary provenance.

| Component | Linux x64 | Windows x64 |
| --- | --- | --- |
| Actions job | `139534824643` | `139534824307` |
| Actions artifact | `15065184844` | `15067410496` |
| Native package bytes | 29,938,447 | 25,701,475 |
| Executable bytes | 114,073,736 | 77,807,104 |
| Executable | `direvanity` | `direvanity.exe` |
| Runtime boundary | Fresh extracted executable exercised under software Vulkan; glibc requirement 2.35 | Build/tests and complete package verified; Windows graphics runtime not exercised |

| Payload | SHA-256 |
| --- | --- |
| Linux package | `99d6a69077b16f63b29e017f0ff3c911d17b907280d086c44ecfa5379a58f3a8` |
| Linux binary | `4c0a0310ff519e6aefb198a4a964f0d2c54f32f7608d4a112133bc103c512c1f` |
| Windows package | `9dc763ec6e5e834f18c333ed3aed52cd1b754740ad13bd6ad8626701b25f67ec` |
| Windows binary | `170cf8c39352c331d990735d0984186388def3d9399b37c9c4b22019bc8e1f60` |

The freshly extracted optimized Linux campaign exited 0 at **185 frames / tick 354**, with ten
successful commands, captured rendering, exact save roundtrip and a validated HUD. Its checksum
`6ca474ed5e8d0a66` matches the CI run. The separate legacy smoke exited 0 at 120 frames / tick 120,
with three commands and checksum `4d08544a95f6c631`. The executable's hash remained unchanged.
All three 1440 × 900 captures were reviewed, including the circular life/Conviction liquid, glass
highlights, gold rims, resource value, cooldown, creator and legacy scene.

This is distinct from the final development binary
`5d5ada367b84f71ddcb01ecf83dea4e80346431b2e8f8e798c0f2e1bf346e4cc`, which passed 184 campaign frames /
tick 353 and 120 legacy frames. Software Vulkan proves functional rendering and persistence; it does
not establish representative gaming-GPU frame rates or Windows interactive runtime quality.

Exact receipts are `dist/release-direvanity/release-validation.json`, each platform's `validation.json`,
and `dist/release-direvanity/extracted-runtime/optimized-runtime-proof.json`. Local development and
all-class details remain in `dist/qa/final-native-proof.json` and `dist/qa/final-campaign-summary.json`.

## Delivery and approval boundaries

The source, playable website and six prerelease assets are published. Remote asset metadata confirms
all six uploads and matches their exact local byte counts and provider SHA-256 digests.

| Published asset | Asset ID | Bytes |
| --- | --- | --- |
| `Direvanity-windows-x64.zip` | `793538333` | 25,701,475 |
| `Direvanity-linux-x64.zip` | `793538390` | 29,938,447 |
| `Direvanity-campaign.html` | `793538499` | 1,300,225 |
| `Direvanity-playtest.html` | `793538513` | 1,248,392 |
| `Direvanity-windows-x64.sha256` | `793538463` | 94 |
| `Direvanity-linux-x64.sha256` | `793538483` | 91 |

The released offline campaign SHA-256 is
`63447084e8034ca8080c842893a5fc447f9984aad4f5fc8e00d784bbf280dd38`; the released legacy browser file is
`7a0b2ce3c7a934f8581c159b92dbf7a064d31f5fafb729b01d6857fb1c396688`.

The complete source/build/evidence checkpoint is generated after this documentation commit as
`dist/Direvanity-checkpoint.zip`. Its adjacent `Direvanity-checkpoint.manifest.json` records the actual
archive identity; the archive's `CHECKPOINT.json` records every included payload. It includes complete
source, both verified platform packages, the exact released offline browser files, screenshots and
receipts. This separate checkpoint is not one of the six published release assets.

Automatic approval review rejected copying the complete project archive to Google Drive because the
specific archive payload and destination were not explicitly authorized in user-authored text. No
successful Drive transfer or readback is claimed. Approval review also rejected renaming the GitHub
repository URL: renaming the game was authorized, but changing the repository address was not
explicitly authorized. Those two actions require specific user approval before another attempt.

## Next production milestone

Bring Hearthmere → Hollow Moor → Forgotten Sanctum to one complete production-quality region:
original rigged hero/enemy models and equipment, authored environment art, typed attack timelines and
real projectile/chain delivery, expressive movement/hit reactions, layered audio/VFX, branching NPC
dialogue, differentiated enemy ecology and a multi-phase Sanctum keeper encounter. Extend the class
resource foundation into signature generators and behaviorally different builds. Prove controller,
rebindable input, accessibility and Windows/Linux hardware input/frame pacing before scaling authoring
across the 12 planned acts. Keep the new chapter and expeditions functional throughout.

Earlier releases and their independent evidence remain in [HISTORY_ALPHA2.md](HISTORY_ALPHA2.md) and
[HISTORY_ALPHA1.md](HISTORY_ALPHA1.md).
