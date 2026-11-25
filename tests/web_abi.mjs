import { readFileSync } from 'node:fs';
import assert from 'node:assert/strict';

const bytes = readFileSync(process.argv[2] || 'target/wasm32-unknown-unknown/wasm-release/dread_wasm.wasm');
const encoder = new TextEncoder();
const decoder = new TextDecoder();

async function engine() {
  return (await WebAssembly.instantiate(bytes, {})).instance.exports;
}

function output(api, length = api.output_len()) {
  // Acquire the current buffer AFTER Rust calls. Allocations can grow memory
  // and detach a previously cached Uint8Array view.
  return decoder.decode(new Uint8Array(api.memory.buffer, api.output_buffer(), length));
}

function save(api) { return output(api, api.save_game()); }
function snapshot(api) { return JSON.parse(output(api, api.snapshot())); }

function input(api, value) {
  const encoded = encoder.encode(typeof value === 'string' ? value : JSON.stringify(value));
  const pointer = api.input_buffer(encoded.length);
  new Uint8Array(api.memory.buffer, pointer, encoded.length).set(encoded);
}

function assertSameWorld(a, b) {
  assert.equal(a.checksum_low(), b.checksum_low());
  // Full byte equality checks the upper bits omitted by checksum_low as well.
  assert.equal(save(a), save(b));
}

const a = await engine();
const b = await engine();
a.new_game(42);
b.new_game(42);
const initial = snapshot(a);
assert.equal(initial.units.length, 16);
assert.equal(initial.status, 'playing');
const ids = initial.units.filter(unit => unit.team === 'player').map(unit => unit.id);
const move = { type: 'move', units: ids, target: { x: 25500, z: 18500 }, attack_move: true };
for (const api of [a, b]) {
  input(api, move);
  assert.equal(api.command(), 0);
  api.step(150);
}
assertSameWorld(a, b);
assert.notDeepEqual(snapshot(a).units[0].position, initial.units[0].position);

// Saves are opaque UTF-8, never JSON.parse -> JSON.stringify. Native u64 RNG
// state exceeds JavaScript Number's exact-integer range; parsing is for UI only.
const saved = save(a);
const rng = BigInt(saved.match(/"rng":(\d+)/)[1]);
assert.ok(rng > BigInt(Number.MAX_SAFE_INTEGER), 'fixture must exercise u64 precision');
assert.notEqual(BigInt(JSON.parse(saved).rng), rng, 'Number conversion rounds the RNG state');
assert.notEqual(JSON.stringify(JSON.parse(saved)), saved, 'fixture exposes lossy Number conversion');
input(a, '{broken');
assert.notEqual(a.command(), 0);
assert.equal(save(a), saved);
input(a, { type: 'hold', units: [] });
assert.equal(a.command(), 2);
assert.match(output(a), /Select one or more allied units first/);
assert.equal(save(a), saved);

// Restore over a different world, proving RNG and all state are recovered.
a.new_game(99);
a.step(17);
assert.notEqual(save(a), saved);
input(a, saved);
assert.equal(a.load_game(), 0);
assertSameWorld(a, b);

for (const invalid of [
  '{broken',
  saved.replace('"version":1', '"version":9999'),
  saved.replace('"width":48', '"width":0'),
  saved.replace(/"hp":\d+/, '"hp":-1'),
]) {
  assert.notEqual(invalid, saved);
  input(a, invalid);
  assert.notEqual(a.load_game(), 0);
  assert.equal(save(a), saved, 'invalid save must not change the running world');
}

const utf8Pointer = a.input_buffer(2);
new Uint8Array(a.memory.buffer, utf8Pointer, 2).set([0xff, 0xfe]);
assert.notEqual(a.load_game(), 0);
assert.equal(save(a), saved);

for (const api of [a, b]) {
  input(api, { type: 'recruit', kind: 'ranger' });
  assert.equal(api.command(), 0);
  api.step(300);
}
assertSameWorld(a, b);
assert.equal(snapshot(a).tick, 450);

// Force allocation beyond existing memory, then send legal JSON plus trailing
// whitespace through a fresh view. Old views must not be retained by clients.
const oldBuffer = a.memory.buffer;
const oldView = new Uint8Array(oldBuffer);
const largeLength = oldBuffer.byteLength + 131072;
const pointer = a.input_buffer(largeLength);
assert.notEqual(a.memory.buffer, oldBuffer, 'fixture must trigger memory growth');
assert.equal(oldView.byteLength, 0, 'old memory views are detached after growth');
const freshView = new Uint8Array(a.memory.buffer, pointer, largeLength);
freshView.fill(32);
const hold = { type: 'hold', units: [1] };
freshView.set(encoder.encode(JSON.stringify(hold)));
assert.equal(a.command(), 0);
input(b, hold);
assert.equal(b.command(), 0);
a.step(60);
b.step(60);
assertSameWorld(a, b);
assert.equal(snapshot(a).tick, 510);

console.log(JSON.stringify({
  wasm_bytes: bytes.length,
  verified: [
    'real Rust movement',
    'two-instance deterministic full-state equality',
    'malformed command rejection',
    'player-facing command errors',
    'opaque u64-safe save roundtrip',
    'invalid JSON/schema/grid/health/UTF-8 saves preserve state',
    'recruitment',
    'memory growth with fresh pointer views',
    '510 fixed ticks',
  ],
  checksum_low_u32: a.checksum_low() >>> 0,
  checksum_low_hex: (a.checksum_low() >>> 0).toString(16).padStart(8, '0'),
}, null, 2));
