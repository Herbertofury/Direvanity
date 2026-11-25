# Development environment recovery

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

The next acceptance action is a finite native offscreen game run using this environment, followed by screenshot inspection and verification of the game simulation's smoke result. Provisioning and device enumeration are complete; game rendering proof is a separate gate.
