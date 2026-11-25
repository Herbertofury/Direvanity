#!/usr/bin/env node
/**
 * Structural control regression tests for the packaged, offline web client.
 * Executes its actual JavaScript and embedded Rust WASM in a Node VM.
 * DOM, Canvas, focus, layout and animation scheduling are mocked. This is not
 * browser rendering, accessibility, input-latency or target-hardware FPS proof.
 * Run: node tests/web_controls.mjs dist/Dreadmarch-playtest.html
 */
import assert from 'node:assert/strict';
import crypto from 'node:crypto';
import fs from 'node:fs';
import path from 'node:path';
import vm from 'node:vm';

const artifact = path.resolve(process.argv[2] ?? 'dist/Dreadmarch-playtest.html');
const html = fs.readFileSync(artifact, 'utf8');
const scripts = [...html.matchAll(/<script\b[^>]*>([\s\S]*?)<\/script>/gi)].map(match => match[1]);
assert.ok(scripts.length >= 2, 'The packaged page must include its WASM payload and game script.');
assert.ok(!html.includes('__DREAD_JS__'), 'The page must have been packaged before this test.');

const errors = [];
const storage = new Map();
const windowListeners = new Map();
const documentListeners = new Map();
let now = 1_000;
let pendingFrame;
let graphicsCalls = 0;

class ClassList {
  constructor(classes = '') { this.values = new Set(classes.split(/\s+/).filter(Boolean)); }
  add(value) { this.values.add(value); }
  remove(value) { this.values.delete(value); }
  contains(value) { return this.values.has(value); }
  toggle(value, force) {
    const enabled = force ?? !this.contains(value);
    if (enabled) this.add(value); else this.remove(value);
    return enabled;
  }
}

function register(listeners, type, handler) {
  const handlers = listeners.get(type) ?? [];
  handlers.push(handler);
  listeners.set(type, handlers);
}

function dispatch(listeners, type, properties = {}) {
  const event = {
    defaultPrevented: false,
    preventDefault() { this.defaultPrevented = true; },
    ...properties,
  };
  for (const handler of listeners.get(type) ?? []) handler(event);
  return event;
}

const graphics = new Proxy({}, {
  get(target, key) {
    if (key in target) return target[key];
    if (key === 'createRadialGradient') return () => ({ addColorStop() {} });
    return () => { graphicsCalls++; };
  },
  set(target, key, value) { target[key] = value; return true; },
});

class Element {
  constructor(id, tag) {
    this.id = id;
    this.classList = new ClassList(tag.match(/class="([^"]*)"/)?.[1] ?? '');
    this.disabled = /\sdisabled(?:\s|>)/.test(tag);
    this.style = {};
    this.listeners = new Map();
    this.attributes = new Map();
    this.textContent = '';
    this.innerHTML = '';
  }
  addEventListener(type, handler) { register(this.listeners, type, handler); }
  setAttribute(name, value) { this.attributes.set(name, String(value)); }
  getContext() { return graphics; }
  setPointerCapture() {}
  getBoundingClientRect() {
    return this.id === 'minimap'
      ? { left: 1_220, top: 140, width: 188, height: 141 }
      : { left: 0, top: 0, width: 1_440, height: 900 };
  }
  click() {
    if (this.disabled) return;
    this.onclick?.({ target: this });
    this.fire('click', { target: this });
  }
  fire(type, properties = {}) { return dispatch(this.listeners, type, { target: this, ...properties }); }
}

