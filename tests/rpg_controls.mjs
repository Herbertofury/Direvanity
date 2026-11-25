#!/usr/bin/env node
/** Actual packaged JS + WASM with a structural DOM/Canvas mock; not visual QA. */
import assert from 'node:assert/strict';
import fs from 'node:fs';
import vm from 'node:vm';
const html=fs.readFileSync(process.argv[2]||'dist/Dreadmarch-campaign.html','utf8');
const scripts=[...html.matchAll(/<script\b[^>]*>([\s\S]*?)<\/script>/gi)].map(m=>m[1]);
const elements=new Map(), storage=new Map(), listeners=new Map();
let now=1000, nextFrame, calls=0, identities=0;
const graphics=new Proxy({}, {get:(o,k)=>k in o?o[k]:k==='createRadialGradient'?()=>({addColorStop(){}}):()=>{calls++;},set:(o,k,v)=>(o[k]=v,true)});
class Element {
  constructor(tag='div',attrs='') {
    this.tagName=tag.toUpperCase();this.children=[];this.listeners=new Map();this.dataset={};this.style={setProperty(k,v){this[k]=v;}};this.attributes=new Map();this.textContent='';this._value='';this._html='';this.className='';this.disabled=false;
    this.classList={add:c=>{this.className+=' '+c;},remove:c=>{this.className=this.className.split(/\s+/).filter(x=>x!==c).join(' ');},contains:c=>this.className.split(/\s+/).includes(c)};
    for(const m of attrs.matchAll(/([\w-]+)(?:="([^"]*)")?/g)) this.setAttribute(m[1],m[2]??'');
  }
  setAttribute(k,v){this.attributes.set(k,v);if(k==='id'){this.id=v;elements.set(v,this);}if(k==='class')this.className=v;if(k==='value')this._value=v;if(k==='disabled')this.disabled=true;if(k.startsWith('data-'))this.dataset[k.slice(5).replace(/-([a-z])/g,(_,c)=>c.toUpperCase())]=v;}
  getAttribute(k){return this.attributes.get(k);}
  get value(){return this._value||(this.tagName==='SELECT'?this.children[0]?.value||'':'');}
  set value(v){this._value=String(v);}
  get options(){return this.children;}
  get innerHTML(){return this._html;}
  set innerHTML(v){this._html=v;this.children=[];for(const m of v.matchAll(/<([a-z][\w-]*)([^>]*)>/g))this.children.push(new Element(m[1],m[2]));}
  append(e){this.children.push(e);}
  replaceChildren(...c){this.children=c;this._html='';if(this.tagName==='SELECT')this._value='';}
  querySelectorAll(selector){const key=selector.match(/^\[([\w-]+)\]$/)?.[1];return this.children.filter(e=>e.attributes.has(key));}
  addEventListener(type,fn){const a=this.listeners.get(type)||[];a.push(fn);this.listeners.set(type,a);}
  fire(type,extra={}){const e={target:this,preventDefault(){},...extra};this['on'+type]?.(e);for(const fn of this.listeners.get(type)||[])fn(e);}
  click(){if(!this.disabled)this.fire('click');}
  focus(){document.activeElement=this;}
  getContext(){return graphics;}
  getBoundingClientRect(){return{left:0,top:0,width:1440,height:900};}
}
for(const m of html.matchAll(/<([a-z][\w-]*)([^>]*\sid="[^"]+"[^>]*)>/g))new Element(m[1],m[2]);
for(const [id,value] of Object.entries({body:'athletic','hair-style':'0'}))elements.get(id).value=value;
const el=id=>{assert.ok(elements.has(id),'Missing element '+id);return elements.get(id);};
const document={hidden:false,documentElement:{style:{setProperty(){}},dataset:{}},getElementById:el,createElement:tag=>new Element(tag),addEventListener:(type,fn)=>{const a=listeners.get(type)||[];a.push(fn);listeners.set(type,a);}};
const context=vm.createContext({document,innerWidth:1440,innerHeight:900,devicePixelRatio:1,TextEncoder,TextDecoder,Uint8Array,WebAssembly,Blob,URL,Date,Math,crypto:{randomUUID:()=>`character-${++identities}`},atob:v=>Buffer.from(v,'base64').toString('binary'),performance:{now:()=>now},localStorage:{setItem:(k,v)=>storage.set(k,String(v)),getItem:k=>storage.get(k)??null},setTimeout:()=>1,clearTimeout(){},requestAnimationFrame:fn=>nextFrame=fn,console});
// Read-only observability added only to this test copy; no simulation mutation hook.
scripts[1]=scripts[1].replace(/\}\)\(\);\s*$/,"globalThis.__rpgTest={state:()=>state,paused:()=>paused,playing:()=>playing,screen,save:()=>output(api.save_game())};})();");
for(const script of scripts)vm.runInContext(script,context,{timeout:5000});
for(let i=0;i<1000&&!document.documentElement.dataset.engineReady;i++)await new Promise(r=>setTimeout(r,5));
assert.equal(document.documentElement.dataset.engineReady,'true');
const inspect=e=>vm.runInContext(e,context,{timeout:10000});
const state=()=>JSON.parse(inspect('JSON.stringify(__rpgTest.state())'));
const raw=()=>inspect('__rpgTest.save()');
const frame=ms=>{now+=ms;assert.equal(typeof nextFrame,'function');const fn=nextFrame;nextFrame=null;fn(now);};
const key=k=>{for(const fn of listeners.get('keydown')||[])fn({key:k,target:{tagName:'BODY'},preventDefault(){}});};
const cases=[];
assert.equal(el('classes').children.length,19);cases.push('all 19 class choices');
el('class-search').value='witch';el('class-search').fire('input');assert.equal(el('classes').children.length,1);el('classes').children[0].click();assert.equal(el('chosen-class').textContent,'Witch Doctor');
el('class-search').value='';el('class-search').fire('input');assert.equal(el('classes').children.length,19);cases.push('class filtering and selection');
el('character-name').value='';el('begin').click();assert.equal(inspect('__rpgTest.playing()'),false);cases.push('invalid creator input stays in creator');
el('character-name').value='Mara';el('skin').value='#704a38';el('body').value='slender';el('hair-style').value='5';el('stature').value='113';el('begin').click();frame(0);
assert.equal(state().campaign.character.name,'Mara');assert.equal(state().campaign.character.class_id,'witch_doctor');assert.equal(state().campaign.character.stature,113);assert.equal(state().campaign.character.hair_style,5);cases.push('appearance committed to authoritative Rust state');
const initial=state().units[0].position,p=JSON.parse(inspect('JSON.stringify(__rpgTest.screen({x:11500,z:18500}))'));el('world').fire('click',{clientX:p.x,clientY:p.y});frame(1500);assert.notDeepEqual(state().units[0].position,initial);cases.push('world click moves real hero');
el('pause').click();const tick=state().tick;frame(1000);assert.equal(state().tick,tick);el('skills').click();el('close-modal').click();assert.equal(inspect('__rpgTest.paused()'),true);el('pause').click();cases.push('panels preserve pre-existing pause state');
el('skills').click();const learn=el('modal-body').querySelectorAll('[data-learn]').find(b=>!b.disabled&&!b.classList.contains('locked'));learn.click();assert.equal(state().campaign.learned.length,2);el('close-modal').click();cases.push('skill buttons update actual skill points and learned tree');
key('i');assert.equal(el('modal-title').textContent,'What you carry');key('Escape');key('j');assert.equal(el('modal-title').textContent,'The story so far');key('Escape');key('m');assert.equal(el('modal-title').textContent,'The Ashen Frontier');key('Escape');cases.push('inventory, journal, world map and keyboard panel controls');
el('save').click();const first=raw(),firstKey=el('saved-characters').value;assert.equal(storage.get('dreadmarch-character:'+firstKey),first);cases.push('opaque UTF-8 save storage');
el('new-character').click();assert.equal(el('creator').classList.contains('hidden'),false);el('classes').children.find(b=>b.dataset.class==='paladin').click();el('character-name').value='Oren';el('begin').click();assert.equal(state().campaign.character.class_id,'paladin');assert.equal(el('saved-characters').children.length,2);assert.equal(storage.get('dreadmarch-character:'+firstKey),first);cases.push('independent character slots preserve previous character');
el('new-character').click();el('saved-characters').value=firstKey;el('continue').click();assert.equal(raw(),first);cases.push('choose and continue an earlier character');
el('skills').click();assert.equal(el('hud').inert,true);el('load').click();assert.equal(el('modal').classList.contains('hidden'),true);assert.equal(el('hud').inert,false);el('skills').click();el('new-character').click();assert.equal(el('modal').classList.contains('hidden'),true);assert.equal(el('creator').inert,false);el('saved-characters').value=firstKey;el('continue').click();cases.push('load and creator transitions clear modal, inert state and stale pause');
document.hidden=true;for(const fn of listeners.get('visibilitychange')||[])fn();const hiddenTick=state().tick;frame(10000);assert.equal(state().tick,hiddenTick);document.hidden=false;for(const fn of listeners.get('visibilitychange')||[])fn();assert.equal(inspect('__rpgTest.paused()'),true);cases.push('hidden tabs pause without dropping simulation ticks');
el('new-character').click();const imported=first.replace('"name":"Mara"','"name":"Imported Mara"');el('import-file').fire('change',{target:{files:[{text:async()=>imported}]}});await new Promise(r=>setTimeout(r,0));assert.equal(state().campaign.character.name,'Imported Mara');assert.equal(el('saved-characters').children.length,3);cases.push('import complete character into a new slot');
const defeated=JSON.parse(first);defeated.status='defeat';defeated.units=[];
el('import-file').fire('change',{target:{files:[{text:async()=>JSON.stringify(defeated)}]}});await new Promise(r=>setTimeout(r,0));
assert.equal(el('modal-title').textContent,'The road falls silent');el('defeat-new').click();assert.equal(inspect('__rpgTest.playing()'),false);assert.equal(el('modal').classList.contains('hidden'),true);assert.equal(el('creator').inert,false);cases.push('defeated saves always offer a working return to creator');
assert.ok(calls>100);cases.push('procedural portrait and region renderer execute');
const receipt={artifact:process.argv[2]||'dist/Dreadmarch-campaign.html',verified:cases,case_count:cases.length,canvas_calls:calls,limitation:'DOM/Canvas structural mocks; real browser visual QA is separate.'};
fs.mkdirSync('dist',{recursive:true});fs.writeFileSync('dist/rpg-controls-proof.json',JSON.stringify(receipt,null,2)+'\n');console.log(JSON.stringify(receipt,null,2));
