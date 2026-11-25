# DREADMARCH

**An ambitious original action RTS. A reusable Rust engine beneath it.**

[**Play the browser encounter**](https://herbertofury.github.io/Dreadmarch/) ·
[Native downloads](https://github.com/Herbertofury/Dreadmarch/releases) ·
[Production roadmap](docs/ROADMAP.md)

Lead a commander and a growing legion through a dark-fantasy battlefield. Reclaim three relics,
survive five enemy breaches, and combine tactical orders with a decisive commander ability.

Our great inspirations are **Diablo IV, Lost Ark, Path of Exile, and Path of Exile II**.
The long-term goal is to meet and surpass their best qualities through exceptional combat,
art direction, animation, systemic depth, responsiveness and polish. That is the production
ambition, not a claim about the current geometric prototype.

## What exists today

- A native **Bevy 0.20.1** 3D client with an isometric camera, selection, orders, HUD and combat effects.
- A completely offline browser playtest compiled from the **same Rust simulation**.
- Integer fixed-step gameplay, obstacle-aware pathfinding, formation destinations and ranged line of sight.
- Commander, vanguard and ranger units; enemy reavers and brutes; combat, recruitment and essence income.
- Three capture objectives, five waves, victory/defeat, restart and validated save/load.
- A renderer-independent engine crate and an independent warehouse-navigation sample.
- Replay tests, full-battle verification, spatial-query benchmarks and a native offscreen render check.

Read [STATUS.md](docs/STATUS.md) for the exact verified state and remaining limitations. The native
and browser presentations are distinct, while authoritative game rules are identical. The browser
playtest uses Canvas2D to make the simulation easy to try; it is not the production GPU renderer.

## Play or build

Download a published playtest from [Releases](https://github.com/Herbertofury/Dreadmarch/releases).
`Dreadmarch-playtest.html` is a single offline file: download it and open it in a modern browser.
Windows native packages contain `dreadmarch.exe`. The program requires no proprietary game assets.

To build from source, install the [official Rust toolchain](https://rustup.rs/) and the normal C/C++
linker for your platform. Windows uses Visual Studio Build Tools with Desktop development with C++.
The repository pins Rust **1.100.0** and provides `Cargo.lock`.

```bash
git clone https://github.com/Herbertofury/Dreadmarch.git
cd Dreadmarch
cargo run -p dreadmarch-client --release
```

Linux requires X11 development/runtime libraries for the windowed client. The first scene is
procedural, so there is no asset download step.

### Browser build

```bash
rustup target add wasm32-unknown-unknown
python3 scripts/build_web.py
# Open dist/Dreadmarch-playtest.html directly. No server or installation needed.
```

### Controls

| Action | Native 3D | Browser playtest |
| --- | --- | --- |
| Select / box select | Left click / left drag; Shift adds | Left click / left drag; Shift adds |
| Move squads | Right click; Shift-right-click to attack-move | Right click |
| Commander ability | Space, then choose a target | Q, then choose a target |
| Recruit vanguard / ranger | 1 / 2, or the action buttons | V / B, or the action buttons |
| Select army / commander | Tab / C | A / 1 |
| Hold position | H | H |
| Pause / resume | P | Space |
| Restart | R | Restart button |
| Save / load | F5 / F9 | Save / Load buttons |
| Camera | WASD pan, Q/E orbit, wheel zoom | Wheel zoom, middle drag, F to fit |
| Field manual | F1 | Help button |

The clients' field manuals describe their selection shortcuts and camera details.

## Reusable engine, separate game

| Package | Responsibility |
| --- | --- |
| `dread-core` | Fixed-point geometry, generic navigation, line of sight, spatial queries, hashing |
| `dread-sim` | Dreadmarch rules, units, combat, economy, objectives, commands and saves |
| `dreadmarch-client` | Native 3D Bevy presentation, UI and input |
| `dread-wasm` | Thin WebAssembly ABI over the same simulation |
| `dread-headless` | Deterministic replay/save verification and CPU timing |
| `engine-lab` | Separate warehouse-routing sample using only the generic engine |

```bash
cargo run -p engine-lab
cargo run -p dread-headless -- --verify
cargo run -p dread-sim --example battle_smoke
cargo run -p dread-core --example spatial_bench --release
```

`dread-core` depends on neither Bevy nor Dreadmarch's game rules. This establishes a real reusable
boundary; a general-purpose visual editor and a second full game remain future milestones.

## Quality gates

```bash
cargo fmt --all -- --check
cargo test --workspace --exclude dreadmarch-client
cargo clippy --workspace --all-targets -- -D warnings
cargo build -p dreadmarch-client
cargo run -p dreadmarch-client -- --smoke-test --screenshot dist/native-smoke.png
python3 scripts/build_web.py
node tests/web_abi.mjs
node tests/web_controls.mjs dist/Dreadmarch-playtest.html
```

The native smoke test renders offscreen using the actual GPU pipeline, applies real gameplay
commands, checks save/load and captures the scene. A software Vulkan driver is sufficient for
functional QA. Its timing is not an RTX 4090 performance result.

## Where this goes

- [Vision and quality targets](docs/VISION.md)
- [Production roadmap](docs/ROADMAP.md)
- [Architecture](docs/ARCHITECTURE.md)
- [Current research and reuse candidates](docs/RESEARCH.md)
- [Technology decisions](docs/TECHNOLOGY.md)
- [Verified checkpoint and next action](docs/STATUS.md)

The research includes **D2Gamma**, **ferrets**, **Digital Extinction**, **Oxidator**, **Veloren**,
**OpenDiablo2** and **DevilutionX**, with their actual languages, maturity and provenance. Current
source was checked rather than assuming old search snippets still describe today's versions.

## License

Original project code is dual licensed under MIT or Apache-2.0, at your option. Dependencies retain
their own licenses. See [THIRD_PARTY.md](THIRD_PARTY.md). No commercial game assets are bundled.
