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
- 15 simulation tests pass, including stored-region path/map corruption rejection. Targeted strict
  Clippy passes. Core, WASM and independent engine tests passed earlier in this implementation wave.
- 14 packaged campaign control checks pass against actual JS and embedded Rust WASM with structural
  DOM/Canvas mocks. They cover creator, appearance, movement, panels, skills, independent save slots,
  resume/import and hidden-tab pause. These mocks do not establish actual browser visual quality.
- Legacy WASM and all 20 legacy packaged control regressions still pass.

## Work underway before this checkpoint is a release

- Native creator/campaign UI and current-source native runtime verification.
- Full research atlas coverage reconciliation and source inventory publication.
- Actual browser visual/input review of the new campaign.
- Current-source cross-platform CI, release packaging and complete artifact readback.

## Production work remains

This is a playable procedural prologue and engine foundation. It is not a massive finished campaign,
finished production art, all source abilities implemented, or demonstrated AAAA quality. Rigged original
characters, faces, authored animation, world art, audio, boss design, advanced itemization, dialogue
choices, controller/direct movement, accessibility and the extensive campaign remain roadmap work.

The previous release's evidence is preserved in [HISTORY_ALPHA1.md](HISTORY_ALPHA1.md). Older native
binaries must not be described as containing the new RPG implementation.
