# Rust WebAssembly boundary

This client boundary runs `dread-sim`; it contains no JavaScript combat or navigation implementation.

## Save bytes must remain opaque

Use `save_game()` and copy the returned UTF-8 JSON bytes directly into storage or a downloaded file. `snapshot()` remains compatible and returns the same complete state, but a parsed snapshot is for display only. Do not save `JSON.stringify(JSON.parse(snapshotText))`: the simulation's `u64` RNG state exceeds JavaScript's exact `Number` integer range and that roundtrip changes it.

```js
const decoder = new TextDecoder();
const encoder = new TextEncoder();

function outputText(api, length = api.output_len()) {
  return decoder.decode(new Uint8Array(
    api.memory.buffer, api.output_buffer() >>> 0, length >>> 0,
  ));
}

const rawSave = outputText(api, api.save_game());
localStorage.setItem('dreadmarch-save', rawSave);

const bytes = encoder.encode(localStorage.getItem('dreadmarch-save'));
const pointer = api.input_buffer(bytes.length) >>> 0;
new Uint8Array(api.memory.buffer, pointer, bytes.length).set(bytes);
if (api.load_game() !== 0) throw new Error(outputText(api));
```

Always construct the JavaScript memory view **after** the Rust call that allocates or serializes. WASM memory growth detaches old ordinary `ArrayBuffer` views. The ABI test deliberately forces growth and verifies the replacement view.

## Exports

| Export | Effect |
| --- | --- |
| `new_game(seed)` | Replace the battle and clear prior input/output buffers. |
| `step(ticks)` | Advance the shared simulation by exactly this many fixed ticks. |
| `input_buffer(length)` | Allocate/resize input and return its writable pointer. |
| `command()` | Parse and apply input JSON. Returns `0` on success, `1` for malformed JSON, `2` for a rejected order. |
| `snapshot()` | Serialize complete state for UI reads; return output length. |
| `save_game()` | Serialize a complete opaque save; return output length. |
| `output_buffer()` / `output_len()` | Read the current result or error text. |
| `load_game()` | Validate the complete input save, then replace the world. A nonzero result leaves the running world unchanged. |
| `checksum_low()` | Lower 32 bits of the replay diagnostic. JavaScript should apply `>>> 0` before unsigned formatting. |

Pointers and lengths are WASM32 values. Do not retain views across subsequent allocating bridge calls. Successful commands and loads clear stale error output; rejected operations return player-facing messages.

Run `node tests/web_abi.mjs path/to/dread_wasm.wasm` to exercise movement, whole-state determinism, large-integer save fidelity, failed-load atomicity, recruitment, and memory growth against the actual compiled module.
