# Development environment recovery

## Current Direvanity continuation · 25 November 2026

Resume from [STATUS.md](STATUS.md) and the identities below. This is the expanded offline single-player
ARPG; the legacy RTS remains a separate regression scenario. Preserve all 19 classes, 174 prototype
nodes, existing saves and committed research. Do not restart the earlier prologue-only or RTS milestone.

### Canonical state

| Item | Verified identity |
| --- | --- |
| Repository | `https://github.com/Herbertofury/Dreadmarch` |
| Working directory | `/workspace/scratch/21e05b9e458e/Direvanity` |
| Implementation commit | `40deec9f9108f3494703e3f0e5c994ef63a191e0` |
| Merged main commit | `c7a76ab62cb6c0adfd00703bd80217ea60dd5b3d` |
| Both commits' implementation tree | `a5591ad20f1131d87fe48a26efb5b3a9349411a8` |
| Pull request | `https://github.com/Herbertofury/Dreadmarch/pull/4`, merged |
| Full CI | `45708059241`, all three jobs passed |
| Pages deployment | `45708530418`, build and deploy passed at the merge revision |
| Live game | `https://herbertofury.github.io/Dreadmarch/` |
| Optimized release build | `45708000312`, both platforms passed at the implementation commit |
| Published prerelease | `https://github.com/Herbertofury/Dreadmarch/releases/tag/v0.2.0-alpha.1`; release `458757949`, tag at the implementation commit |

The current implementation contains two playable chapter loops, six fixed regions, four side quests
and one repeatable expedition region. The 12-act/36-region-group/48-arc production catalogue and its
100+ hour categories are unvalidated authoring estimates, not shipped hours or finished AAA production.

### Completed work: do not repeat without a relevant change

- Final local formatting, 45 Rust tests and strict workspace/all-target Clippy passed; CI is green.
- All 19 classes completed both chapters and all four side quests: 76 side-quest instances,
  57 expeditions, every objective and modifier 19 times, 259,620 total ticks including prologue ticks.
- Three additional earned-save combat runs verify modifier effects. Numeric-boundary valid saves,
  malformed progression rejection and nonfinite planning targets are covered.
- Browser structural controls passed 23 cases, including exact earned-save imports. Live browser
  interaction separately verified class resources, casting/cooldowns, learned companion, pause,
  atlas/local map/navigation, recovered loot/equipment/search/sort, journal, save/reload and motion.
- Final development native binary passed actual campaign and legacy rendering, save and HUD checks.
- Both optimized platform artifacts were downloaded, matched to provider and package hashes,
  safely extracted, and reconciled with 25 complete bundled project data/documentation files per platform.
- The exact freshly extracted optimized Linux binary passed campaign and legacy runtime gates;
  all three captures were reviewed. No local rebuild replaced the CI executable.
- The `v0.2.0-alpha.1` prerelease is published, not a draft. All six uploaded release assets have
  provider SHA-256 digests and sizes matching the verified local bytes. Exact asset identities are in
  `dist/qa/release-publication.json`; do not upload the same unchanged assets again.
- Existing research inventories are committed. Reuse their IDs, paths and coverage boundaries.

The six browser screenshots are retained as `dist/qa/direvanity-*.jpg`. The late-campaign browser
file-chooser import stalled and was interrupted; no live-browser visual completion is claimed for
those three fixture imports. Their structural real-WASM import proof remains valid. Do not repeat
an unchanged stalled picker call or claim screenshots that were not captured.

### Exact package and runtime checkpoint

Release artifact IDs are Linux `15065184844` and Windows `15067410496`; jobs are Linux `139534824643`
and Windows `139534824307`. Full receipts are under `dist/release-direvanity/`:

- `release-validation.json`: both platforms and independent runtime summary.
- `linux-ci/outer/Direvanity-linux-x64.zip`: package SHA-256
  `99d6a69077b16f63b29e017f0ff3c911d17b907280d086c44ecfa5379a58f3a8`.
- `windows-ci/outer/Direvanity-windows-x64.zip`: package SHA-256
  `9dc763ec6e5e834f18c333ed3aed52cd1b754740ad13bd6ad8626701b25f67ec`.
