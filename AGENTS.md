# Dreadmarch development contract

Read README.md and docs/VISION.md, then the code for the affected subsystem.
The canonical source repository is https://github.com/Herbertofury/Dreadmarch.

- Preserve the ambition: a high-end original action RTS and a reusable Rust engine.
- Diablo IV, Lost Ark, Path of Exile, and Path of Exile II are creative inspirations.
- Keep generic engine systems in dread-core, game rules in dread-sim, and rendering/input in clients.
- All clients must use the same authoritative Rust simulation. No JavaScript gameplay fork.
- Integer deterministic simulation, stable command ordering, explicit save/replay versions.
- Never claim an aspiration, benchmark target, compiled binary, or screenshot is a shipped AAAA feature.
- Never cap unit counts, omit content, or silently change simulation behavior to improve benchmark numbers.
- Use current compatible dependencies, pin the toolchain, retain Cargo.lock, and record upgrade evidence.
- Adopt upstream code only with exact provenance and its notices; third-party assets have separate provenance.
- Do not use image generation unless the current request explicitly authorizes it.
- Verify changes with the cheapest decisive tests, then exercise actual gameplay and a release build.
- Update docs/STATUS.md with verified results, remaining work, exact commit, and the next actionable milestone.
- Preserve unrelated user work. Checkpoint before long gates. Do not repeat a failed route unchanged.

Main gates: cargo fmt --all -- --check; cargo test --workspace --exclude dreadmarch-client;
cargo clippy --workspace --all-targets -- -D warnings; cargo build -p dreadmarch-client;
python3 scripts/build_web.py; cargo run -p dread-headless -- --verify.
