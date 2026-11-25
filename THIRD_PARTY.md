# Third-party provenance and redistribution notices

Original Rust game and engine code, browser presentation, procedural geometry, and documentation in this repository were created for Dreadmarch. No proprietary Diablo, Divinity, Lost Ark, or Path of Exile implementation code or media files are distributed here. Research candidates in `docs/RESEARCH.md` are references; they are not automatically vendored code.

## Exact dependency inventory

`Cargo.lock` records the exact dependency graph. The initial locked graph contains **482 third-party registry packages** and six local workspace packages. This intentionally includes platform-specific, optional, build, and development dependencies; listing a package does not assert that it is linked into every executable.

| Foundation | Use | Declared license and upstream |
| --- | --- | --- |
| Bevy 0.20.1 | Native application, ECS, renderer, UI, and assets | MIT OR Apache-2.0; <https://github.com/bevyengine/bevy> |
| wgpu | Bevy graphics abstraction | MIT OR Apache-2.0; <https://github.com/gfx-rs/wgpu> |
| serde / serde_json | Typed state serialization, saves, and ABI JSON | MIT OR Apache-2.0; <https://github.com/serde-rs/serde> and <https://github.com/serde-rs/json> |
| Fira Mono subset embedded by bevy_text 0.20.1 | Default native UI font | SIL Open Font License 1.1; the full upstream notice is included in the bundle |

## Generate the full notice bundle

With Python 3.11 or newer and the repository's Rust toolchain:

```sh
cargo fetch --locked
python scripts/collect_notices.py
```

The fetch materializes packages for the complete locked graph. The collector itself runs offline, reads `cargo metadata --locked --offline`, and writes `dist/THIRD_PARTY_NOTICES.txt`. It preserves the complete available license, notice, copyright, and author files; records the Cargo package checksums and individual text hashes; and verifies recovered upstream notices against both their SHA-256 and the published package's source revision. It does not replace package notices with generic license templates.

Some published crates omit their repository-level license files. Exact upstream texts recovered for those packages are checked into `third_party/notices/`, with package assignments, source URLs, revisions, and content hashes in `third_party/notices/index.json`. Shared monorepo notices are downloaded once per repository revision and file. The font license is also cached there and is associated with the exact embedded font hash. These files are notices only; no upstream game code or artwork is imported.

**Distribution:** include `THIRD_PARTY_NOTICES.txt`, this document, and Dreadmarch's `LICENSE-MIT` and `LICENSE-APACHE` with native binaries. Publish the generated notice bundle beside separately distributed browser or native artifacts. Preserve the notices when redistributing the engine. The bundle inventories Cargo packages; operating-system libraries and compiler toolchain components are identified separately from this registry inventory.

## Recorded exceptions

**constgebra 0.1.4** declares `MIT OR Apache-2.0` in its published manifest, but supplies no license/notice file in either its crate archive or the upstream tree at commit `eae8e094e5779f42f1db2a7adf9036aa33744bc8` (<https://github.com/knickish/constgebra/tree/eae8e094e5779f42f1db2a7adf9036aa33744bc8>). The bundle preserves the manifest declaration and attribution and lists the missing upstream text explicitly. No copyright notice has been fabricated.

**dispatch 0.2.0** declares `MIT` in its manifest but also supplies no license/notice text in its crate or upstream tree at commit `82d6c7a5b75dc0c71c3f46f87bb6c16a476f7748` (<https://github.com/SSheldon/rust-dispatch/tree/82d6c7a5b75dc0c71c3f46f87bb6c16a476f7748>). Its manifest declaration and attribution are preserved in the bundle.

The collector writes the bundle and returns a nonzero status while any exception remains. `--allow-incomplete` can be used deliberately to produce a distribution supplement with that exception still visible; it does not remove the exception or claim the bundle is complete. Resolve the missing upstream notice before treating this check as a fully satisfied redistribution gate.

Future imported code or assets must add their exact source revision, license, notices, and any modifications. When the lock changes, regenerate the bundle and review any new missing notices instead of reusing a stale dependency list.

## Research catalogues

`data/research/` contains versioned factual names, identifiers, category/ownership information, tree
relationships and original mechanical synthesis. The three atlases preserve their source registers,
input hashes, edition boundaries and unresolved coverage. Upstream tooltip prose and assets are not
redistributed. These records are research references and do not imply that source-engine code was
relicensed under Dreadmarch's MIT/Apache licenses.

OpenDivine and Divine Tools are recorded with their upstream GPL-3.0 licenses; divproxy is recorded
as AGPL-3.0. The pinned current DevilutionX source uses Sustainable Use License 1.0; it is not treated
as a permissive commercial-engine base. No code from these projects is imported into Dreadmarch.
See the exact pinned source-license ledger in `data/research/diablo-catalog.json` and the Divinity atlas.
