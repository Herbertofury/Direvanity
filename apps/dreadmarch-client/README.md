# Native Direvanity client

The native client presents the same authoritative Rust campaign as the browser build, using
Bevy 0.20.1. The game is an original offline single-player ARPG in development. The native world
currently uses original procedural geometry. Authored production characters, rigs, environments,
audio and the extensive campaign remain production work.

```sh
cargo run --locked -p dreadmarch-client --release
# Packaged executable: direvanity / direvanity.exe
```

## The living command deck

- Animated vitality and class-resource reservoirs use an embedded GPU material with exact circular
  clipping, glass highlights, an engraved rim and class-specific liquid motion. They show exact
  current/maximum values. Resource
  names, colors, capacities and combat/idle regeneration come from the Rust class catalogue.
- Every active class discipline remains on the deck, including unlearned skills. The selected
  skill, resource cost, insufficient-resource state and real remaining cooldown are visible.
  Select a learned skill to arm it, then click the world or press Space to cast. Select an
  unlearned skill to inspect its requirements and learn it with an available point.
- The experience ribbon, level, skill points and gold reflect actual progression.
- The journal reads the authoritative campaign and side-quest entries, consequences and active
  expedition objective. The atlas presents the complete overworld before current-region points
  of interest, roads and choices.
- Skills, inventory, journal, atlas and help pause simulation while open. Closing a panel preserves
  an existing manual pause. All inventory entries remain in one scrollable pack; there is no
  item-count limit or pagination.
- F4 reduces decorative HUD motion. Exact values and cooldown information remain visible.

## Campaign controls

| Control | Action |
| --- | --- |
| WASD / click ground | Move relative to the camera / approach a point |
| Space | Cast the selected learned active skill at the cursor |
| 1–9 | Cast learned active skills in catalogue order |
| Click a skill card | Select and arm it; inspect requirements if unlearned |
| J | Interact with the closest person, choice, cache, loot or road |
| K / I / N / M | Skills / inventory / journal / world atlas |
| [ / ] | Select previous / next skill or inventory item |
| L / U / T | Learn selected skill / equip selected item / retrain in Hearthmere |
| Mouse wheel / Page Up / Page Down in an open panel | Scroll the complete panel |
| P | Toggle manual pause |
| F4 | Toggle reduced HUD motion |
| F2 | Open the character creator |
| F9 in the creator | Resume a saved journey |
| Q / E / mouse wheel outside panels / C | Orbit / zoom / center hero |
| F5 / F9 | Save / load a validated portable save |
| F1 / Escape | Help / close current panel, then exit |
| F12 | Save `direvanity-screenshot.png` |
| R | Restart the current character's journey |

The creator retains all 19 classes and independent appearance choices. **Edit Name** enables typing;
Enter finishes editing, and Enter outside that editor begins the journey. Begin creates a new world;
F9 resumes the saved world instead.

The default save is `direvanity-campaign.json`. If that file does not exist, F9 can restore the older
`dreadmarch-campaign.json`. Saving then creates a Direvanity copy and preserves the old file. An
existing invalid current save is reported as an error rather than silently replaced by old progress.
Writes use a synchronized temporary file and rename. `--save PATH` disables automatic legacy fallback
for that explicit path; `--load PATH` imports a validated exported campaign or skirmish save.

## Optional skirmish

```sh
cargo run --locked -p dreadmarch-client --release -- --skirmish
```

The earlier tactical encounter remains available: left-click/drag selects warriors, Shift adds to
selection, right-click moves, Shift + right-click attack-moves, Tab selects the army, C centers the
commander, Space casts Soulfire, 1/2 recruit, and H holds. WASD pans the camera. The tactical footer
shows the real costs and commands. Its default save is `direvanity-skirmish.json`, with fallback to
`dreadmarch-save.json` only when the new default file is absent.

## Render and regression checks

```sh
cargo test --locked -p dreadmarch-client
cargo run --locked -p dreadmarch-client -- --campaign-smoke --screenshot native-campaign.png
cargo run --locked -p dreadmarch-client -- --smoke-test --screenshot native-skirmish.png
```

These finite smoke routes render the real scene and native UI to an offscreen GPU image, exercise
real commands and UI activation, validate save/load checksums, and require the requested screenshot
capture to complete. The stable `DREADMARCH_*_SMOKE_OK` log markers are retained for existing CI.
They are historical test identifiers, not the displayed game name.

Review the generated images for layout and presentation. Offscreen software Vulkan proves functional
rendering; it does not establish gaming-GPU frame rates, Windows interactive input, or production AAA
visual quality. The source tests separately cover all-class hotbar accessibility, panel pause behavior,
legacy save continuity and rejection of corrupt current saves.

The UI uses Bevy's native widgets and overflow. `src/orb.wgsl` is an original shader embedded in the
executable, with no runtime asset download. Reservoir animation updates GPU uniforms instead of
changing UI geometry every frame; reduced motion preserves numeric state and cooldown information.
References: [native UI material interface](https://bevy.org/examples/ui-user-interface/ui-material/) and
[scroll behavior](https://bevy.org/examples/ui-user-interface/scroll/).