const elements = new Map();
for (const match of html.matchAll(/<[^>]*\sid="([^"]+)"[^>]*>/g)) {
  elements.set(match[1], new Element(match[1], match[0]));
}
const element = id => {
  assert.ok(elements.has(id), `Missing packaged element #${id}`);
  return elements.get(id);
};
const document = {
  hidden: false,
  documentElement: { dataset: {} },
  getElementById: element,
  addEventListener: (type, handler) => register(documentListeners, type, handler),
};
const context = vm.createContext({
  document,
  innerWidth: 1_440,
  innerHeight: 900,
  devicePixelRatio: 1,
  TextDecoder,
  TextEncoder,
  Uint8Array,
  WebAssembly,
  atob: value => Buffer.from(value, 'base64').toString('binary'),
  performance: { now: () => now },
  console: { log() {}, warn() {}, error: error => errors.push(String(error)) },
  localStorage: {
    setItem: (key, value) => storage.set(key, String(value)),
    getItem: key => storage.get(key) ?? null,
  },
  requestAnimationFrame: callback => { pendingFrame = callback; },
  addEventListener: (type, handler) => register(windowListeners, type, handler),
});
for (const [index, script] of scripts.entries()) {
  vm.runInContext(script, context, { filename: `packaged-script-${index}.js`, timeout: 5_000 });
}
const readyDeadline = Date.now() + 5_000;
while (!document.documentElement.dataset.engineReady && errors.length === 0 && Date.now() < readyDeadline) {
  await new Promise(resolve => setTimeout(resolve, 5));
}
assert.equal(document.documentElement.dataset.engineReady, 'true', `Engine did not initialize: ${errors.join('; ')}`);

const inspect = expression => vm.runInContext(expression, context, { timeout: 10_000 });
const json = expression => JSON.parse(inspect(`JSON.stringify(${expression})`));
const key = (value, repeat = false) => dispatch(windowListeners, 'keydown', { key: value, repeat });
const frame = milliseconds => {
  now += milliseconds;
  const callback = pendingFrame;
  assert.equal(typeof callback, 'function', 'The client must schedule its next frame.');
  pendingFrame = undefined;
  callback(now);
};
const pointer = (type, button, point, shiftKey = false) => element('battlefield').fire(type, {
  button, clientX: point.x, clientY: point.y, pointerId: 1, shiftKey,
});
const pointFor = unit => json(`project(${unit.position.x / 1_000},${unit.position.z / 1_000},.5)`);
const allies = () => json('world.units.filter(unit => unit.team === "player" && unit.hp > 0)');
const selectedIds = () => json('[...selected].sort((a,b) => a-b)');
const commander = () => json('world.units.find(unit => unit.kind === "commander" && unit.hp > 0)');
const clickUnit = (unit, shift = false) => {
  const point = pointFor(unit);
  pointer('pointerdown', 0, point, shift);
  pointer('pointerup', 0, point, shift);
};
const rawSnapshot = () => inspect('(() => { const size=api.snapshot(); return decoder.decode(new Uint8Array(api.memory.buffer,api.output_buffer(),size)); })()');
const checksum = () => inspect('api.checksum_low()');
const newEncounter = () => {
  element('play-again').click();
  document.hidden = false;
  frame(0);
};
const assertTransientsCleared = () => {
  assert.equal(inspect('casting'), false, 'Soulfire targeting must be cleared.');
  assert.equal(inspect('drag'), null, 'Selection dragging must be cleared.');
  assert.equal(inspect('pan'), null, 'Camera dragging must be cleared.');
  assert.equal(inspect('hover'), null, 'An old pointer target must not leak into a restored match.');
};

const results = [];
function check(name, run) {
  try {
    run();
    results.push({ name, ok: true });
  } catch (error) {
    results.push({ name, ok: false, error: error.message });
  }
}

check('Packaged engine starts and selects every initial ally', () => {
  assert.equal(element('deploy').disabled, false);
  assert.deepEqual(selectedIds(), allies().map(unit => unit.id).sort((a, b) => a - b));
});

check('Objective cards are balanced top-level siblings', () => {
  const stack = [];
  let roots = 0;
  for (const match of element('objectives').innerHTML.matchAll(/<\/?div\b[^>]*>/gi)) {
    const tag = match[0];
    if (/^<\//.test(tag)) {
      assert.ok(stack.length > 0, 'Unexpected objective closing div.');
      stack.pop();
    } else {
      const objective = /class="objective(?:\s|\")/.test(tag);
      if (objective) {
        assert.equal(stack.length, 0, 'Objective cards must not nest inside another objective.');
        roots++;
      }
      stack.push(tag);
    }
  }
  assert.equal(stack.length, 0, 'Objective markup has unclosed divs.');
  assert.equal(roots, inspect('world.objectives.length'));
});

check('Deploy hides the menu and fixed ticks advance', () => {
  element('deploy').click();
  assert.equal(element('menu').classList.contains('hidden'), true);
  assert.equal(inspect('paused'), false);
  frame(101);
  assert.equal(inspect('world.tick'), 3);
  assert.ok(graphicsCalls > 0, 'Frame execution should issue drawing calls, although pixels are mocked.');
});

