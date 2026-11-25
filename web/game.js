'use strict';
const $ = id => document.getElementById(id);
const canvas = $('battlefield'), ctx = canvas.getContext('2d');
const mini = $('minimap'), mc = mini.getContext('2d');
const decoder = new TextDecoder(), encoder = new TextEncoder();
const SAVE_KEY = 'dreadmarch.save.v1';
const SEED = 20261124, STEP_MS = 1000 / 30;
let api, world, previousUnits = new Map(), selected = new Set();
let width = 0, height = 0, dpr = 1, accumulator = 0, lastTime = 0;
let paused = true, started = false, attackMove = true, casting = false;
let drag = null, pan = null, hover = null, resultSeen = false, manualPaused = true;
let scale = 13, offsetX = 0, offsetY = 0, fitted = true;
let particles = [], markers = [], noticesUntil = 0, eventTick = -1, eventIndex = 0;
let fpsFrames = 0, fpsSince = 0, uiTick = -1, rawSnapshot = '';
const objectiveNames = ['THE WATCHER’S SPIRE', 'THE SHATTERED ALTAR', 'THE LAST CONCORD'];
const tone = {player:'#82dac4',enemy:'#df8065',gold:'#d8bd7c'};

function notice(text) { $('notice').textContent = text; $('notice').classList.add('show'); noticesUntil = performance.now() + 3300; }
function project(x,z,elevation=0) { return {x:offsetX + (x-z)*scale,y:offsetY + (x+z)*scale*.5-elevation*scale}; }
function unproject(x,y) { const a=(x-offsetX)/scale,b=(y-offsetY)/(scale*.5); return {x:(a+b)*.5,z:(b-a)*.5}; }
function pointAt(event) { const rect=canvas.getBoundingClientRect(); return {x:event.clientX-rect.left,y:event.clientY-rect.top}; }
function worldPoint(screen) { const p=unproject(screen.x,screen.y); return {x:Math.round(p.x*1000),z:Math.round(p.z*1000)}; }
function fit() {
  if(!world)return;
  const availableWidth=Math.max(280,width-(width>760?470:230));
  const availableHeight=Math.max(250,height-205);
  scale=Math.min(availableWidth/(world.map.width+world.map.height),availableHeight/((world.map.width+world.map.height)*.5));
  scale=Math.max(5,scale);
  const midX=(width>760?width*.53:width*.52),midY=(height+45)*.5;
  offsetX=midX-(world.map.width-world.map.height)*.5*scale;
  offsetY=midY-(world.map.width+world.map.height)*.25*scale;
  fitted=true;
}
function resize() { width=innerWidth;height=innerHeight;dpr=devicePixelRatio||1;canvas.width=Math.round(width*dpr);canvas.height=Math.round(height*dpr);if(fitted)fit(); }
function readWorld(advance=false) {
  if(advance && world) previousUnits=new Map(world.units.map(u=>[u.id,u.position]));
  const length=api.snapshot();rawSnapshot=decoder.decode(new Uint8Array(api.memory.buffer,api.output_buffer(),length));
  world=JSON.parse(rawSnapshot);
  const alive=new Set(world.units.filter(u=>u.hp>0&&u.team==='player').map(u=>u.id));
  selected=new Set([...selected].filter(id=>alive.has(id)));
  if(eventTick!==world.tick){eventTick=world.tick;eventIndex=0;}
  for(const event of world.events.slice(eventIndex)) eventEffect(event);
  eventIndex=world.events.length;
}
function writeInput(text) {const bytes=encoder.encode(text),ptr=api.input_buffer(bytes.length);new Uint8Array(api.memory.buffer,ptr,bytes.length).set(bytes);}
function errorText() {return decoder.decode(new Uint8Array(api.memory.buffer,api.output_buffer(),api.output_len()));}
function issue(command) {
  if(!world || world.status!=='playing')return false;
  writeInput(JSON.stringify(command));
  const result=api.command();
  if(result){notice(errorText().replaceAll('Rift pulse','Soulfire'));return false;}
  readWorld();updateUI(true);return true;
}
function selectAll() {selected=new Set(world.units.filter(u=>u.team==='player'&&u.hp>0).map(u=>u.id));updateUI(true);}
function selectHero(){const hero=world.units.find(u=>u.kind==='commander'&&u.hp>0);selected=new Set(hero?[hero.id]:[]);updateUI(true);}
function reset() {casting=false;drag=null;pan=null;hover=null;api.new_game(SEED);eventTick=-1;eventIndex=0;particles=[];markers=[];resultSeen=false;accumulator=0;readWorld();previousUnits=new Map();selectAll();fit();$('result').classList.add('hidden');$('manual').classList.add('hidden');paused=false;started=true;updateUI(true);notice('The legion is yours. Reclaim the relics.');}
function pauseToggle(){if(!started)return;paused=!paused;updateUI(true);notice(paused?'Battle paused. Orders can still be issued.':'Battle resumed.');}
function save(){try{api.snapshot();const data=decoder.decode(new Uint8Array(api.memory.buffer,api.output_buffer(),api.output_len()));localStorage.setItem(SAVE_KEY,data);notice('Complete battle saved in this browser.');}catch(error){notice('Save failed: '+error.message);}}
function load(){try{const text=localStorage.getItem(SAVE_KEY);if(!text){notice('No saved battle in this browser yet.');return;}writeInput(text);if(api.load_game()){notice('Save rejected: '+errorText());return;}casting=false;drag=null;pan=null;hover=null;particles=[];markers=[];eventTick=-1;eventIndex=0;previousUnits=new Map();readWorld();selectAll();accumulator=0;resultSeen=false;started=true;paused=false;$('menu').classList.add('hidden');$('result').classList.add('hidden');fit();updateUI(true);notice('Battle restored and validated.');}catch(error){notice('Load failed: '+error.message);}}
function eventEffect(event){
  if(event.type==='wave'){notice('Breach '+event.number+' approaches from the east.');return;}
  if(event.type==='captured'){notice('Relic claimed. Essence income increased.');markers.push({position:event.position,age:0,life:2.5,kind:'capture'});}
  if(event.type==='ability'){markers.push({position:event.position,age:0,life:.9,kind:'ability',radius:event.radius/1000});}
  if(event.type==='hit'||event.type==='died'){
    const color=event.type==='died'?(event.team==='player'?tone.player:tone.enemy):'#ebcb8c';
    const count=event.type==='died'?14:5;
    for(let i=0;i<count;i++){const a=(i/count)*Math.PI*2+world.tick*.137;particles.push({position:event.position,vx:Math.cos(a)*(1+i%3)*10,vy:Math.sin(a)*15-10,age:0,life:.3+(i%4)*.12,color});}
  }
}
function updateUI(force=false){
  if(!world||(!force&&uiTick===world.tick))return;uiTick=world.tick;
  const allies=world.units.filter(u=>u.team==='player'&&u.hp>0),hero=allies.find(u=>u.kind==='commander');
  $('essence').textContent=world.economy.essence;$('army').textContent=allies.length;
  const seconds=Math.floor(world.tick/30);$('clock').textContent=String(Math.floor(seconds/60)).padStart(2,'0')+':'+String(seconds%60).padStart(2,'0');
  $('selected').textContent=selected.size+' selected';$('commander-hp').textContent=hero?'Commander '+hero.hp+' / '+hero.max_hp:'Commander fallen';
  $('selection-kind').textContent=selected.size===1&&hero&&selected.has(hero.id)?'THE COMMANDER':'YOUR LEGION';
  $('wave').textContent='BREACH '+world.wave+' / 5';$('next-wave').textContent=world.wave<5?Math.ceil(world.wave_ticks_remaining/30)+'s':'FINAL WAVE';
  $('wave-bar').style.width=(world.wave<5?(1-world.wave_ticks_remaining/(world.wave===0?600:900))*100:100)+'%';
  $('objectives').innerHTML=world.objectives.map((o,i)=>`<div class="objective ${o.captured?'captured':''}"><i>${o.captured?'◆':'◇'}</i><div><strong>${objectiveNames[i]||('RELIC '+o.id)}</strong><small>${o.captured?'CLAIMED · ESSENCE ACTIVE':o.progress?'ATTUNING '+Math.floor(o.progress/150*100)+'%':'UNCLAIMED'}</small><div class="bar"><i style="width:${o.captured?100:o.progress/150*100}%"></i></div></div></div>`).join('');
  $('cooldown').textContent=world.commander_cooldown?Math.ceil(world.commander_cooldown/30)+'s':'READY';
  $('cast').classList.toggle('active',casting);$('cast').disabled=world.commander_cooldown>0||!hero||world.status!=='playing';
  $('attack-move').classList.toggle('active',attackMove);$('pause').textContent=paused?'▶':'Ⅱ';$('pause').setAttribute('aria-label',paused?'Resume battle':'Pause battle');
  $('vanguard').disabled=world.economy.essence<25||world.status!=='playing';$('ranger').disabled=world.economy.essence<35||world.status!=='playing';
  $('vanguard-cost').textContent='◆ 25';$('ranger-cost').textContent='◆ 35';
  if(world.status!=='playing'&&!resultSeen){resultSeen=true;paused=true;$('result-title').textContent=world.status==='victory'?'The Concord is yours.':'The commander has fallen.';$('result-copy').textContent=world.status==='victory'?'Three relics reclaimed. Five breaches silenced. Your legion holds the frontier.':'Regroup. Use Soulfire to heal the front line, recruit reinforcements, and keep the commander protected.';$('result').classList.remove('hidden');}
}
function polygon(points,fill,stroke){ctx.beginPath();points.forEach((p,i)=>i?ctx.lineTo(p.x,p.y):ctx.moveTo(p.x,p.y));ctx.closePath();if(fill){ctx.fillStyle=fill;ctx.fill();}if(stroke){ctx.strokeStyle=stroke;ctx.lineWidth=.6;ctx.stroke();}}
function diamond(x,z,size,fill,stroke,elevation=0){polygon([project(x-size,z,elevation),project(x,z-size,elevation),project(x+size,z,elevation),project(x,z+size,elevation)],fill,stroke);}
function column(x,z,s,h,color){const a=project(x-s,z-s),b=project(x+s,z-s),c=project(x+s,z+s),d=project(x-s,z+s),A=project(x-s,z-s,h),B=project(x+s,z-s,h),C=project(x+s,z+s,h),D=project(x-s,z+s,h);polygon([a,b,B,A],color[0]);polygon([b,c,C,B],color[1]);polygon([c,d,D,C],color[0]);polygon([A,B,C,D],color[2],'#81937e28');}
function terrain(){
  const {width:w,height:h,blocked}=world.map;
  for(let sum=0;sum<w+h;sum++)for(let x=0;x<w;x++){const z=sum-x;if(z<0||z>=h)continue;const noise=((x*73+z*137+x*z*11)%29)/29;
    const p=project(x,z),p1=project(x+1,z),p2=project(x+1,z+1),p3=project(x,z+1);
    let fill=`rgb(${24+noise*10},${48+noise*13},${44+noise*10})`;
    const road=(Math.abs(z-18)<2&&x<31)||(Math.abs(x-25)<1.5&&z>5)||(Math.abs(z-28)<1.5&&x>24);
    if(road)fill=`rgb(${47+noise*8},${62+noise*9},${52+noise*6})`;
    polygon([p,p1,p2,p3],fill,'#9ba98b07');
    if(blocked[z*w+x]){const edge=x===0||z===0||x===w-1||z===h-1;column(x+.5,z+.5,.47,edge?.25:.6+noise*.8,edge?['#1b302c','#142522','#2a4138']:['#334038','#25352f','#5d6654']);}
    else if(noise>.9&&!road){const q=project(x+.3,z+.6);ctx.strokeStyle='#52735b55';ctx.beginPath();ctx.moveTo(q.x,q.y);ctx.lineTo(q.x-2,q.y-3);ctx.moveTo(q.x,q.y);ctx.lineTo(q.x+2,q.y-5);ctx.stroke();}
  }
  // Original procedural field markings and ruined portal plinths.
  for(const z of [6.5,18.5,28.5]){diamond(44.5,z,1.3,'#593b3222','#9f615442');column(45,z,.35,1.8,['#514139','#2b302a','#8e6d50']);column(44,z,.35,1.8,['#514139','#2b302a','#8e6d50']);}
}
function drawObjectives(time){for(const o of world.objectives){const x=o.position.x/1000,z=o.position.z/1000,p=project(x,z);const color=o.captured?tone.player:tone.gold;
  ctx.save();ctx.translate(p.x,p.y);ctx.scale(1,.5);ctx.strokeStyle=o.captured?'#89d9bd66':'#d4b87944';ctx.lineWidth=1;ctx.beginPath();ctx.arc(0,0,2.2*scale,0,Math.PI*2);ctx.stroke();ctx.restore();
  diamond(x,z,.95,'#162b27',color+'77');column(x,z,.35,.7,['#4a5143','#29392e','#92947b']);
  const tip=project(x,z,1.9+Math.sin(time*.002+o.id)*.1);ctx.save();ctx.shadowBlur=15;ctx.shadowColor=color;polygon([{x:tip.x,y:tip.y-7},{x:tip.x+4,y:tip.y},{x:tip.x,y:tip.y+7},{x:tip.x-4,y:tip.y}],color);ctx.restore();
  if(o.progress&&!o.captured){ctx.strokeStyle=color;ctx.lineWidth=2;ctx.beginPath();ctx.ellipse(p.x,p.y,scale*1.4,scale*.7,0,-Math.PI/2,-Math.PI/2+Math.PI*2*o.progress/150);ctx.stroke();}
}}
function drawUnit(unit,time,alpha){
  if(unit.hp<=0)return;const old=previousUnits.get(unit.id)||unit.position;const x=(old.x+(unit.position.x-old.x)*alpha)/1000,z=(old.z+(unit.position.z-old.z)*alpha)/1000,p=project(x,z);
  const player=unit.team==='player',color=player?tone.player:tone.enemy,hero=unit.kind==='commander',brute=unit.kind==='brute',size=Math.max(3,scale*(hero?.35:brute?.4:.25));
  ctx.fillStyle='#0006';ctx.beginPath();ctx.ellipse(p.x,p.y+2,size*1.5,size*.5,0,0,Math.PI*2);ctx.fill();
  if(selected.has(unit.id)){ctx.strokeStyle=hero?tone.gold:'#86e1c6aa';ctx.lineWidth=1.2;ctx.beginPath();ctx.ellipse(p.x,p.y+1,size*1.7,size*.8,0,0,Math.PI*2);ctx.stroke();}
  if(unit.target&&selected.has(unit.id)){const dest=project(unit.target.x/1000,unit.target.z/1000);ctx.setLineDash([3,5]);ctx.strokeStyle='#a0d7b725';ctx.beginPath();ctx.moveTo(p.x,p.y);ctx.lineTo(dest.x,dest.y);ctx.stroke();ctx.setLineDash([]);}
  const moving=old.x!==unit.position.x||old.z!==unit.position.z,bob=moving?Math.sin(time*.015+unit.id)*.8:0,headY=p.y-size*2.4+bob;
  polygon([{x:p.x,y:headY+size*.4},{x:p.x-size*.8,y:p.y-1},{x:p.x+size*.8,y:p.y-1}],player?'#294b42':'#623a31',color+'88');
  if(hero){polygon([{x:p.x-size*.6,y:headY+size*.5},{x:p.x-size*1.5,y:p.y+3},{x:p.x+size*.7,y:p.y-2}], '#c4a25f');ctx.strokeStyle='#e2d19b';ctx.lineWidth=2;ctx.beginPath();ctx.moveTo(p.x+size*.7,p.y);ctx.lineTo(p.x+size*1.5,headY-3);ctx.stroke();}
  else if(unit.kind==='ranger'){ctx.strokeStyle='#d8ba81';ctx.lineWidth=1;ctx.beginPath();ctx.arc(p.x+size*.8,headY+size, size, -1.3,1.3);ctx.stroke();}
  else {polygon([{x:p.x-size*.7,y:headY+size*.7},{x:p.x-size*1.3,y:headY+size*1.1},{x:p.x-size*.7,y:p.y},{x:p.x-size*.2,y:headY+size*1.2}],player?'#779b8c':'#ab7160');}
  ctx.fillStyle=hero?'#e3ca8b':color;ctx.beginPath();ctx.arc(p.x,headY,size*.45,0,Math.PI*2);ctx.fill();
  if(hero){ctx.fillStyle='#f2d59c';ctx.fillRect(p.x-size*.7,headY-size*.6,size*1.4,1);}
  if(unit.hp<unit.max_hp||selected.has(unit.id)||hero){const barWidth=Math.max(15,size*3);ctx.fillStyle='#071514dd';ctx.fillRect(p.x-barWidth/2,headY-6,barWidth,2);ctx.fillStyle=hero?tone.gold:color;ctx.fillRect(p.x-barWidth/2,headY-6,barWidth*unit.hp/unit.max_hp,2);}
}
function drawEffects(delta){
  markers=markers.filter(marker=>marker.age<marker.life);for(const marker of markers){marker.age+=delta;const t=marker.age/marker.life,p=project(marker.position.x/1000,marker.position.z/1000);const radius=(marker.radius||1.2)*scale*(marker.kind==='ability'?.3+t*.9:1+t*.5);ctx.save();ctx.globalAlpha=1-t;ctx.strokeStyle=marker.kind==='move'?'#d9c58b':'#a9f4d9';ctx.lineWidth=marker.kind==='ability'?3:1;ctx.beginPath();ctx.ellipse(p.x,p.y,radius,radius*.5,0,0,Math.PI*2);ctx.stroke();if(marker.kind==='ability'){const g=ctx.createRadialGradient(p.x,p.y,0,p.x,p.y,radius);g.addColorStop(0,'#dfffe666');g.addColorStop(1,'#89d9bd00');ctx.fillStyle=g;ctx.fill();}ctx.restore();}
  particles=particles.filter(p=>p.age<p.life);for(const part of particles){part.age+=delta;const p=project(part.position.x/1000,part.position.z/1000);ctx.globalAlpha=1-part.age/part.life;ctx.fillStyle=part.color;ctx.fillRect(p.x+part.vx*part.age,p.y-5+part.vy*part.age+20*part.age*part.age,2,2);}ctx.globalAlpha=1;
}
function drawMini(){mc.clearRect(0,0,192,144);mc.fillStyle='#132a27';mc.fillRect(0,0,192,144);mc.fillStyle='#4b5746';world.map.blocked.forEach((b,i)=>{if(b)mc.fillRect((i%world.map.width)*4,Math.floor(i/world.map.width)*4,4,4);});for(const o of world.objectives){mc.fillStyle=o.captured?tone.player:tone.gold;mc.fillRect(o.position.x/250-3,o.position.z/250-3,6,6);}for(const u of world.units){if(u.hp<=0)continue;mc.fillStyle=u.team==='player'?tone.player:tone.enemy;mc.fillRect(u.position.x/250-1,u.position.z/250-1,u.kind==='commander'?3:2,u.kind==='commander'?3:2);} }
function render(time,delta){
  ctx.setTransform(dpr,0,0,dpr,0,0);const bg=ctx.createRadialGradient(width*.5,height*.4,30,width*.5,height*.5,width*.75);bg.addColorStop(0,'#223b34');bg.addColorStop(1,'#071315');ctx.fillStyle=bg;ctx.fillRect(0,0,width,height);
  if(!world)return;terrain();drawObjectives(time);
  const alpha=paused?1:Math.min(1,accumulator/STEP_MS);[...world.units].sort((a,b)=>(a.position.x+a.position.z)-(b.position.x+b.position.z)||a.id-b.id).forEach(u=>drawUnit(u,time,alpha));drawEffects(delta);drawMini();
  if(casting&&hover){const p=worldPoint(hover),s=project(p.x/1000,p.z/1000),hero=world.units.find(u=>u.kind==='commander'&&u.hp>0),valid=hero&&Math.hypot(hero.position.x-p.x,hero.position.z-p.z)<=12000;ctx.strokeStyle=valid?'#a5f4d0':'#d78a74';ctx.lineWidth=1.5;ctx.beginPath();ctx.ellipse(s.x,s.y,scale*4,scale*2,0,0,Math.PI*2);ctx.stroke();ctx.fillStyle=valid?'#9be8c915':'#e2896a15';ctx.fill();}
  if(drag&&hover){ctx.fillStyle='#98d3b919';ctx.strokeStyle='#99c9aa99';ctx.lineWidth=1;ctx.fillRect(drag.x,drag.y,hover.x-drag.x,hover.y-drag.y);ctx.strokeRect(drag.x,drag.y,hover.x-drag.x,hover.y-drag.y);}
  const vignette=ctx.createRadialGradient(width*.5,height*.48,height*.2,width*.5,height*.5,width*.8);vignette.addColorStop(0,'#0000');vignette.addColorStop(1,'#03110faa');ctx.fillStyle=vignette;ctx.fillRect(0,0,width,height);
  if(paused&&started&&$('manual').classList.contains('hidden')&&world.status==='playing'){ctx.fillStyle='#d9c58b';ctx.font='11px Arial';ctx.textAlign='center';ctx.fillText('BATTLE PAUSED · SPACE TO RESUME',width*.5,100);}
}
function frame(time){
  const delta=lastTime?(time-lastTime)/1000:0;lastTime=time;
  if(api&&started&&!paused&&world.status==='playing'){
    accumulator+=delta*1000;
    while(accumulator>=STEP_MS){api.step(1);readWorld(true);accumulator-=STEP_MS;if(world.status!=='playing')break;}
    updateUI();
  }
  render(time,delta);fpsFrames++;if(time-fpsSince>1000){$('runtime').textContent='RUST / WASM · '+Math.round(fpsFrames*1000/(time-fpsSince))+' FPS';fpsFrames=0;fpsSince=time;}
  if(time>noticesUntil)$('notice').classList.remove('show');requestAnimationFrame(frame);
}
canvas.addEventListener('contextmenu',event=>event.preventDefault());
canvas.addEventListener('pointerdown',event=>{
  if(!started||!world)return;hover=pointAt(event);canvas.setPointerCapture(event.pointerId);
  if(event.button===1){pan={...hover,offsetX,offsetY};event.preventDefault();return;}
  if(event.button===2){casting=false;const p=worldPoint(hover);if(issue({type:'move',units:[...selected].sort((a,b)=>a-b),target:p,attack_move:attackMove}))markers.push({position:p,age:0,life:1,kind:'move'});updateUI(true);return;}
  if(event.button===0&&casting){const p=worldPoint(hover);if(issue({type:'cast',target:p}))casting=false;updateUI(true);return;}
  if(event.button===0)drag={...hover,shift:event.shiftKey};
});
canvas.addEventListener('pointermove',event=>{hover=pointAt(event);if(pan){offsetX=pan.offsetX+hover.x-pan.x;offsetY=pan.offsetY+hover.y-pan.y;fitted=false;}});
canvas.addEventListener('pointerup',event=>{
  const p=pointAt(event);if(pan){pan=null;return;}if(!drag)return;
  const box=Math.hypot(p.x-drag.x,p.y-drag.y)>6;
  let found=[];for(const u of world.units){if(u.team!=='player'||u.hp<=0)continue;const q=project(u.position.x/1000,u.position.z/1000,.5);if(box?q.x>=Math.min(drag.x,p.x)&&q.x<=Math.max(drag.x,p.x)&&q.y>=Math.min(drag.y,p.y)&&q.y<=Math.max(drag.y,p.y):Math.hypot(q.x-p.x,q.y-p.y)<Math.max(13,scale*.8))found.push(u.id);}
  if(!box&&found.length>1)found=found.slice(0,1);
  if(!drag.shift)selected.clear();for(const id of found){if(drag.shift&&!box&&selected.has(id))selected.delete(id);else selected.add(id);}drag=null;updateUI(true);
});
canvas.addEventListener('pointercancel',()=>{drag=null;pan=null;});
canvas.addEventListener('wheel',event=>{event.preventDefault();const p=pointAt(event),before=unproject(p.x,p.y);scale=Math.min(55,Math.max(4,scale*Math.exp(-event.deltaY*.001)));offsetX=p.x-(before.x-before.z)*scale;offsetY=p.y-(before.x+before.z)*scale*.5;fitted=false;},{passive:false});
mini.addEventListener('click',event=>{if(!world)return;const r=mini.getBoundingClientRect(),x=(event.clientX-r.left)/r.width*world.map.width,z=(event.clientY-r.top)/r.height*world.map.height;offsetX=width*.5-(x-z)*scale;offsetY=height*.5-(x+z)*scale*.5;fitted=false;});
$('select-all').onclick=selectAll;$('select-hero').onclick=selectHero;$('attack-move').onclick=()=>{attackMove=!attackMove;updateUI(true);notice(attackMove?'Advance: squads engage enemies along the route.':'Move: squads follow the route without stopping to engage.');};
$('hold').onclick=()=>issue({type:'hold',units:[...selected].sort((a,b)=>a-b)});
$('cast').onclick=()=>{if(world.commander_cooldown){notice('Soulfire is recharging.');return;}casting=!casting;updateUI(true);notice(casting?'Choose a Soulfire target within commander range.':'Soulfire targeting cancelled.');};
$('vanguard').onclick=()=>issue({type:'recruit',kind:'vanguard'});$('ranger').onclick=()=>issue({type:'recruit',kind:'ranger'});
$('pause').onclick=pauseToggle;$('fit').onclick=fit;$('save').onclick=save;$('load').onclick=load;
$('help').onclick=()=>{manualPaused=paused;paused=true;$('manual').classList.remove('hidden');updateUI(true);};
$('close-help').onclick=()=>{$('manual').classList.add('hidden');paused=manualPaused;lastTime=performance.now();updateUI(true);};
$('restart').onclick=reset;$('play-again').onclick=reset;$('view-field').onclick=()=>{$('result').classList.add('hidden');};
$('deploy').onclick=()=>{$('menu').classList.add('hidden');started=true;paused=false;lastTime=performance.now();notice('Select your legion. Right click to advance.');updateUI(true);};
addEventListener('keydown',event=>{
  if(!api||!started||!$('manual').classList.contains('hidden'))return;
  const key=event.key.toLowerCase();if(event.repeat)return;if([' ','a','1','q','v','b','h','f','f5','f9','?','escape'].includes(key))event.preventDefault();
  if(key===' ')pauseToggle();else if(key==='a')selectAll();else if(key==='1')selectHero();else if(key==='q')$('cast').click();else if(key==='v')$('vanguard').click();else if(key==='b')$('ranger').click();else if(key==='h')$('hold').click();else if(key==='f')fit();else if(key==='f5')save();else if(key==='f9')load();else if(key==='?')$('help').click();else if(key==='escape'){casting=false;drag=null;updateUI(true);}
});
document.addEventListener('visibilitychange',()=>{lastTime=performance.now();if(document.hidden&&started&&!paused){paused=true;notice('Battle paused while the tab was away.');updateUI(true);}});
addEventListener('resize',resize);resize();requestAnimationFrame(frame);
(async()=>{try{
  const bytes=Uint8Array.from(atob(DREAD_WASM_BASE64),char=>char.charCodeAt(0));
  const result=await WebAssembly.instantiate(bytes,{});api=result.instance.exports;api.new_game(SEED);readWorld();selectAll();fit();updateUI(true);
  $('deploy').disabled=false;$('deploy').textContent='DEPLOY THE LEGION';$('loading-status').textContent='Ready · 30 Hz authoritative Rust simulation · No installation required';
  document.documentElement.dataset.engineReady='true';
}catch(error){$('loading-status').textContent='Engine could not start: '+error.message;$('deploy').textContent='ENGINE LOAD FAILED';console.error(error);}})();