- Linux binary SHA-256: `4c0a0310ff519e6aefb198a4a964f0d2c54f32f7608d4a112133bc103c512c1f`.
- Windows binary SHA-256: `170cf8c39352c331d990735d0984186388def3d9399b37c9c4b22019bc8e1f60`.
- `extracted-runtime/optimized-runtime-proof.json`: campaign exit 0, 185 frames / tick 354,
  ten commands, checksum `6ca474ed5e8d0a66`; legacy exit 0, 120 frames / tick 120, three commands,
  checksum `4d08544a95f6c631`. Both match CI state and preserve the exact binary hash.

Linux requires glibc 2.35. Software Vulkan proves functional rendering, not gaming-GPU frame pacing.
Windows build/tests and payload validation passed; Windows graphics/input runtime remains untested.
The separate development binary is `5d5ada367b84f71ddcb01ecf83dea4e80346431b2e8f8e798c0f2e1bf346e4cc`;
its 184-frame campaign receipt must not be substituted for the optimized package's 185-frame proof.

The release-CI browser WASM hash is `a77b2c868d80b5d8c22fdd079cb8111153666f496eec163d63659370217d6026`.
The local development WASM hash is `dada0101efb8415504a5c12f04df80c87383c4331934160dfcd453016a26f2dd`.
Keep their build receipts separate. Existing local complete-development archive
`dist/Direvanity-verified-development-checkpoint.zip` has SHA-256
`5f2db429bf68b22d199255238f8108e6d5d014cce816c21f6c65b70ffe1fbb86`; it predates the final release-package
bundle and records its then-uncommitted source state. Do not present it as the final distribution.

### Blockers and rejected routes

Automatic approval review rejected the complete archive's Google Drive upload because the specific
payload and destination lack explicit user-authored authorization. It separately rejected changing
the GitHub repository URL, distinguishing that operation from the authorized game rename. Neither
operation succeeded. Obtain specific user approval before trying either again; no alternate upload
or rename route may bypass those rejections.

Do not repeat the installation-token release-create HTTP403 route or expand token permissions.
Use supported authenticated publication controls and verify the actual resulting release assets.
Download Actions artifacts through the dedicated GitHub artifact-download operation, not the blocked
browser download handoff. Preserve exact run/artifact IDs; do not rebuild or redownload already verified
unchanged packages. The scoped Xvfb/AF_UNIX failure below remains unchanged; do not reprovision it.

### Exact next delivery action

The GitHub prerelease and its six assets are already published and their remote digests/sizes are
verified in `dist/qa/release-publication.json`. The separate complete source/build/evidence archive is
packaged after this documentation commit as `dist/Direvanity-checkpoint.zip`; consult its adjacent
`.manifest.json` and internal `CHECKPOINT.json` for its actual identity and payload verification.
Reuse an existing verified checkpoint. If those artifacts are absent, package the source with both
verified platform ZIPs/receipts, the exact released offline browser files and captured QA evidence.
The Drive transfer and repository URL rename remain approval-blocked until specifically authorized.
The late-save browser visual boundary remains recorded above. After delivery, continue from the next
implementation milestone; do not rediscover source, rebuild unchanged binaries, re-upload the six
existing release assets or rerun settled gameplay checks.

### Next implementation milestone

Bring Hearthmere → Hollow Moor → Forgotten Sanctum to a production-quality region with original
rigged art, typed attack timelines, real projectile/chain delivery, distinct class generators, authored
enemy ecology, branching dialogue, audio/VFX and a multi-phase boss. Verify controller/rebinding,
accessibility and Windows/Linux hardware frame pacing before scaling to the planned acts. Preserve
the playable expansion and expeditions throughout; do not substitute more duration estimates for
finished content.

Use `source /workspace/scratch/f01d9d0c81e4/toolchain/env.sh` for the restored Rust toolchain and
`source /workspace/scratch/f01d9d0c81e4/system-packages/env.sh` for the validated offscreen Vulkan route.
Reuse these paths while their host and linkage remain unchanged. Earlier alpha.2 release provenance
is preserved in [HISTORY_ALPHA2.md](HISTORY_ALPHA2.md); historical environment records follow.

