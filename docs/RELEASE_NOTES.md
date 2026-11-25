# v0.1.0-alpha.1 — First playable foundation

Dreadmarch begins as an original dark-fantasy action RTS and reusable Rust engine, inspired by
Diablo IV, Lost Ark, Path of Exile and Path of Exile II.

This prerelease contains the first geometric encounter: commander and squads, formation movement,
combat, Soulfire, recruitment, three relics, five enemy waves, victory/defeat and validated saves.
The native Bevy3D game and the offline browser playtest share the same authoritative Rust simulation.

## Downloads

- Windows: extract `Dreadmarch-windows-x64.zip` and run `dreadmarch.exe`. If `VCRUNTIME140.dll`
  is missing, install the [official Microsoft Visual C++ x64 Redistributable](https://aka.ms/vc14/vc_redist.x64.exe).
  See [Microsoft's runtime guide](https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist/).
- Linux: extract `Dreadmarch-linux-x64.zip` and run `dreadmarch`. The executable requires glibc
  2.35 or newer, X11 runtime libraries and a working graphics driver. If an archive tool drops
  executable permissions, run `chmod +x dreadmarch` before launching.
- Browser: download `Dreadmarch-playtest.html` and open it directly. Everything is embedded.

The repository pins Rust 1.100.0, Bevy 0.20.1 and a locked dependency graph. Source is included by
GitHub with the release. The generic engine can also be exercised with `cargo run -p engine-lab`.

## What was verified

Cross-platform CI passed verification and native Linux/Windows jobs, including strict Clippy,
the three native client regressions and native builds. The optimized Windows x64 executable is built
and packaged; Windows GPU runtime and windowed input remain open verification work.

The portable optimized Linux build uses Ubuntu 24.04. Its exact downloaded CI artifact passed
provider and package checksums, fresh extraction, executable-mode checks, and independent inspection
of the glibc requirement. That extracted binary completed a real 120-frame offscreen Vulkan run on
Debian 13, including movement, recruitment, Soulfire, save/load and a visually inspected screenshot
with the complete HUD. Its final simulation checksum was `4d08544a95f6c631`.

Rendering was verified using software Vulkan. This establishes functional rendering and gameplay
correctness, not target-hardware performance. [STATUS.md](STATUS.md#native-release-evidence) records
the exact build revisions, artifact hashes and remaining verification boundaries.

## Next implementation milestone

**M1 combat and navigation** adds unit footprints and avoidance, congestion tests, data-driven
abilities and status effects, authored combat animation, and a boss encounter. Windows hardware
capture and frame-time instrumentation accompany that vertical slice.

This is an early playable engineering foundation, not finished AAAA production. Art, animation,
audio, deep loot/build systems, campaign, multiplayer, crowd avoidance and visual authoring remain
explicit milestones. Read docs/STATUS.md for verified results and docs/VISION.md for the quality bar.
