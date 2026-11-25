# Verified RPG checkpoint

Dreadmarch · https://github.com/Herbertofury/Dreadmarch · 25 November 2026 (America/Denver)

## Current direction

A character-led, offline, single-player action RPG is now the primary game. Original Divine Divinity
(2002), the complete Diablo series, Lost Ark, Path of Exile and Path of Exile II are explicit inspirations.
The first RTS encounter remains an optional regression scenario. See [RPG_DIRECTION.md](RPG_DIRECTION.md).

## Verified in the current working implementation

- All 19 distinct researched Diablo class identities have creator choices and original adaptation trees.
- 174 original nodes across the class catalogue: 3 branches per class, 4 for Spiritborn. These are a
  prototype design, separate from the thousands of inventoried source-game abilities and tree records.
- Persistent name, silhouette, six hair styles, skin/hair/eye/cloth colors and stature.
- Shared Rust campaign rules: skill learning, prerequisites, cooldowns, focus, 11 effect families,
  XP/levels, equipment, loot, dialogue, main/side quest state and persistent region travel.
- Three connected regions: Hearthmere, Hollow Moor and Forgotten Sanctum. A complete prologue loop.
- Every class completed both quests through the public WASM command interface, using actual combat,
  movement and interactions: 19 playthroughs and 52,530 simulated ticks. No stat/world edits were used.
- Persistent killed enemies and uncollected loot; reward uniqueness; byte-exact full save roundtrips.
- 16 simulation tests pass, including stored-region path/map corruption rejection. Targeted strict
  Clippy passes. Core, WASM and independent engine tests passed earlier in this implementation wave.
- 16 packaged campaign control checks pass against actual JS and embedded Rust WASM with structural
  DOM/Canvas mocks. They cover creator, appearance, movement, panels, skills, independent save slots,
  resume/import, hidden-tab pause, modal transition integrity and defeated-save recovery. These mocks do not establish actual browser visual quality.
- Legacy WASM and all 20 legacy packaged control regressions still pass.

## Additional observed verification

- Native creator/campaign source compiled successfully; all four native regressions passed, including
  creator button selection, independent appearance, real skill learning and portable campaign saves.
- The live browser campaign was visually inspected and exercised through actual controls: customization,
  Witch Doctor selection, movement, accepting the main quest, learning Spirit Harvest/Wicker Companion,
  save, retraining and restoration. A second Paladin slot preserved the first character and its progress.
- Initial cross-platform CI run `45653298115` passed all jobs on source `718fcf8`; Pages deployment
  `45653476481` passed. The later review fixes and native integration require their own final release gate.
- Source checkpoint `4978a61` publishes all three research datasets and atlases. Uploaded Git blob hashes
  matched the complete local files. Divinity contains 1,684 typed records; other exact counts are in
  their atlases. No missing-denominator or current-build uncertainty is silently marked complete.
- Challenge fixes preflight item and encounter identity allocation, saturate currency rewards, validate
  stored-region navigation, and reset/contain modals across load/creator transitions.

## Work underway before this checkpoint is a release

- Current-source native campaign/creator render verification and optimized release packages.
- Final exact-source cross-platform CI after review fixes.
- Final modal-focus browser regression after deployment of the hardened client.
- Current-source cross-platform CI, release packaging and complete artifact readback.

## Production work remains

This is a playable procedural prologue and engine foundation. It is not a massive finished campaign,
finished production art, all source abilities implemented, or demonstrated AAAA quality. Rigged original
characters, faces, authored animation, world art, audio, boss design, advanced itemization, dialogue
choices, controller/direct movement, accessibility and the extensive campaign remain roadmap work.

The previous release's evidence is preserved in [HISTORY_ALPHA1.md](HISTORY_ALPHA1.md). Older native
binaries must not be described as containing the new RPG implementation.
