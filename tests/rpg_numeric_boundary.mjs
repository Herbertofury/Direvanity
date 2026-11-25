#!/usr/bin/env node
/** Valid imported numerical-boundary saves remain valid under actual public commands. */
import { readFileSync, mkdirSync, writeFileSync } from 'node:fs';
import { createHash } from 'node:crypto';
import assert from 'node:assert/strict';
const bytes=readFileSync(process.argv[2]||'target/wasm32-unknown-unknown/wasm-release/dread_wasm.wasm');
const a=(await WebAssembly.instantiate(bytes,{})).instance.exports;
const encoder=new TextEncoder(),decoder=new TextDecoder();
const input=value=>{const b=encoder.encode(typeof value==='string'?value:JSON.stringify(value));const p=a.input_buffer(b.length);new Uint8Array(a.memory.buffer,p,b.length).set(b);};
const output=(n=a.output_len())=>decoder.decode(new Uint8Array(a.memory.buffer,a.output_buffer(),n));
const save=()=>output(a.save_game());
const snapshot=()=>JSON.parse(output(a.snapshot()));
const cmd=value=>{input(value);assert.equal(a.command(),0,output());};
const profile=JSON.parse(output(a.class_catalogue())).classes.find(c=>c.id==='paladin');
input({name:'Boundary',class_id:'paladin',body:'athletic',skin:'#c69b7b',hair:'#2d2521',hair_style:0,eyes:'#81b7b0',accent:'#d9b66c',stature:100});
assert.equal(a.new_campaign(9),0);
// This deliberately constructed import tests a valid numerical edge, not normal-play progression.
const fixture=snapshot();
fixture.campaign.level=4294967295;
fixture.campaign.experience=4294967294;
fixture.campaign.learned=[profile.skills[0].id,profile.skills[1].id];
fixture.campaign.skill_points=4294967295;
input(fixture);assert.equal(a.load_game(),0,output());
cmd({type:'move',units:[1],target:{x:11500,z:18500},attack_move:true});a.step(120);
cmd({type:'interact',target:'elder'});
assert.equal(snapshot().campaign.experience,4294967295);
const earned=save();input(earned);assert.equal(a.load_game(),0,output());assert.equal(save(),earned);
cmd({type:'respec'});assert.equal(snapshot().campaign.skill_points,4294967296);
const retrained=save();input(retrained);assert.equal(a.load_game(),0,output());assert.equal(save(),retrained);
cmd({type:'learn_skill',skill:profile.skills[1].id});assert.equal(snapshot().campaign.skill_points,4294967295);
const valid=save();
input(valid.replace(/"skill_points":\d+/, '"skill_points":18446744073709551615'));
assert.notEqual(a.load_game(),0);assert.equal(save(),valid);
const receipt={result:'passed',wasm_sha256:createHash('sha256').update(bytes).digest('hex'),checks:['valid max-level import','public quest XP reward preserves loadable save','max-level retraining preserves all skill points','learning after retraining stays valid','hostile u64 skill-point overflow rejects atomically']};
mkdirSync('dist',{recursive:true});writeFileSync('dist/numeric-boundary-proof.json',JSON.stringify(receipt,null,2)+'\n');
console.log(JSON.stringify(receipt,null,2));
