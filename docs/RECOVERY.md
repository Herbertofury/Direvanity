# Development environment recovery

## Current RPG continuation — 25 November 2026

Resume from [STATUS.md](STATUS.md), [RPG_DIRECTION.md](RPG_DIRECTION.md) and
[CLASS_SYNTHESIS.md](CLASS_SYNTHESIS.md). The primary product is an offline single-player ARPG.
The legacy RTS is an optional regression scenario. The older records below preserve environment
repairs and alpha.1 provenance; they are not the current product milestone or release binaries.

- Public release: `v0.1.0-alpha.2`, tag commit `e4a6e9c971ead55ecc2569fb1d931f33a3522eb6`.
- Tested Rust/native source: `a96d473d834dac66b0c0f0977300ed6dca45899e`.
- Final browser text/deployment source: `f1e52718899cc621177bf84420a64cbf75910903`.
- Full CI: `45657960569`; optimized release: `45657992875`; final Pages: `45660608035`.
- Nineteen classes, 174 original nodes and three persistent regions are implemented. Full versioned
  research catalogues and their coverage boundaries are already committed; do not redo that inventory.
- Actual browser main/side-quest playthrough, all 19 public-command campaign playthroughs, native
  creator/campaign rendering and freshly extracted optimized Linux execution have passed.
- Current Linux binary: `95755e96f4e10b025de6ae011b90cc3769f2f9c04685861c7f9305e42b4840c4`.
- Current Windows binary: `ebcf6ca79a6f4c87b872c02d07ecd47da9e6411a12c614914f8cd7fe46683349`.
- Windows text was restored from exact Git blobs after CRLF checkout conversion; binary identity
  was preserved. Its distribution receipt retains the original CI artifact/package hashes.
- `CHECKPOINT.json` records the exact complete-archive source revision. The release asset
  `Dreadmarch-delivery.json` records independent archive validation and post-transfer full readback.

**Exact next implementation boundary:** R1's typed ability timeline and one complete class combat
loop in an authored region, then the contrasting-class vertical slice. Implement real targeting,
windup/commit/recovery, projectile/chain delivery, class resources and signature mechanics; preserve
all 19 creator choices and extend each class through explicit coverage tasks. Original rigged art,
branching quests, itemization and hardware performance remain production work. Do not restart an
RTS-first milestone or treat procedural visuals as finished production quality.

Use `source /workspace/scratch/f01d9d0c81e4/toolchain/env.sh` for the restored local Rust toolchain.
The scoped Vulkan environment below remains valid while its files and host are unchanged. Download
CI artifacts through the dedicated GitHub artifact-download operation, then verify the provider and
inner-package digests. Do not repeat the blocked browser artifact handoff, the installation-token
release-create HTTP403 route, or the unchanged Xvfb/AF_UNIX failure.

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

The native offscreen gates are complete for both the RPG campaign and legacy encounter. The
current extracted optimized Linux campaign run exited 0 at 185 frames/tick 354 with ten command
checks, exact save roundtrip and validated HUD; checksum `21debfd8d94ba33c`. Its 1440 × 900 creator
and campaign captures were reviewed. The separate legacy smoke passed 120 frames and three
commands. Reuse this environment rather than reprovisioning; continue the R1 RPG work above.
Windows target-hardware rendering and input capture remain outstanding.

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

## Windows runtime dependency

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
