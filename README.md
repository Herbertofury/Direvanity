# DIREVANITY

**A massive single-player action RPG in development. A reusable Rust engine beneath it.**

[**Play the campaign**](https://herbertofury.github.io/Dreadmarch/) ·
[Windows / Linux / offline downloads](https://github.com/Herbertofury/Dreadmarch/releases) ·
[Research atlases](docs/research/) · [Production roadmap](docs/ROADMAP.md)

Create a character, choose a class, explore an original dark-fantasy world, meet its people, discover
secrets and equipment, and develop a build through an authored campaign. The original **Divine Divinity
(2002)** is central to the world and role-playing direction. Our great inspirations also include the
**entire Diablo series, Diablo IV, Lost Ark, Path of Exile and Path of Exile II**.

The ambition is to meet and surpass the strongest qualities of these games through exceptional combat,
art direction, animation, systemic depth, exploration, responsiveness and polish. “AAAA” is our quality
destination. The current playable build contains two connected chapter loops, four side quests and repeatable
expeditions. Its original procedural art remains a prototype; the massive production campaign is in development.

## What is playable

- **19 class identities** and 174 original prototype skill nodes: Warrior, Rogue, Sorcerer/Sorceress,
  Monk, Bard, Barbarian, Amazon, Assassin, Necromancer, Paladin, Druid, Crusader, Demon Hunter,
  Witch Doctor, Wizard, Blood Knight, Tempest, Spiritborn and Warlock.
- **Actual class resources:** Fury, Conviction, Mana, Essence, Resonance and the other class profiles
  have distinct capacities, recovery rhythms and kill returns. Both clients read the same Rust rules.
- **Animated class-aware HUD:** life/resource reservoirs, learned-skill deck, cooldown and affordability
  feedback, XP progression, journal and an overworld-first atlas with local navigation.
- Character name, independent body silhouette, six hair styles, colors and stature with live preview.
- **Six connected story regions:** Hearthmere, Hollow Moor, Forgotten Sanctum, Ember Coast, Glasswood
  and Drowned Archive. Recover the Ashen Concord, choose its fate, then pursue the Crown of Names.
- A persistent **shelter / release** choice changes crossing benefits and chapter dialogue.
- **Four side quests:** the lost satchel, Oren's ferry bell, Vey's living seed and Edda's drowned ledger.
- **Shattered Concord expeditions:** seeded layouts and encounters, hunt/seal/purge objectives,
  wrath/haste/drought modifiers, escalating depth, earned relics, recoverable loot and exact save/resume.
- Persistent defeated enemies, open caches, uncollected loot, complete inventory and equipped items.
- Searchable/sortable browser inventory, comparison, free town retraining and reduced-motion controls.
- Independent browser character slots, portable exact JSON saves and legacy-save compatibility.
- A native Bevy 3D client and an entirely offline browser build using the **same Rust simulation**.

### The long campaign

The accepted direction now targets **100+ hours each of main campaign, optional stories and endgame**,
with no 100-hour ceiling. [The production contract](docs/DIREVANITY_PRODUCTION.md) defines 12 acts,
36 regional groups, 48 faction arcs and eight endgame disciplines. Its hour ranges are planning
estimates, **not shipped content or measured playtime**. The campaign content currently implemented
is the slice described above; production art, extensive authored stories and AAA quality remain goals.

### Reaching the new chapter

1. Finish the Ashen Concord quest and return the relic to Maelin in Hearthmere.
2. Choose an oath beside Maelin: shelter the living or release the bound souls.
3. Follow the northern road to Ember Coast, then the Glasswood and Drowned Archive. The journal
   tracks the beacon, orchard heart and Crown of Names, plus the optional local stories.
4. Return the crown to Maelin. Hearthmere's eastern rift opens the repeatable expedition system.
5. Complete the run's actual objective, claim its cache, and return through the western rift.
   Returning before completion preserves the encounter so you can resume it.

Current build and runtime evidence belongs in [STATUS.md](docs/STATUS.md). The older
[v0.1.0-alpha.2 release](https://github.com/Herbertofury/Dreadmarch/releases/tag/v0.1.0-alpha.2)
and its evidence are preserved in [HISTORY_ALPHA2.md](docs/HISTORY_ALPHA2.md). The original RTS remains
an [optional skirmish](https://herbertofury.github.io/Dreadmarch/skirmish.html) and regression fixture.

## Research, then original synthesis

The research preserves source-game identities, editions, patches, tree structure and coverage boundaries.
A record in an index is not automatically a complete behavioral specification. Source mechanics inform
original Direvanity abilities; commercial game code, dialogue, art, music and models are not bundled.

- [Diablo class and ability atlas](docs/research/DIABLO_CLASS_ATLAS.md)
- [Divinity series atlas, including original Divine Divinity](docs/research/DIVINITY_ATLAS.md)
- [Lost Ark / Path of Exile systems and class mapping](docs/research/ARPG_SYSTEMS.md)
- [Accepted RPG direction](docs/RPG_DIRECTION.md)
- [Original class synthesis and production rules](docs/CLASS_SYNTHESIS.md)

Research datasets live in `data/research/`. Playable original definitions live in
`data/gameplay/classes.json`. Keeping those domains separate makes actual implementation coverage visible.

## Run or build

The browser campaign is a single offline file, `Direvanity-campaign.html`. Open it in a modern desktop
browser; no proprietary asset download or server is required. Use the published play link or build:

```bash
git clone https://github.com/Herbertofury/Dreadmarch.git Direvanity
cd Direvanity
python3 scripts/build_web.py
# Open dist/Direvanity-campaign.html.
```

Install the [official Rust toolchain](https://rustup.rs/) first. The repository pins **Rust 1.100.0**,
**Bevy 0.20.1** and **wgpu 30.0.2** through the toolchain file and Cargo.lock. To build the native client:

```bash
cargo run --locked -p dreadmarch-client --release
```

Windows needs the normal MSVC build tools; Linux windowed builds need X11 development/runtime libraries
and graphics drivers. These are current dependencies, not evidence of finished production rendering.
The browser uses Canvas2D for an accessible simulation playtest; Bevy is the native rendering path.

### Browser campaign controls

| Action | Control |
| --- | --- |
| Move / approach / interact | Left click ground or a marked person, object or road |
| Cast selected class skill | Right click a target |
| Select a learned active skill | 1–9 or hotbar button |
| Skills / inventory / journal / world | K / I / J / M |
| Pause / resume | Space or Pause button |
| Close panel | Escape |
| Zoom | Mouse wheel |
| Save / load / export | Header buttons |
| Choose another character / import | Character creator |

Panels pause the world and preserve the previous pause state. Hidden tabs pause explicitly. Saves are
opaque UTF-8 from Rust; clients do not round-trip the 64-bit RNG through JavaScript Number.

### Native campaign controls

Launch `direvanity` (or `direvanity.exe`) to open the creator. Class and appearance buttons edit the
preview. Use **EDIT NAME**, type, and press Enter; Enter outside the name field begins the journey.

| Action | Control |
| --- | --- |
| Move | WASD relative to the camera, or click ground |
| Cast selected skill / cast a learned active | Space at cursor / 1–6 in tree order |
| Arm a targeted cast | Cast button, then click ground |
| Interact with nearest person, loot, cache or road | J |
| Skills / inventory | K / I |
| Previous / next skill or inventory item | [ / ] |
| Learn / equip / free town retraining | L / U / T |
| Orbit / zoom / center hero | Q and E / wheel / C |
| Pause / restart current character | P / R |
| Creator / save / load | F2 / F5 / F9 |
| Help / screenshot | F1 / F12 |
| Close panel or creator; then exit | Escape |

The default campaign save is `direvanity-campaign.json`. If it is absent, the native client can load
the legacy `dreadmarch-campaign.json` without overwriting it. Browser character storage retains its
legacy keys so a game rename does not erase your characters. Use `--save PATH` for a separate character
file or `--load PATH` to import a validated shared JSON save. `--class ID --name NAME` initializes an
editable creator draft; `--skirmish` launches the earlier tactical encounter. Browser multi-character
slots and native save-file paths are separate interfaces to the same portable save format.

## Reusable engine

| Package | Responsibility |
| --- | --- |
| `dread-core` | Integer geometry, generic navigation, line of sight, spatial queries, hashing |
| `dread-sim` | Campaign and skirmish rules, abilities, characters, quests, items, commands, saves |
| `dreadmarch-client` | Native Bevy presentation, UI and input |
| `dread-wasm` | Thin WebAssembly ABI over the same authoritative simulation |
| `dread-headless` | Deterministic replay/save verification and CPU timing |
| `engine-lab` | Independent warehouse-routing sample importing only the generic engine |

`dread-core` depends on neither Bevy nor Direvanity rules. A stable SDK, visual content editor and a
second full game remain production milestones; the independent consumer already verifies the boundary.

## Verify

```bash
python3 scripts/validate_content_plan.py
cargo test --workspace --exclude dreadmarch-client --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
python3 scripts/build_web.py
node tests/rpg_abi.mjs
node tests/rpg_controls.mjs dist/Direvanity-campaign.html
node tests/web_abi.mjs
node tests/web_controls.mjs dist/Direvanity-playtest.html
cargo run --locked -p dread-sim --example battle_smoke
```

The campaign ABI suite completes both chapters, all four side quests and three expedition objectives
with **every class**, through actual public commands, then verifies persistence and save fidelity. Structural browser tests and actual visual/runtime
inspection are reported separately. Software Vulkan proves functional rendering, not gaming-GPU FPS.

## Production destination

Read [VISION.md](docs/VISION.md), [ROADMAP.md](docs/ROADMAP.md), [ARCHITECTURE.md](docs/ARCHITECTURE.md),
[TECHNOLOGY.md](docs/TECHNOLOGY.md) and [STATUS.md](docs/STATUS.md). The next content milestone is a
production-quality regional slice: original rigged character art, expressive combat, authored enemies
and boss, richer quest decisions, inventory depth and controller movement, measured on real hardware.
The detailed 100+ / 100+ / 100+ hour production contract is [DIREVANITY_PRODUCTION.md](docs/DIREVANITY_PRODUCTION.md).

## License

Original project code is dual licensed under MIT or Apache-2.0. Dependencies retain their licenses;
research data preserves source provenance. See [THIRD_PARTY.md](THIRD_PARTY.md). Inspiration credits
are acknowledgements, not affiliation or a claim to the original games’ assets.
