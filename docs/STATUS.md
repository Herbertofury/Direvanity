# Verified RPG checkpoint

Dreadmarch · https://github.com/Herbertofury/Dreadmarch · 25 November 2026 (America/Denver)

## Current direction and scope

A character-led, offline, single-player action RPG is the primary game. Original Divine Divinity
(2002), the complete Diablo series, Lost Ark, Path of Exile and Path of Exile II are explicit inspirations.
The earlier RTS encounter remains an optional regression scenario. See [RPG_DIRECTION.md](RPG_DIRECTION.md).

The delivered content is a playable procedural prologue and reusable engine foundation. A massive
finished campaign, production character art, all researched mechanics implemented, and demonstrated
AAAA quality remain production goals. [CLASS_SYNTHESIS.md](CLASS_SYNTHESIS.md) states the simplified
prototype behaviors and the contracts needed to replace them with full class combat systems.

## Implemented

- All 19 distinct researched Diablo class identities in both creators, with independent body choices.
- 174 original prototype nodes across 58 branches: three branches per class, four for Spiritborn.
- Persistent name, silhouette, six hair styles, skin/hair/eye/cloth colors and stature.
- Shared Rust campaign rules: prerequisites, skill learning, cooldowns, focus, 11 effect families,
  XP/levels, equipment, loot, dialogue, main/side quests and persistent region travel.
- Hearthmere, Hollow Moor and Forgotten Sanctum form a complete main-quest and side-quest prologue.
- Persistent defeated enemies and uncollected loot, one-time rewards and portable exact JSON saves.
- Native Bevy creator, campaign UI, movement, casting, learning, interaction, inventory and save/load.
- Browser character slots, save/resume, import/export, contained panels and hidden-tab pause.

## Source and automated verification

