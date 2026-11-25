# DREADMARCH

**A massive single-player action RPG in development. A reusable Rust engine beneath it.**

[**Play the campaign**](https://herbertofury.github.io/Dreadmarch/) ·
[Source and downloads](https://github.com/Herbertofury/Dreadmarch/releases) ·
[Research atlases](docs/research/) · [Production roadmap](docs/ROADMAP.md)

Create a character, choose a class, explore an original dark-fantasy world, meet its people, discover
secrets and equipment, and develop a build through an authored campaign. The original **Divine Divinity
(2002)** is central to the world and role-playing direction. Our great inspirations also include the
**entire Diablo series, Diablo IV, Lost Ark, Path of Exile and Path of Exile II**.

The ambition is to meet and surpass the strongest qualities of these games through exceptional combat,
art direction, animation, systemic depth, exploration, responsiveness and polish. “AAAA” is our quality
destination. The current playable build is a procedural prologue, not a finished massive campaign.

## What is playable

- **19 class identities:** Warrior, Rogue, Sorcerer/Sorceress, Monk, Bard, Barbarian, Amazon, Assassin,
  Necromancer, Paladin, Druid, Crusader, Demon Hunter, Witch Doctor, Wizard, Blood Knight, Tempest,
  Spiritborn and Warlock. Class and body silhouette are independent.
- Character name, body silhouette, six hair styles, skin/hair/eye/cloth colors and stature, with a live preview.
- **174 original prototype skill nodes**, organized by class into prerequisites and branches. They are
  separate from the source-game research catalogue; not every researched source ability is implemented.
- One hero, active abilities, mastery passives, cooldowns, focus, XP/levels, skill points and free hub retraining.
- Three connected regions: **Hearthmere → Hollow Moor → Forgotten Sanctum**. Talk to the archivist,
  recover the Ashen Concord, return it, and resolve the quartermaster’s missing-satchel quest.
- Persistent defeated enemies, unopened/opened caches, dropped loot, inventory and equipped items.
- Independent browser character slots, exact save/load and portable JSON save export/import.
- Native Bevy 3D presentation and an offline browser client backed by the **same Rust simulation**.

The native campaign integration and release evidence are tracked in [STATUS.md](docs/STATUS.md).
Always check the release tag: the older `v0.1.0-alpha.1` binaries contain the earlier RTS encounter.
That encounter remains an [optional skirmish](https://herbertofury.github.io/Dreadmarch/skirmish.html)
and regression fixture. The campaign is now the primary product.

## Research, then original synthesis

The research preserves source-game identities, editions, patches, tree structure and coverage boundaries.
A record in an index is not automatically a complete behavioral specification. Source mechanics inform
original Dreadmarch abilities; commercial game code, dialogue, art, music and models are not bundled.

- [Diablo class and ability atlas](docs/research/DIABLO_CLASS_ATLAS.md)
- [Divinity series atlas, including original Divine Divinity](docs/research/DIVINITY_ATLAS.md)
- [Lost Ark / Path of Exile systems and class mapping](docs/research/ARPG_SYSTEMS.md)
- [Accepted RPG direction](docs/RPG_DIRECTION.md)
- [Original class synthesis and production rules](docs/CLASS_SYNTHESIS.md)

Research datasets live in `data/research/`. Playable original definitions live in
`data/gameplay/classes.json`. Keeping those domains separate makes actual implementation coverage visible.

## Run or build

The browser campaign is a single offline file, `Dreadmarch-campaign.html`. Open it in a modern desktop
browser; no proprietary asset download or server is required. Use the published play link or build:

```bash
git clone https://github.com/Herbertofury/Dreadmarch.git
cd Dreadmarch
python3 scripts/build_web.py
# Open dist/Dreadmarch-campaign.html.
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

Launch `dreadmarch` (or `dreadmarch.exe`) to open the creator. Class and appearance buttons edit the
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

The default campaign save is `dreadmarch-campaign.json`. Use `--save PATH` for a separate character
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

`dread-core` depends on neither Bevy nor Dreadmarch rules. A stable SDK, visual content editor and a
second full game remain production milestones; the independent consumer already verifies the boundary.

## Verify

```bash
cargo test --workspace --exclude dreadmarch-client --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
python3 scripts/build_web.py
node tests/rpg_abi.mjs
node tests/rpg_controls.mjs dist/Dreadmarch-campaign.html
node tests/web_abi.mjs
node tests/web_controls.mjs dist/Dreadmarch-playtest.html
cargo run --locked -p dread-sim --example battle_smoke
```

The campaign ABI suite completes both quests with **every class**, through actual public commands,
then verifies region persistence and save fidelity. Structural browser tests and actual visual/runtime
inspection are reported separately. Software Vulkan proves functional rendering, not gaming-GPU FPS.

## Production destination

Read [VISION.md](docs/VISION.md), [ROADMAP.md](docs/ROADMAP.md), [ARCHITECTURE.md](docs/ARCHITECTURE.md),
[TECHNOLOGY.md](docs/TECHNOLOGY.md) and [STATUS.md](docs/STATUS.md). The next content milestone is one
production-quality RPG region: original rigged character art, expressive combat, authored enemies and
boss, branching quest decisions, inventory depth and direct/controller movement, measured on real hardware.

## License

Original project code is dual licensed under MIT or Apache-2.0. Dependencies retain their licenses;
research data preserves source provenance. See [THIRD_PARTY.md](THIRD_PARTY.md). Inspiration credits
are acknowledgements, not affiliation or a claim to the original games’ assets.