check('Exact clicks pick each intended unit, including overlapping vanguards', () => {
  newEncounter();
  for (const unit of allies()) {
    clickUnit(unit);
    assert.deepEqual(selectedIds(), [unit.id], `Clicking ${unit.kind} ${unit.id} selected another unit.`);
  }
});

check('Rectangle selection includes the complete group', () => {
  newEncounter();
  const units = allies();
  const points = units.map(pointFor);
  const start = { x: Math.min(...points.map(point => point.x)) - 15, y: Math.min(...points.map(point => point.y)) - 15 };
  const end = { x: Math.max(...points.map(point => point.x)) + 15, y: Math.max(...points.map(point => point.y)) + 15 };
  pointer('pointerdown', 0, start);
  pointer('pointermove', 0, end);
  pointer('pointerup', 0, end);
  assert.deepEqual(selectedIds(), units.map(unit => unit.id).sort((a, b) => a - b));
});

check('Shift click adds and toggles only the intended unit', () => {
  newEncounter();
  const units = allies().filter(unit => unit.kind === 'vanguard');
  assert.ok(units.length >= 2, 'This fixture needs adjacent selectable vanguards.');
  clickUnit(units[0]);
  clickUnit(units[1], true);
  assert.deepEqual(selectedIds(), [units[0].id, units[1].id].sort((a, b) => a - b));
  clickUnit(units[1], true);
  assert.deepEqual(selectedIds(), [units[0].id]);
});

check('Move, advance and hold reach the authoritative simulation', () => {
  newEncounter();
  element('select-hero').click();
  if (inspect('attackMove')) element('attack-move').click();
  const before = commander().position;
  const destination = json('project(12.5,18.5)');
  pointer('pointerdown', 2, destination);
  pointer('pointerup', 2, destination);
  assert.equal(commander().order, 'move');
  frame(101);
  assert.notDeepEqual(commander().position, before);
  element('attack-move').click();
  pointer('pointerdown', 2, destination);
  pointer('pointerup', 2, destination);
  assert.equal(commander().order, 'attack_move');
  element('hold').click();
  assert.equal(commander().order, 'hold');
  assert.equal(commander().target, null);
});

check('Soulfire repeat does not cancel targeting; a valid cast changes Rust state', () => {
  newEncounter();
  key('q');
  key('q', true);
  assert.equal(inspect('casting'), true);
  const hero = commander();
  const target = json(`project(${hero.position.x / 1_000},${hero.position.z / 1_000})`);
  pointer('pointerdown', 0, target);
  pointer('pointerup', 0, target);
  assert.equal(inspect('casting'), false);
  assert.ok(inspect('world.commander_cooldown') > 0);
  assert.equal(element('cast').disabled, true);
});

check('An out-of-range cast leaves the authoritative world intact', () => {
  newEncounter();
  key('q');
  const before = rawSnapshot();
  const far = json('project(47,35)');
  pointer('pointerdown', 0, far);
  pointer('pointerup', 0, far);
  assert.equal(rawSnapshot(), before);
  assert.equal(inspect('casting'), true, 'A rejected target should allow another target selection.');
});

for (const [shortcut, kind, cost] of [['v', 'vanguard', 25], ['b', 'ranger', 35]]) {
  check(`Recruiting ${kind} once ignores repeated keydowns`, () => {
    newEncounter();
    const count = allies().length;
    const essence = inspect('world.economy.essence');
    assert.ok(essence >= cost * 2, 'The fixture must afford a second purchase to expose repeat regressions.');
    key(shortcut);
    key(shortcut, true);
    key(shortcut, true);
    assert.equal(allies().length, count + 1);
    assert.equal(inspect('world.economy.essence'), essence - cost);
  });
}

check('Space pauses once despite keyboard repeat and paused time is not simulated', () => {
  newEncounter();
  key(' ');
  key(' ', true);
  assert.equal(inspect('paused'), true);
  const tick = inspect('world.tick');
  frame(1_000);
  assert.equal(inspect('world.tick'), tick);
  key(' ');
  frame(101);
  assert.equal(inspect('world.tick'), tick + 3);
});

