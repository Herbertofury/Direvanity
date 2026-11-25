# Provenance

Original Rust game/engine code, browser presentation, procedural geometry and documentation in this
repository were created for Dreadmarch. No Diablo, Lost Ark or Path of Exile source or media files
are distributed here.

| Dependency | Use | License/source |
| --- | --- | --- |
| Bevy | Native application, ECS, rendering, UI, asset framework | MIT OR Apache-2.0; https://github.com/bevyengine/bevy |
| wgpu | Bevy's graphics abstraction | MIT OR Apache-2.0; https://github.com/gfx-rs/wgpu |
| serde | Typed state serialization | MIT OR Apache-2.0; https://github.com/serde-rs/serde |
| serde_json | Save/ABI JSON encoding and decoding | MIT OR Apache-2.0; https://github.com/serde-rs/json |

Cargo.lock is the exact dependency graph. Transitive dependencies have their own package notices.
The native built-in font is supplied by Bevy with its upstream package licensing.

Upstream research candidates in docs/RESEARCH.md are references, not automatically vendored code.
Any future imported code or asset must add its exact source revision, license and modifications here.