## Rootless Debian runtime packages and offscreen Vulkan

Recorded 2026-11-24, America/Denver. This entry covers development-runner provisioning, not a game performance result.

### Failure signature

The Debian 13 (trixie) runner has no sudo access, no usable global APT package lists, no `/var/lib/dpkg/status`, and an unreadable global APT retry configuration. Ordinary package download/update attempts failed with missing packages, missing list directories or permission errors.

The existing system had X11 support libraries, xkbcomp, xauth, Vulkan loader 1.4.309 and LLVM 19.1.7. It did not have Xvfb or a Vulkan ICD. No X server, display variable or `/tmp/.X11-unix` socket was present.

### Recovered route

Use an entirely task-local APT configuration with local `Dir::Etc`, configuration parts, source list, package state, status file, lists, cache and logs. Use the system Debian archive keyring and the official signed `https://deb.debian.org/debian` trixie main index. Invoke APT with `APT_CONFIG` set to this local file, download packages without installing them globally, and extract with `dpkg-deb -x` into the local prefix.

No global APT configuration, permissions, network policy or system package installation was changed.

The successful package set was:

| Package | Version |
| --- | --- |
| mesa-vulkan-drivers | 25.0.7-2+deb13u1 |
| vulkan-tools | 1.4.304.0+dfsg1-1 |
| xvfb | 2:21.1.16-1.3+deb13u4 |
| xserver-common | 2:21.1.16-1.3+deb13u4 |

All linked libraries for the extracted Xvfb, vulkaninfo and libvulkan_lvp.so resolved against the scoped prefix and existing system libraries. Package indices were signature-checked by APT; package archive hashes are in the local provisioning record.

### Remaining display boundary

Xvfb failed before opening a display. A scoped diagnostic established that creating an AF_UNIX socket returns `EPERM` even inside the writable task directory. This is an execution-environment restriction, not another missing X11 dependency. No X server remains running.

Do not repeat the unchanged Xvfb route or change permission/network controls. The available route is native offscreen rendering: disable the window event-loop plugin for the smoke path, render the same scene to an image target, and capture the GPU readback. This must still execute the real renderer and scene; device enumeration alone does not prove the game renders correctly.

### Verified Vulkan result

The extracted `vulkaninfo --summary` completed with exit code 0 and reported:

- Vulkan instance version: **1.4.309**.
- Device: **llvmpipe (LLVM 19.1.7, 256 bits)**.
- Device type: CPU.
- Device API version: **1.4.305**.
- Driver: **Mesa 25.0.7-2+deb13u1**.
- Headless-surface extension available.

Software Vulkan is appropriate for correctness and screenshot checks. Its timings cannot substantiate performance claims for a dedicated gaming GPU.

### Exact local checkpoint

Prefix: `/workspace/scratch/f01d9d0c81e4/system-packages`.

```bash
source /workspace/scratch/f01d9d0c81e4/system-packages/env.sh
vulkaninfo --summary
```

The environment script clears display variables, sets `XDG_RUNTIME_DIR` to a private task runtime directory, points `LD_LIBRARY_PATH` at the extracted libraries, and selects the scoped `lvp_icd.json` through `VK_DRIVER_FILES` and `VK_ICD_FILENAMES`. It sets `WGPU_BACKEND=vulkan`, `LIBGL_ALWAYS_SOFTWARE=1` and `LP_NUM_THREADS=2`. The thread setting limits contention with concurrent builds; it is not a game setting or performance benchmark configuration.

Local evidence:

- `system-packages/runtime/vulkaninfo-summary.txt`
- `system-packages/runtime/provisioning.json`
- `system-packages/runtime/xvfb.log`
- `system-packages/apt/apt.conf`
- `system-packages/env.sh`

The package prefix is reproducible build-runner material and does not belong in the game source or distribution. Reuse this checkpoint while the runner exists. If it changes, validate library resolution and enumerate the selected device again before attributing a native-render failure to game code.

