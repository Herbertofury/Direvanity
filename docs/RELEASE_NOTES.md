# Dreadmarch v0.1.0-alpha.2 — The Ashen Concord

The project now centers on a character-led single-player action RPG, with original Divine Divinity
(2002), the Diablo series, Lost Ark, Path of Exile and Path of Exile II explicitly credited as inspirations.

## Playable RPG foundation

- Character creator and all 19 researched class identities, with independent body/appearance choices.
- 174 original prototype skill nodes, prerequisites, cooldown/focus costs, XP/levels and retraining.
- Three connected regions, NPC dialogue, complete main and side quests, equipment and persistent loot.
- Native Bevy campaign interface and offline browser campaign using the same Rust simulation.
- Browser character slots, save/resume and portable JSON save export/import; native `--load PATH`.
- The prior RTS skirmish remains available separately through `--skirmish` and the legacy HTML.

## Research and original design

Complete versioned input-table inventories and readable atlases are included for Diablo, Divinity,
Lost Ark and Path of Exile. Divinity contains 1,684 typed edition-specific records. Original Divine
Divinity is reconciled as 88 ordinary +8 Divine tree skills, plus3 innate class specials. The atlases
record exact scope, source versions, unknown denominators and unverified patch-specific data.

The playable 174-node catalogue is an original prototype. It does not claim that every source-game
ability, rune, passive, modifier or skill behavior is implemented. Production class resources,
projectiles, transformations, authored animations, original world art, extensive narrative content,
advanced itemization and the full massive campaign remain planned work.

## Verification

Every class completes both prologue quests through actual public Rust/WASM commands. Tests cover
persistent enemies/loot, one-time rewards, prerequisites, equipment, exact saves, invalid-save atomicity,
allocation exhaustion, currency overflow and modal/save-screen transitions. The live browser creator,
quest interaction, skills, retraining, save restoration and independent character slots were exercised.

See `docs/STATUS.md` and the included build/runtime receipts for the exact source revisions and limits.
Native software-Vulkan captures establish functional rendering, not target gaming-GPU performance.
Windows packages require the normal Microsoft Visual C++ x64 runtime; Windows GPU/window behavior
must still be verified on real Windows hardware.
