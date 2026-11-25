# First playable foundation

Dreadmarch begins as an original dark-fantasy action RTS and reusable Rust engine, inspired by
Diablo IV, Lost Ark, Path of Exile and Path of Exile II.

This prerelease contains the first geometric encounter: commander and squads, formation movement,
combat, Soulfire, recruitment, three relics, five enemy waves, victory/defeat and validated saves.
The native Bevy3D game and the offline browser playtest share the same authoritative Rust simulation.

## Downloads

- Windows: extract Dreadmarch-windows-x64.zip and run dreadmarch.exe.
- Linux: extract Dreadmarch-linux-x64.zip and run dreadmarch.
- Browser: download Dreadmarch-playtest.html and open it directly. Everything is embedded.

The repository pins Rust 1.100.0, Bevy 0.20.1 and a locked dependency graph. Source is included by
GitHub with the release. The generic engine can also be exercised with `cargo run -p engine-lab`.

This is an early playable engineering foundation, not finished AAAA production. Art, animation,
audio, deep loot/build systems, campaign, multiplayer, crowd avoidance and visual authoring remain
explicit milestones. Read docs/STATUS.md for verified results and docs/VISION.md for the quality bar.