The Rust/native source checkpoint is
[`a96d473d834dac66b0c0f0977300ed6dca45899e`](https://github.com/Herbertofury/Dreadmarch/commit/a96d473d834dac66b0c0f0977300ed6dca45899e).
[CI run 45657960569](https://github.com/Herbertofury/Dreadmarch/actions/runs/45657960569) passed every job.
Later documentation, checkpoint packaging and a browser objective-text correction do not change the Rust/native source.

| Gate | Observed result |
| --- | --- |
| Core, simulation, WASM and independent consumer tests | 27 passed |
| Native regression tests | 4 passed on Linux and Windows |
| Format and strict Clippy | Passed, including all native targets |
| Complete campaign through every class | 19 real public-command playthroughs; 52,530 simulated ticks; both quests completed |
| Packaged campaign controls | 16 checks passed with actual JS/WASM and structural DOM/Canvas mocks |
| Legacy packaged controls | 20 checks passed; legacy WASM integration also passed |
| Legacy full battle, replay/save, independent engine consumer | Passed in final-source CI |

Campaign checks exercise real movement/combat/interaction, prerequisites, reward uniqueness,
equipment, persistent enemies/loot, exact cross-instance save restoration and invalid-save atomicity.
The challenge pass fixed item/unit allocation exhaustion before reward/travel mutations, currency
overflow, stored-region navigation validation, modal transitions, native movement throttling after
clock rollback, defeated-save camera initialization and presentation scheduling after travel.

## Actual runtime and visual evidence

- Native campaign: 184 frames, 10 command checks, exact save roundtrip and validated HUD; checksum
  `b31fcf2df3844452`. Creator, class choice, quest, cache/equipment, skill and travel were exercised.
- Native legacy tactical mode: 120 frames, three command checks, exact save roundtrip and validated
  HUD; checksum `4d08544a95f6c631`. The receipt records the success marker without inventing a lost
  final process-exit response.
- Native captures are 1440 × 900 and were visually reviewed. Current proof is in
  `native-campaign-receipt.json`, `native-campaign-smoke.log`, `native-rts-regression.log` and their PNGs.
- The native environment used Vulkan through Mesa Lavapipe. This proves functional rendering and
  command/UI behavior; it does not establish performance on a gaming GPU or a real Windows window.
- A complete live browser Witch Doctor playthrough reached level 6, defeated all ten region enemies,
  completed both quests, earned both unique rewards and returned to Hearthmere. Movement, companion
  casting, learning, loot/equipment, region persistence and quest turn-ins used actual controls. The
  exported save and `rpg-live-playthrough.json` preserve this evidence. Earlier review also verified
  customization, retraining, save restoration and independent Paladin/Witch Doctor slots.
- [Pages run 45657139297](https://github.com/Herbertofury/Dreadmarch/actions/runs/45657139297) deployed
  source `1558e58`. Browser source is unchanged in `a96d473`; live review verified modal focus
  wrapping, inert background, Escape focus restoration, prior pause preservation and creator/resume.
  [Pages run 45660608035](https://github.com/Herbertofury/Dreadmarch/actions/runs/45660608035) then
  deployed `f1e5271`, including the objective-text correction. Both saved character slots survived;
  the level-6 Witch Doctor restored with both quests complete, 289 coins, ten kills and four items.
  The corrected journal was visually reviewed; packaged campaign controls still pass.
- Structural DOM/Canvas tests establish control behavior, not visual quality. Actual browser review
  and native rendered captures are recorded separately.

## Research provenance and coverage

All three complete JSON catalogues and readable atlases are published. File SHA-256 values and Git
blob identities are recorded in `research-validation.json`. Exact counts, edition boundaries and
unknown denominators remain visible in each atlas. Divinity contains 1,684 typed edition-specific
records. Research entries are not counted as implemented Dreadmarch abilities.

## Release and complete artifact

Release build run [45657992875](https://github.com/Herbertofury/Dreadmarch/actions/runs/45657992875)
builds optimized Windows/Linux packages from `a96d473`. The Linux job passed and its exact ZIP
was downloaded through the GitHub artifact connector, integrity checked and safely extracted.
All 25 packaged files were hashed; the three research catalogues match the worktree and both source
revisions. The Linux binary retains executable mode `0755` and requires glibc 2.35 or later.

The extracted optimized Linux executable exited **0** after the independent campaign smoke: 185
frames, tick 354, ten command checks, exact save roundtrip, HUD validation and captured images.
Checksum `21debfd8d94ba33c` matches the optimized CI campaign result. Both 1440 × 900 captures were
visually reviewed. `rpg-extracted-native-receipt.json` preserves package and binary hashes, every
extracted file hash, CI smoke results and the independent run. Development, CI and extracted-package
evidence remain separate. The Windows job also passed its tests and optimized build. Its x86-64 PE
executable is 76,911,616 bytes, SHA-256 `ebcf6ca79a6f4c87b872c02d07ecd47da9e6411a12c614914f8cd7fe46683349`.
Windows checkout had converted 22 tracked text files to CRLF; parsed JSON data was identical. Those
files were restored to the exact `a96d473` Git blobs while retaining the tested binary unchanged.
The package receipt records both original CI and distribution identities. Repository attributes now
keep future text checkouts at LF. Both packages contain all three exact research catalogues.

Distribution package SHA-256 values:

- Linux: `ba76dcd5f8ad49af9bb1c81b5a705ede3eecd3699860a24365bd028a44969b43`
- Windows: `60a76ac1aaef2ef6d6010b4a152445647f1346bed44e1ed0ed380524d0c4c7f5`

The public [v0.1.0-alpha.2 release](https://github.com/Herbertofury/Dreadmarch/releases/tag/v0.1.0-alpha.2)
was published on 25 November 2026 from tag commit `e4a6e9c971ead55ecc2569fb1d931f33a3522eb6`.
All 17 initial release assets were read back through GitHub metadata and matched to their local
byte counts and SHA-256 digests. The exact map is preserved in `public-release-assets.json`.

The complete checkpoint format includes the clean Git source, both offline browser clients, both
platform packages, all research catalogues and the separate development/CI/extracted runtime proof.
`CHECKPOINT.json` identifies the source revision and every payload hash, size and permission. A
separate verifier checks the archive against Git blobs and nested native-package receipts.

Complete-file backup readback remains a mandatory delivery gate. Its observed result and exact
archive SHA-256 are recorded in the release asset `Dreadmarch-delivery.json`, which is generated
after the archive is transferred. This post-transfer receipt is intentionally outside the archive
whose identity it verifies. The final delivery must not claim success without that receipt.

## Production work remains

Original rigged characters, faces, authored animation, world art, audio, boss design, differentiated
class resources, real projectile/chain delivery, advanced itemization, branching dialogue, controller
support, accessibility and the extensive campaign remain roadmap work. Native WASD/click movement
is implemented; production controller and rebinding support is not yet complete.

The earlier milestone's evidence is preserved in [HISTORY_ALPHA1.md](HISTORY_ALPHA1.md). Alpha.1 native
binaries do not contain this RPG implementation. Dependency notices retain the two disclosed upstream
text exceptions in [THIRD_PARTY.md](../THIRD_PARTY.md); the notice bundle does not claim those resolved.