check('The manual restores the previous pause choice', () => {
  newEncounter();
  element('help').click();
  assert.equal(inspect('paused'), true);
  assert.equal(element('manual').classList.contains('hidden'), false);
  element('close-help').click();
  assert.equal(inspect('paused'), false);
  key(' ');
  element('help').click();
  element('close-help').click();
  assert.equal(inspect('paused'), true);
});

check('Save/load preserves exact JSON, full unit records and a large u64 RNG state', () => {
  newEncounter();
  // Trigger a real wave so the save exercises RNG state beyond JS safe integers.
  inspect('api.step(620); readWorld(); updateUI(true)');
  const before = rawSnapshot();
  assert.ok([...before.matchAll(/:\s*(\d+)(?=[,}])/g)].some(match => BigInt(match[1]) > 2n ** 53n),
    'The fixture must include an integer that cannot round-trip through a JS Number.');
  const fullUnitCount = inspect('world.units.length');
  const hash = checksum();
  element('save').click();
  assert.equal(storage.get('dreadmarch.save.v1'), before, 'Save must preserve raw Rust JSON.');
  inspect('api.step(60); readWorld(); updateUI(true)');
  element('load').click();
  assert.equal(rawSnapshot(), before);
  assert.equal(checksum(), hash);
  assert.equal(inspect('world.units.length'), fullUnitCount);
});

check('Corrupt saves are rejected without mutating the running match', () => {
  newEncounter();
  const before = rawSnapshot();
  storage.set('dreadmarch.save.v1', '{invalid json');
  element('load').click();
  assert.equal(rawSnapshot(), before);
  assert.match(element('notice').textContent, /rejected/i);
});

check('Restart clears Soulfire targeting and old pointer state', () => {
  newEncounter();
  key('q');
  pointer('pointermove', 0, { x: 700, y: 400 });
  assert.equal(inspect('casting'), true);
  element('help').click();
  element('restart').click();
  assertTransientsCleared();
  assert.equal(element('manual').classList.contains('hidden'), true);
  const unit = allies().find(candidate => candidate.kind === 'vanguard');
  clickUnit(unit);
  assert.deepEqual(selectedIds(), [unit.id]);
  assert.equal(inspect('world.commander_cooldown'), 0, 'First click after restart must select, not cast.');
});

check('Load clears casting and selection or camera gestures', () => {
  newEncounter();
  element('save').click();
  key('q');
  pointer('pointermove', 0, { x: 700, y: 400 });
  key('F9');
  assertTransientsCleared();
  pointer('pointerdown', 0, { x: 700, y: 400 });
  assert.notEqual(inspect('drag'), null);
  key('F9');
  assertTransientsCleared();
  pointer('pointerdown', 1, { x: 700, y: 400 });
  assert.notEqual(inspect('pan'), null);
  key('F9');
  assertTransientsCleared();
});

check('Pointer cancellation clears a captured selection or camera gesture', () => {
  newEncounter();
  pointer('pointerdown', 0, { x: 700, y: 400 });
  element('battlefield').fire('pointercancel');
  assert.equal(inspect('drag'), null);
  pointer('pointerdown', 1, { x: 700, y: 400 });
  element('battlefield').fire('pointercancel');
  assert.equal(inspect('pan'), null);
});

check('Tab visibility loss explicitly pauses without replaying away-time', () => {
  newEncounter();
  const tick = inspect('world.tick');
  document.hidden = true;
  dispatch(documentListeners, 'visibilitychange');
  assert.equal(inspect('paused'), true);
  now += 60_000;
  document.hidden = false;
  dispatch(documentListeners, 'visibilitychange');
  frame(0);
  assert.equal(inspect('world.tick'), tick);
});

check('The packaged client emitted no console errors', () => {
  assert.deepEqual(errors, []);
});

const failed = results.filter(result => !result.ok);
console.log(JSON.stringify({
  scope: 'Packaged JavaScript and real Rust WASM; mocked DOM/Canvas, not browser rendering proof',
  artifact: path.basename(artifact),
  artifact_sha256: crypto.createHash('sha256').update(html).digest('hex'),
  passed: results.length - failed.length,
  failed: failed.length,
  results,
}, null, 2));
if (failed.length > 0) process.exitCode = 1;
