#!/usr/bin/env node
/** Real Rust/WASM campaign acceptance. No HP, inventory, XP or world edits. */
import { readFileSync, writeFileSync, mkdirSync } from 'node:fs';
import assert from 'node:assert/strict';

const bytes = readFileSync(process.argv[2] || 'target/wasm32-unknown-unknown/wasm-release/dread_wasm.wasm');
const encoder = new TextEncoder(), decoder = new TextDecoder();
const instance = async () => (await WebAssembly.instantiate(bytes, {})).instance.exports;
const output = (a, n = a.output_len()) => decoder.decode(new Uint8Array(a.memory.buffer, a.output_buffer(), n));
const input = (a, value) => {
  const b = encoder.encode(typeof value === 'string' ? value : JSON.stringify(value));
  const p = a.input_buffer(b.length);
  new Uint8Array(a.memory.buffer, p, b.length).set(b);
};
const snap = a => JSON.parse(output(a, a.snapshot()));
const save = a => output(a, a.save_game());
const pois = a => JSON.parse(output(a, a.campaign_locations()));
const hero = a => snap(a).units.find(u => u.kind === 'commander');
const distance = (a,b) => Math.hypot(a.x-b.x, a.z-b.z);
function cmd(a, value) { input(a, value); const rc = a.command(); assert.equal(rc, 0, `${JSON.stringify(value)}: ${output(a)}`); }
const creator = id => ({ name: `Élan ${id}`, class_id: id, body: 'stocky', skin: '#945e45', hair: '#d9d0c1', hair_style: 4, eyes: '#74bcb5', accent: '#a86b9d', stature: 108 });
const a = await instance();
const catalogue = JSON.parse(output(a, a.class_catalogue()));
assert.equal(catalogue.classes.length, 19);
assert.equal(new Set(catalogue.classes.map(c => c.id)).size, 19);
assert.equal(catalogue.classes.reduce((n,c) => n+c.skills.length,0),174);

function learnAvailable(a, profile) {
  for (const s of profile.skills) {
    const c = snap(a).campaign;
    if (c.skill_points && !c.learned.includes(s.id) && c.level >= s.minimum_level && (!s.requires || c.learned.includes(s.requires)))
      cmd(a,{type:'learn_skill',skill:s.id});
  }
}
function fightStep(a, profile) {
  const w = snap(a), h = w.units.find(u => u.kind === 'commander');
  assert.ok(h && w.status === 'playing', `${profile.id} must survive the public-command playthrough`);
  const enemy = w.units.filter(u => u.team === 'enemy').sort((u,v)=>distance(h.position,u.position)-distance(h.position,v.position))[0];
  for (const s of profile.skills.filter(s => s.kind === 'active' && w.campaign.learned.includes(s.id))) {
    const c = snap(a).campaign;
    if (c.cooldowns[s.id] || c.focus < s.focus_cost) continue;
    if (s.effect === 'heal' && h.hp < h.max_hp * .75) cmd(a,{type:'cast_skill',skill:s.id,target:h.position});
    else if (enemy && !['heal','guard','dash'].includes(s.effect) && distance(h.position,enemy.position) <= s.range)
      cmd(a,{type:'cast_skill',skill:s.id,target:enemy.position});
  }
  a.step(15);
}
function walk(a, profile, target, tolerance=500) {
  cmd(a,{type:'move',units:[1],target,attack_move:true});
  for(let i=0;i<600;i++) {
    if(distance(hero(a).position,target) <= tolerance) return;
    fightStep(a,profile);
  }
  assert.fail(`Movement failed for ${profile.id}: ${JSON.stringify(hero(a).position)} -> ${JSON.stringify(target)}`);
}
function interact(a, profile, id) {
  const poi = pois(a).find(p=>p.id===id);
  assert.ok(poi, `Missing ${id}`);
  walk(a,profile,poi.position,1800);
  cmd(a,{type:'interact',target:id});
}
function clear(a,profile) {
  for(let i=0;i<600 && snap(a).units.some(u=>u.team==='enemy');i++) {
    const w=snap(a),h=hero(a),enemy=w.units.filter(u=>u.team==='enemy').sort((u,v)=>distance(h.position,u.position)-distance(h.position,v.position))[0];
    if(i%12===0) cmd(a,{type:'move',units:[1],target:enemy.position,attack_move:true});
    fightStep(a,profile);
  }
  assert.equal(snap(a).units.filter(u=>u.team==='enemy').length,0);
}

