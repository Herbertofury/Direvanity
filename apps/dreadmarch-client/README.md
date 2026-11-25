# Native Dreadmarch client

This is the original 3D presentation for the deterministic `dread-sim` game. It uses Bevy 0.20.1 and Bevy's compatible wgpu graph. No Diablo, Lost Ark, Path of Exile, or other commercial game assets are included. Geometry is authored procedurally from meshes and materials in `src/main.rs`.

```sh
cargo run -p dreadmarch-client --release
```

## Play

Select warriors with left-click or a drag rectangle. Hold Shift to add to selection. Right-click issues a formation move; Shift + right-click issues an attack-move. The simulation decides navigation, targeting, damage, capture, recruitment costs and outcomes. Rendering never advances those systems independently.

| Control | Action |
| --- | --- |
| Tab | Select all living allies |
| C | Select and center commander |
| Space | Cast Soulfire at ground cursor |
| 1 / 2 | Recruit Vanguard / Ranger |
| H | Hold selected units |
| WASD | Pan |
| Q / E | Orbit |
| Wheel | Zoom |
| P | Pause |
| R | Restart the seeded scenario |
| F1 | Help |
| F5 / F9 | Save / load `dreadmarch-save.json` in the working directory |
| F12 | Save `dreadmarch-screenshot.png` |
| Esc | Exit |

The bottom action bar also provides mouse controls. The Soulfire button arms a ground-target action. Invalid commands display the simulation error; insufficient resources and cooldowns never report success.

## Render smoke check

```sh
cargo run -p dreadmarch-client -- --smoke-test --screenshot smoke.png
```

This builds the real native scene and UI, renders them to a GPU image without a window, advances the real simulation, requires movement/recruitment/casting commands to succeed, validates a save/load checksum, optionally captures the actual frame, prints `DREADMARCH_RENDER_SMOKE_OK`, then exits. The capture observer must fire and the PNG must exist for success when a screenshot is requested. Inspect the PNG as the visual acceptance step. This finite smoke route supplements interactive input verification; it is not a complete gameplay acceptance test or a claim about production visual quality.

## Current boundary

The playable foundation provides original geometric presentation and real shared gameplay. Production character/environment assets, animation rigs, sound, networking, graphics settings and authored campaign content remain roadmap work. The prototype does not claim to match the visual quality of its commercial inspirations.