### Invalidation and next action

Reassess this route if the base OS, architecture, Mesa/LLVM linkage, Vulkan loader or socket capability changes. Do not assume that a different runner shares the X11 restriction.

The Direvanity optimized package independently passed this same offscreen route at 185 frames /
tick 354, with ten campaign commands, exact save roundtrip and validated HUD; checksum
`6ca474ed5e8d0a66`. Its 1440 × 900 creator and campaign captures were reviewed. The separate legacy
smoke passed 120 frames and three commands. Exact package lineage is recorded in the current
checkpoint above. Reuse the environment rather than reprovisioning. Windows target-hardware
rendering and interactive input capture remain outstanding.

## Historical alpha.1 portable Linux baseline

Recorded 2026-11-25 UTC. The first optimized Linux build used `ubuntu-latest`. Its CI renderer smoke
passed, but the downloaded executable failed on the Debian 13 validation host before initialization
with `GLIBC_2.43 not found`. Host glibc is 2.41. The rejected executable's SHA-256 is
`65f2f9bc52af4e8082268018d40b7cbfd886484166568df29cbb0866296db4c0`.

The correction pins the Linux release builder to **Ubuntu 24.04** and inspects ELF versioned symbols
during packaging. The package gate rejects a maximum glibc requirement above **2.39**. This keeps
the release's build baseline explicit; passing a renderer test on the build runner alone is not
sufficient portability evidence.

Portable workflow run `45567826892`, job `139046854131`, artifact `15001077210` built source
`93c51dc4ede21580c7bc90ba92d98b48cb282844`. Verification downloaded the artifact through GitHub,
matched its provider digest, validated the inner ZIP's bundled SHA and CRC, extracted it into a
fresh directory, checked its executable mode, and independently inspected the extracted executable.
The actual glibc requirement is **2.35**. No local rebuild replaced the downloaded executable.

The new binary SHA-256 is `c89dd59542129e18efef6423e09e655afa9862723deb5510734ff28c0fba5465`.
It completed the following command under the scoped environment, with exit code 0:

```bash
source /workspace/scratch/f01d9d0c81e4/system-packages/env.sh
/workspace/scratch/f01d9d0c81e4/Dreadmarch/dist/portable-verified/extracted/Dreadmarch-linux-x64/dreadmarch \
  --smoke-test \
  --screenshot /workspace/scratch/f01d9d0c81e4/Dreadmarch/dist/portable-verified/native-smoke.png
```

```text
DREADMARCH_RENDER_SMOKE_OK frames=120 tick=120 units=12 checksum=4d08544a95f6c631 screenshot_captured=true commands_ok=3 save_roundtrip=true hud_validated=true
```

The new screenshot was visually inspected. Exact hashes, paths, CI provenance, host details and
verification limits are recorded in `dist/portable-verified/receipt.json`; the accompanying log and
PNG are in the same directory. [HISTORY_ALPHA1.md](HISTORY_ALPHA1.md) retains the key public
evidence. Do not reuse the rejected Ubuntu-latest executable or count its CI-only smoke as the
portable artifact result.

## Historical alpha.2 Windows runtime dependency

The alpha.2 Windows x64 package contains a 76,911,616-byte executable built from
`a96d473d834dac66b0c0f0977300ed6dca45899e`, SHA-256
`ebcf6ca79a6f4c87b872c02d07ecd47da9e6411a12c614914f8cd7fe46683349`. Its PE imports include
`VCRUNTIME140.dll`. If Windows reports that DLL missing, install the
[official Microsoft Visual C++ x64 Redistributable](https://aka.ms/vc14/vc_redist.x64.exe), then launch
the game again. Consult [Microsoft's supported-runtime guide](https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist/)
for the current runtime requirements. No Microsoft runtime DLLs are bundled with the game.

Windows CI passed the native build, strict Clippy and four regression tests. These are distinct
from running the game through a Windows graphics driver; Windows GPU runtime and interactive
windowed-input verification remain part of R1's hardware capture work.