const playthroughs=[];
for (const profile of catalogue.classes) {
  input(a,creator(profile.id)); assert.equal(a.new_campaign(20261125),0);
  assert.deepEqual(snap(a).campaign.character,creator(profile.id));
  assert.equal(snap(a).units.length,1,'A single hero begins the campaign');
  const unchanged=save(a);
  input(a,{...creator(profile.id),name:'\n'}); assert.notEqual(a.new_campaign(1),0); assert.equal(save(a),unchanged);
  input(a,{type:'learn_skill',skill:'not_this_class'}); assert.notEqual(a.command(),0); assert.equal(save(a),unchanged);
  input(a,{type:'travel',region:'forgotten_sanctum'}); assert.notEqual(a.command(),0); assert.equal(save(a),unchanged);
  learnAvailable(a,profile);
  const skill=profile.skills[0];
  cmd(a,{type:'cast_skill',skill:skill.id,target:hero(a).position});
  const afterCast=save(a);
  input(a,{type:'cast_skill',skill:skill.id,target:hero(a).position}); assert.notEqual(a.command(),0); assert.equal(save(a),afterCast,'Cooldown rejection is atomic');
  interact(a,profile,'elder');
  interact(a,profile,'merchant');
  interact(a,profile,'chest');
  const weapon=snap(a).campaign.inventory[0]; cmd(a,{type:'equip',item:weapon.id});
  const opened=save(a);
  input(a,{type:'interact',target:'chest'}); assert.notEqual(a.command(),0); assert.equal(save(a),opened);
  interact(a,profile,'moor_gate');
  assert.equal(snap(a).campaign.region,'hollow_moor');
  clear(a,profile);
  learnAvailable(a,profile);
  interact(a,profile,'satchel');
  interact(a,profile,'shrine');
  const moorLoot=snap(a).campaign.ground_loot;
  assert.equal(moorLoot.length,6,'Every defeated enemy leaves an item');
  walk(a,profile,moorLoot[0].position,2000); cmd(a,{type:'interact',target:`loot:${moorLoot[0].item.id}`});
  const remainingLoot=snap(a).campaign.ground_loot;
  interact(a,profile,'sanctum_gate');
  clear(a,profile);
  interact(a,profile,'relic');
  assert.equal(snap(a).campaign.main_quest,2);
  interact(a,profile,'chest');
  interact(a,profile,'moor_gate');
  assert.deepEqual(snap(a).campaign.ground_loot,remainingLoot,'Uncollected items persist across region travel');
  assert.equal(snap(a).units.filter(u=>u.team==='enemy').length,0,'Enemies stay defeated on revisit');
  interact(a,profile,'home_gate');
  interact(a,profile,'merchant');
  assert.ok(snap(a).campaign.flags.includes('satchel_returned'));
  interact(a,profile,'elder');
  let w=snap(a);
  assert.equal(w.campaign.main_quest,3);
  assert.ok(w.campaign.level>=5);
  assert.ok(w.campaign.inventory.some(i=>i.name==="Concord Keeper's Seal"));
  const rewardCount=w.campaign.inventory.length,xp=w.campaign.experience,coins=w.economy.essence;
  cmd(a,{type:'interact',target:'elder'});
  w=snap(a); assert.equal(w.campaign.inventory.length,rewardCount);assert.equal(w.campaign.experience,xp);assert.equal(w.economy.essence,coins);
  const exact=save(a), b=await instance();
  input(b,exact);assert.equal(b.load_game(),0,output(b));assert.equal(save(b),exact,'Full campaign save is byte-exact across instances');
  for(const malformed of [exact.replace('"version":2','"version":999'),exact.replace('"skill_points":','"skill_points":999999,"discarded":'),exact.replace('"class_id":"'+profile.id+'"','"class_id":"missing"')]) {
    input(a,malformed);assert.notEqual(a.load_game(),0);assert.equal(save(a),exact);
  }
  cmd(a,{type:'respec'});assert.equal(snap(a).campaign.learned.length,1);assert.equal(snap(a).campaign.skill_points,snap(a).campaign.level+1);
  playthroughs.push({class:profile.id,tick:w.tick,level:w.campaign.level,health:hero(a).hp,kills:w.economy.kills,main_quest:w.campaign.main_quest,side_quest:true,checksum:(b.checksum_low()>>>0).toString(16).padStart(8,'0')});
}
mkdirSync('dist',{recursive:true});
const receipt={wasm_bytes:bytes.length,classes:19,original_adaptation_nodes:174,public_command_playthroughs:playthroughs,verified:['appearance persistence','19 complete prologue playthroughs','skill prerequisites and cooldown atomicity','equipment','main and side quest rewards once','no enemy respawn on revisit','persistent uncollected loot','opaque save roundtrips','invalid saves preserve running world','free hub respec']};
writeFileSync('dist/rpg-abi-proof.json',JSON.stringify(receipt,null,2)+'\n');
console.log(JSON.stringify(receipt,null,2));
