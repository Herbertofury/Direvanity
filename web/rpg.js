'use strict';
(()=>{
const $=id=>document.getElementById(id), canvas=$('world'), ctx=canvas.getContext('2d'), portrait=$('portrait'), pc=portrait.getContext('2d');
const decode=new TextDecoder(),encode=new TextEncoder();let api,state,locations=[],journal=[],worldAtlas=[],metadataStamp='',catalogue,chosen='paladin',playing=false,paused=false,selectedSkill='',modalType='',pausedBeforeModal=false,last=performance.now(),acc=0,uiStamp='',dialogueStamp='',dialogueUntil=0,noticeTimer,zoom=1,view={x:0,y:0},terrainKey='',terrainCanvas,terrainOffset={x:0,y:0},activeSlot='',modalFocus=null,hudTick=-1,hudPaused=null,slotStamp='',nearbyStamp='',mapStamp='',motionReduced=false,geometryStamp='',mapRevision=0,progressStamp='';
const skillButtons=new Map(),localMapCache=new Map();
const skillIcons={damage:'ϟ',strike:'⚔',area:'✹',cone:'⋀',heal:'✚',ward:'⛨',summon:'☽',dash:'➶',projectile:'➶',chain:'ϟ',drain:'♦',slow:'❄',poison:'♧',nova:'✴',buff:'✧'};
const icons={warrior:'⚔',barbarian:'♜',paladin:'✦',crusader:'⛨',amazon:'➶',rogue:'⌁',assassin:'✧',demon_hunter:'⋈',monk:'☯',sorcerer:'ϟ',wizard:'✴',necromancer:'♧',witch_doctor:'☽',druid:'❧',spiritborn:'☼',blood_knight:'♦',tempest:'≈',warlock:'⛧',bard:'♫'};
const regionNames={hearthmere:'Hearthmere',hollow_moor:'The Hollow Moor',forgotten_sanctum:'The Forgotten Sanctum',ember_coast:'The Ember Coast',glasswood:'The Glasswood',drowned_archive:'The Drowned Archive',expedition:'The Shattered March'};
const regionDescriptions={hearthmere:'Sanctuary · people · training',hollow_moor:'Haunted road · lost satchel · shrine',forgotten_sanctum:'Buried temple · guardians · the Concord',ember_coast:'Broken shoreline · the next chapter',glasswood:'Ancient forest · fractured memories',drowned_archive:'Sunken history · a consequential choice',expedition:'Repeatable expeditions · escalating encounters'};
const fallbackResource={name:'Focus',color:'#77b5c5',style:'mana',maximum:100,description:'Your skills draw on Focus. Rest and regeneration replenish it.'};
function resourceInfo(){return profile()?.resource||fallbackResource;}
function applyTheme(){const pr=profile(),r=resourceInfo();document.documentElement.style.setProperty('--accent',pr.color);document.documentElement.style.setProperty('--resource',r.color);document.documentElement.style.setProperty('--resource-dark',r.color+'55');$('resource-reservoir').dataset.behavior=r.style;}
function changeMotion(){motionReduced=!motionReduced;document.documentElement.dataset.motion=motionReduced?'reduced':'full';$('motion-toggle').setAttribute('aria-pressed',String(motionReduced));$('motion-toggle').textContent='Motion: '+(motionReduced?'reduced':'full');try{localStorage.setItem('direvanity-reduced-motion',String(motionReduced));}catch{};}
function regionName(id){return regionNames[id]||String(id).split('_').map(w=>w[0]?.toUpperCase()+w.slice(1)).join(' ');}

const esc=s=>String(s).replace(/[&<>"']/g,c=>({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[c]));
function output(n){return decode.decode(new Uint8Array(api.memory.buffer,api.output_buffer(),n));}
function input(value){const bytes=encode.encode(typeof value==='string'?value:JSON.stringify(value));const ptr=api.input_buffer(bytes.length);new Uint8Array(api.memory.buffer,ptr,bytes.length).set(bytes);}
function refresh(){
  state=JSON.parse(output(api.snapshot()));
  const c=state.campaign;
  const stamp=c?JSON.stringify([c.region,c.main_quest,c.flags,c.chronicle]):'';
  if(stamp!==metadataStamp||!locations.length){
    metadataStamp=stamp;locations=JSON.parse(output(api.campaign_locations()));
    const geometry=JSON.stringify([c.region,state.map.width,state.map.height,state.map.blocked]);
    if(geometry!==geometryStamp){geometryStamp=geometry;mapRevision++;terrainKey='';localMapCache.clear();}
    journal=api.campaign_journal?JSON.parse(output(api.campaign_journal())):[];
  }
}
function command(value){if(!api||!state)return false;input(value);const result=api.command();if(result){notice(output(api.output_len()));return false;}refresh();uiStamp='';return true;}
function notice(text){$('notice').textContent=text;$('notice').classList.add('show');clearTimeout(noticeTimer);noticeTimer=setTimeout(()=>$('notice').classList.remove('show'),3200);}
function appearance(){return{name:$('character-name').value,class_id:chosen,body:$('body').value,skin:$('skin').value,hair:$('hair').value,hair_style:Number($('hair-style').value),eyes:$('eyes').value,accent:$('accent').value,stature:Number($('stature').value)};}
function profile(){return catalogue.classes.find(c=>c.id===(state?.campaign?.character.class_id||chosen));}
function drawClasses(){const q=$('class-search').value.trim().toLowerCase();$('classes').replaceChildren();for(const c of catalogue.classes){if(q&&!`${c.name} ${c.aliases.join(' ')}`.toLowerCase().includes(q))continue;const b=document.createElement('button');b.className='class-choice';b.setAttribute('aria-pressed',String(c.id===chosen));b.dataset.class=c.id;b.innerHTML=`<b aria-hidden="true">${icons[c.id]||'✧'}</b>${esc(c.name)}`;b.addEventListener('click',()=>choose(c.id));$('classes').append(b);}}
function choose(id){
  chosen=id;const c=catalogue.classes.find(c=>c.id===id);
  $('chosen-class').textContent=c.name;$('class-description').textContent=c.description;
  $('class-kicker').textContent=c.combat_style.toUpperCase()+' / '+c.branches.length+' DISCIPLINES';
  $('branches').innerHTML=c.branches.map(b=>`<span>${esc(b)}</span>`).join('');
  applyTheme();const r=resourceInfo();$('creator-resource').innerHTML=`<strong>${esc(r.name.toUpperCase())}</strong>${esc(r.description)}`;
  drawClasses();drawPortrait();
}
function humanoid(g,x,y,size,look,enemy=false){
  g.save();g.translate(x,y);const s=size*(look.stature||100)/100;g.scale(s,s);
  const broad=look.body==='stocky'?1.24:look.body==='slender'?.82:1;
  const stride=motionReduced?0:Math.sin((look._stride||0)*.38)*3,lean=motionReduced?0:Math.sin((look._stride||0)*.19)*.7;
  const classId=look.class_id||'',caster=['sorcerer','wizard','necromancer','witch_doctor','warlock','druid','bard'].includes(classId),ranged=['amazon','rogue','demon_hunter'].includes(classId),shield=['paladin','crusader','warrior'].includes(classId);
  g.fillStyle='#02080a75';g.beginPath();g.ellipse(2,3,17*broad,6,0,0,Math.PI*2);g.fill();
  // The silhouette is vector-authored original art; class equipment stays cosmetic.
  const cloak=g.createRadialGradient(0,-42,3,0,-38,47);cloak.addColorStop(0,look.accent);cloak.addColorStop(1,'#101d24');
  g.fillStyle=cloak;g.beginPath();g.moveTo(-12*broad,-51);g.quadraticCurveTo(-20*broad,-25,-20*broad+stride*.3,-1);g.quadraticCurveTo(0,7,21*broad+stride*.4,-2);g.lineTo(11*broad,-51);g.closePath();g.fill();
  g.strokeStyle='#e6d0a725';g.lineWidth=.8;g.beginPath();g.moveTo(-11*broad,-45);g.lineTo(-13*broad,0);g.moveTo(10*broad,-45);g.lineTo(14*broad,0);g.stroke();
  for(const side of [-1,1]){
    const legX=side*5*broad,step=side*stride;
    polygon(g,[[legX-3*broad,-28],[legX+4*broad,-28],[legX+3*broad+step*.25,-5],[legX-3*broad+step*.25,-5]],'#28343a');
    polygon(g,[[legX-4*broad+step*.25,-12],[legX+4*broad+step*.25,-12],[legX+5*broad+step*.35,2],[legX-4*broad+step*.35,2]],side<0?'#3e4544':'#56605a','#1c292d');
    g.strokeStyle='#a4946b';g.lineWidth=1;g.beginPath();g.moveTo(legX-3*broad,-10);g.lineTo(legX+3*broad,-10);g.stroke();
  }
  g.translate(lean,-Math.abs(stride)*.15);
  const tunic=g.createRadialGradient(-4,-45,2,0,-35,28);tunic.addColorStop(0,look.accent);tunic.addColorStop(1,'#25383c');
  polygon(g,[[-11*broad,-53],[11*broad,-53],[13*broad,-34],[10*broad,-22],[-10*broad,-22],[-13*broad,-34]],tunic,'#111e26');
  if(!caster){
    polygon(g,[[-10*broad,-50],[0,-47],[10*broad,-50],[9*broad,-34],[0,-30],[-9*broad,-34]],enemy?'#725250':'#697775','#1b2c30');
    polygon(g,[[-9*broad,-49],[0,-46],[0,-33],[-8*broad,-36]],enemy?'#916659':'#8d9690');
    g.strokeStyle='#ccbd8f';g.lineWidth=.8;g.beginPath();g.moveTo(0,-46);g.lineTo(0,-32);g.stroke();
  }else{
    polygon(g,[[-9*broad,-52],[0,-42],[9*broad,-52],[6*broad,-31],[-6*broad,-31]],'#2a383e','#9d9971');
    g.fillStyle=look.eyes;g.beginPath();g.arc(0,-43,2.2,0,Math.PI*2);g.fill();
  }
  for(const side of [-1,1]){
    const arm=side*13*broad;
    polygon(g,[[arm-side*4,-49],[arm+side*4,-47],[arm+side*4+stride*.2,-28],[arm-side*1+stride*.2,-26]],'#27383c','#17252b');
    polygon(g,[[arm-side*5,-52],[arm+side*4,-53],[arm+side*7,-45],[arm-side*3,-43]],enemy?'#8b655a':'#7f8880','#293337');
    g.strokeStyle='#c7b083';g.lineWidth=.9;g.beginPath();g.moveTo(arm-side*3,-51);g.lineTo(arm+side*4,-50);g.stroke();
    g.fillStyle=look.skin;g.beginPath();g.ellipse(arm+side*2+stride*.2,-25,3,4.5,-side*.1,0,Math.PI*2);g.fill();
  }
  polygon(g,[[-11*broad,-29],[11*broad,-29],[11*broad,-24],[-11*broad,-24]],'#4f3e2e','#14232a');
  g.fillStyle='#c2a56c';g.fillRect(-2.5,-29,5,5);g.fillStyle='#59635b';g.fillRect(-1,-28,2,3);
  if(caster){
    g.strokeStyle='#85745a';g.lineWidth=2.3;g.beginPath();g.moveTo(20*broad,4);g.lineTo(22*broad,-70);g.stroke();
    g.strokeStyle='#d2bc84';g.lineWidth=1.3;g.beginPath();g.moveTo(18*broad,-60);g.lineTo(23*broad,-71);g.lineTo(26*broad,-60);g.stroke();
    g.fillStyle=look.eyes;g.beginPath();g.ellipse(22*broad,-66,2.8,5,0,0,Math.PI*2);g.fill();
  }else if(ranged){
    g.strokeStyle='#bcab7a';g.lineWidth=2;g.beginPath();g.moveTo(23*broad,-64);g.quadraticCurveTo(38*broad,-33,22*broad,-4);g.stroke();
    g.strokeStyle='#b6c5b5';g.lineWidth=.6;g.beginPath();g.moveTo(23*broad,-64);g.lineTo(22*broad,-4);g.stroke();
  }else{
    polygon(g,[[20*broad,-25],[18*broad,-63],[21*broad,-75],[24*broad,-63],[23*broad,-25]],'#bec9c4','#38484a');
    polygon(g,[[21*broad,-72],[22*broad,-62],[22*broad,-25],[21*broad,-25]],'#e4e3cc');
    g.fillStyle='#baa374';g.fillRect(15*broad,-28,13*broad,2.6);g.fillStyle='#69513b';g.fillRect(20*broad,-25,3*broad,11);
  }
  if(shield){
    polygon(g,[[-24*broad,-44],[-13*broad,-48],[-5*broad,-42],[-7*broad,-25],[-16*broad,-18],[-25*broad,-29]],'#6a7774','#c6b483');
    polygon(g,[[-22*broad,-42],[-14*broad,-45],[-8*broad,-40],[-10*broad,-27],[-16*broad,-22],[-22*broad,-30]],look.accent,'#243437');
    g.fillStyle='#e2d3a5';g.font='12px Georgia';g.textAlign='center';g.fillText('✦',-16*broad,-30);
  }
  g.fillStyle='#66503f';g.fillRect(-3,-57,6,7);
  const face=g.createRadialGradient(-4,-65,1,0,-62,14);face.addColorStop(0,look.skin);face.addColorStop(1,'#493a32');
  g.fillStyle=face;g.beginPath();g.moveTo(-8,-69);g.quadraticCurveTo(-10,-59,-5,-53);g.quadraticCurveTo(0,-49,5,-53);g.quadraticCurveTo(10,-60,8,-69);g.quadraticCurveTo(0,-76,-8,-69);g.fill();
  const hairStyle=look.hair_style||0;g.fillStyle=look.hair;
  if(hairStyle!==5){
    g.beginPath();g.moveTo(-9,-61);g.bezierCurveTo(-14,-79,8,-83,10,-66);g.lineTo(6,-67);g.quadraticCurveTo(0,-73,-8,-66);g.closePath();g.fill();
    if(hairStyle===0){polygon(g,[[-10,-72],[8,-73],[5,-66],[-5,-59],[-2,-68]],look.hair);}
    if(hairStyle===2){polygon(g,[[-9,-70],[-6,-64],[-8,-47],[-13,-42],[-12,-62]],look.hair);polygon(g,[[6,-70],[10,-69],[13,-42],[8,-48]],look.hair);}
    if(hairStyle===3){polygon(g,[[-3,-72],[-3,-84],[1,-89],[5,-78],[3,-71]],look.hair);}
    if(hairStyle===4){g.lineWidth=4;g.strokeStyle=look.hair;g.beginPath();g.moveTo(-9,-69);g.quadraticCurveTo(-13,-56,-8,-43);g.lineTo(-11,-31);g.stroke();g.strokeStyle='#a99c77';g.lineWidth=.8;for(let j=0;j<5;j++){g.beginPath();g.moveTo(-12,-47+j*3);g.lineTo(-8,-44+j*3);g.stroke();}}
  }
  g.fillStyle='#2a2d29';g.fillRect(-5,-64,3.8,1);g.fillRect(2,-64,3.8,1);g.fillStyle=look.eyes;g.fillRect(-4.7,-62.4,2.5,1.5);g.fillRect(2.3,-62.4,2.5,1.5);
  g.strokeStyle='#4b3a30';g.lineWidth=.7;g.beginPath();g.moveTo(.3,-62);g.lineTo(-.4,-57.7);g.lineTo(1.6,-57.3);g.moveTo(-2.3,-54.7);g.quadraticCurveTo(0,-53.8,2.3,-55);g.stroke();
  g.restore();
}
function drawPortrait(){const w=portrait.width,h=portrait.height;pc.clearRect(0,0,w,h);const glow=pc.createRadialGradient(w/2,h*.58,10,w/2,h*.58,220);glow.addColorStop(0,'#c3a66b20');glow.addColorStop(1,'#0000');pc.fillStyle=glow;pc.fillRect(0,0,w,h);pc.strokeStyle='#93805a55';for(let i=0;i<3;i++){pc.beginPath();pc.ellipse(w/2,h*.87,110+i*19,26+i*4,0,0,Math.PI*2);pc.stroke();}humanoid(pc,w/2,h*.84,5.5,appearance());pc.fillStyle='#c6b07866';pc.font='50px Georgia';pc.textAlign='center';pc.fillText(icons[chosen]||'✧',w/2,100);}
function reveal(){resetModal();playing=true;paused=false;last=performance.now();acc=0;selectedSkill=learnedActives()[0]?.id||'';applyTheme();slotStamp='';nearbyStamp='';mapStamp='';hudTick=-1;$('creator').classList.add('hidden');for(const id of ['hud','quest-panel','actionbar','map-panel'])$(id).classList.remove('hidden');uiStamp='';terrainKey='';drawUI();notice('Your journey begins. Speak with Archivist Maelin.');}
function start(){input(appearance());if(api.new_campaign(20261125)){const msg=output(api.output_len());$('creator-status').textContent=msg;notice(msg);return;}activeSlot=newSlot();refresh();reveal();save(true);}
function newSlot(){return globalThis.crypto?.randomUUID?.()||Date.now().toString(36)+'-'+Math.random().toString(36).slice(2);}
function savedIndex(){try{return JSON.parse(localStorage.getItem('dreadmarch-characters-v1')||'[]');}catch{return[];}}
function listSaves(){const select=$('saved-characters'),previous=select.value;select.replaceChildren();for(const entry of savedIndex().sort((a,b)=>b.updated.localeCompare(a.updated))){const o=document.createElement('option');o.value=entry.id;o.textContent=entry.name+' · '+entry.class_name+' · Lv '+entry.level;select.append(o);}if(previous&&savedIndex().some(e=>e.id===previous))select.value=previous;else if(activeSlot)select.value=activeSlot;$('continue').disabled=!select.options.length&&!localStorage.getItem('dreadmarch-campaign-v1');}
function save(silent=false){try{if(!state?.campaign)return false;const exact=output(api.save_game());if(!activeSlot)activeSlot=newSlot();localStorage.setItem('dreadmarch-character:'+activeSlot,exact);const index=savedIndex().filter(e=>e.id!==activeSlot);index.push({id:activeSlot,name:state.campaign.character.name,class_name:profile().name,level:state.campaign.level,updated:new Date().toISOString()});localStorage.setItem('dreadmarch-characters-v1',JSON.stringify(index));listSaves();if(!silent)notice('Character, inventory and world state saved.');return true;}catch(e){notice('Save failed: '+e.message);return false;}}
function restore(raw,slot){const preview=JSON.parse(raw);if(!preview.campaign){notice('Choose a campaign character save.');return false;}input(raw);if(api.load_game()){notice(output(api.output_len()));return false;}activeSlot=slot||newSlot();refresh();chosen=state.campaign.character.class_id;reveal();notice('Saved character and world restored.');return true;}
function load(){try{const slot=playing?activeSlot:$('saved-characters').value;const raw=slot?localStorage.getItem('dreadmarch-character:'+slot):localStorage.getItem('dreadmarch-campaign-v1');if(!raw){notice('There is no saved campaign yet.');return;}if(restore(raw,slot)&&!slot)save(true);}catch(e){notice('Load failed: '+e.message);}}
async function importSave(file){try{if(!file)return;const raw=await file.text();if(restore(raw,''))save(true);}catch(e){notice('Import failed: '+e.message);}$('import-file').value='';}
function returnToCreator(){if(state?.status==='playing'&&!save(true))return;resetModal();playing=false;paused=true;$('creator').classList.remove('hidden');for(const id of ['hud','quest-panel','actionbar','dialogue','map-panel','pause-indicator'])$(id).classList.add('hidden');state=null;listSaves();choose(chosen);}
function hero(){return state?.units.find(u=>u.kind==='commander'&&u.team==='player');}
function project(p){return{x:(p.x-p.z)*.025,y:(p.x+p.z)*.0125};}
function screen(p){const r=project(p);return{x:r.x*zoom+view.x,y:r.y*zoom+view.y};}
function worldPoint(x,y){const a=(x-view.x)/zoom/.025,b=(y-view.y)/zoom/.0125;return{x:Math.round((a+b)/2),z:Math.round((b-a)/2)};}
function polygon(g,points,fill,stroke){g.beginPath();points.forEach(([x,y],i)=>i?g.lineTo(x,y):g.moveTo(x,y));g.closePath();g.fillStyle=fill;g.fill();if(stroke){g.strokeStyle=stroke;g.stroke();}}
function terrain(){const region=state.campaign.region;const key=region+':'+mapRevision+':'+state.map.width+':'+state.map.height;if(key===terrainKey)return;terrainKey=key;terrainCanvas=document.createElement('canvas');terrainCanvas.width=2400;terrainCanvas.height=1500;const g=terrainCanvas.getContext('2d');terrainOffset={x:1000,y:120};g.translate(terrainOffset.x,terrainOffset.y);const palettes={hearthmere:['#4c594e','#48564b','#515c4d'],hollow_moor:['#3b504c','#3d514d','#354a46'],forgotten_sanctum:['#444a4e','#40474b','#4a4d4e'],ember_coast:['#52524a','#4a5049','#55574b'],glasswood:['#38534a','#3b594d','#365347'],drowned_archive:['#3c4c51','#38474e','#425055'],expedition:['#454550','#42444c','#4b4852']};const colors=palettes[region]||palettes.forgotten_sanctum;const cells=state.map.blocked;for(let z=0;z<state.map.height;z++)for(let x=0;x<state.map.width;x++){const p=project({x:x*1000,z:z*1000});const blocked=cells?cells[z*state.map.width+x]:false;const road=!blocked&&((z>=17&&z<=20)||(region==='hearthmere'&&x>=9&&x<=13));const c=road?'#777164':colors[(x*7+z*11)%3];polygon(g,[[p.x,p.y],[p.x+25,p.y+12.5],[p.x,p.y+25],[p.x-25,p.y+12.5]],c,road?'#2a303249':'#18252a12');if(!blocked){const tint=road?'#b7a98225':'#a5b38c17';g.fillStyle=tint;for(let k=0;k<3;k++){const ox=((x*17+z*31+k*13)%27)-13,oy=((x*7+z*13+k*9)%13);g.fillRect(p.x+ox,p.y+6+oy,road?5:2,1);}}if(blocked){const y=p.y-30;polygon(g,[[p.x-25,y+12.5],[p.x,y+25],[p.x,p.y+25],[p.x-25,p.y+12.5]],'#283238');polygon(g,[[p.x,y+25],[p.x+25,y+12.5],[p.x+25,p.y+12.5],[p.x,p.y+25]],'#323f42');polygon(g,[[p.x,y],[p.x+25,y+12.5],[p.x,y+25],[p.x-25,y+12.5]],'#66706b');}else if((x*31+z*13)%17===0){g.fillStyle=region==='hollow_moor'?'#7a9d7a55':'#b9aa7b25';g.beginPath();g.ellipse(p.x,p.y+13,5,2,0,0,Math.PI*2);g.fill();}}}
function render(){if(!playing||!state?.campaign)return;const w=innerWidth,h=innerHeight;const dpr=devicePixelRatio||1;if(canvas.width!==Math.round(w*dpr)||canvas.height!==Math.round(h*dpr)){canvas.width=Math.round(w*dpr);canvas.height=Math.round(h*dpr);}ctx.setTransform(dpr,0,0,dpr,0,0);ctx.clearRect(0,0,w,h);ctx.fillStyle='#101b20';ctx.fillRect(0,0,w,h);const hp=hero();if(hp){const center=project(hp.position);view.x=w*.56-center.x*zoom;view.y=h*.57-center.y*zoom;}terrain();ctx.drawImage(terrainCanvas,view.x-terrainOffset.x*zoom,view.y-terrainOffset.y*zoom,terrainCanvas.width*zoom,terrainCanvas.height*zoom);const drawn=[];for(const p of locations)drawn.push({depth:p.position.x+p.position.z,poi:p});for(const u of state.units)drawn.push({depth:u.position.x+u.position.z,unit:u});drawn.sort((a,b)=>a.depth-b.depth);for(const object of drawn){if(object.poi){const p=object.poi,s=screen(p.position);ctx.strokeStyle=p.kind==='gate'?'#79b9b0':'#d6b16c';ctx.fillStyle='#12242ba8';ctx.beginPath();ctx.ellipse(s.x,s.y,22*zoom,11*zoom,0,0,Math.PI*2);ctx.fill();ctx.stroke();ctx.fillStyle=p.kind==='gate'?'#8ecdc1':'#e0c28b';ctx.font=`${Math.round(24*zoom)}px Georgia`;ctx.textAlign='center';ctx.fillText(p.kind==='person'?'♙':p.kind==='gate'?'⌁':p.kind==='relic'?'✦':p.kind==='shrine'?'♧':'◇',s.x,s.y-7*zoom);ctx.font='11px system-ui';ctx.fillStyle='#e0dfc9';ctx.fillText(p.name,s.x,s.y-39*zoom);}else{const u=object.unit,s=screen(u.position),main=u.kind==='commander';const look=main?{...state.campaign.character,_stride:u.path?.length?state.tick:0}:{body:u.kind==='brute'?'stocky':'athletic',skin:u.team==='enemy'?'#975d56':'#9bb7aa',hair:'#353232',eyes:'#e0bd78',accent:u.team==='enemy'?'#82463e':'#518c7a',stature:100,_stride:u.path?.length?state.tick:0};if(main){ctx.strokeStyle=profile().color;ctx.lineWidth=2;ctx.beginPath();ctx.ellipse(s.x,s.y,18*zoom,9*zoom,0,0,Math.PI*2);ctx.stroke();ctx.lineWidth=1;}humanoid(ctx,s.x,s.y,(main?.7:u.kind==='brute'?.8:.55)*zoom,look,u.team==='enemy');if(u.hp<u.max_hp){ctx.fillStyle='#151515';ctx.fillRect(s.x-18,s.y-51*zoom,36,4);ctx.fillStyle=u.team==='enemy'?'#bd6b58':'#80b6a0';ctx.fillRect(s.x-18,s.y-51*zoom,36*u.hp/u.max_hp,4);}}}for(const loot of state.campaign.ground_loot){const s=screen(loot.position);ctx.fillStyle=loot.item.rarity==='rare'?'#dabb70':'#e1dfc6';ctx.font='17px Georgia';ctx.textAlign='center';ctx.fillText('◆',s.x,s.y-7);}for(const e of state.events||[]){if(e.type==='ability'){const p=screen(e.position);ctx.strokeStyle=profile().color;ctx.lineWidth=3;ctx.beginPath();ctx.ellipse(p.x,p.y,e.radius*.035*zoom,e.radius*.0175*zoom,0,0,Math.PI*2);ctx.stroke();ctx.lineWidth=1;}}if(paused){ctx.fillStyle='#060e1833';ctx.fillRect(0,0,w,h);}const shade=ctx.createRadialGradient(w*.55,h*.5,Math.min(w,h)*.28,w*.5,h*.5,Math.max(w,h)*.7);shade.addColorStop(0,'#0000');shade.addColorStop(1,'#050a10aa');ctx.fillStyle=shade;ctx.fillRect(0,0,w,h);}
function learnedActives(){const c=state.campaign;return profile().skills.filter(s=>s.kind==='active'&&c.learned.includes(s.id));}
function trackedQuest(){return (state?.campaign?.region==='expedition'?journal.find(q=>q.kind==='endgame'&&q.status==='active'):null)||journal.find(q=>q.kind==='campaign'&&q.status==='active')||journal.find(q=>q.kind==='endgame'&&q.status==='active')||journal.find(q=>q.status==='return')||journal.find(q=>q.status==='active');}
function questText(c){
  const tracked=trackedQuest();if(tracked)return tracked.objective;
  return c.main_quest===0?'Speak with Archivist Maelin in Hearthmere.':c.main_quest===1?'Cross the Hollow Moor. Enter the Forgotten Sanctum and recover the Ashen Concord.':c.main_quest===2?'Return the Concord to Maelin in Hearthmere.':'The prologue is complete. Explore remaining discoveries and follow your next lead in the journal.';
}

function drawUI(){
  if(!state?.campaign)return;
  if(performance.now()>dialogueUntil)$('dialogue').classList.add('hidden');
  if(hudTick===state.tick&&hudPaused===paused&&uiStamp)return;
  hudTick=state.tick;hudPaused=paused;
  const c=state.campaign,h=hero(),pr=profile(),r=resourceInfo(),maximum=r.maximum||100;
  $('hero-name').textContent=c.character.name;$('hero-class').textContent=pr.name;
  $('region-name').textContent=regionName(c.region);$('level').textContent=`LEVEL ${c.level}`;
  $('gold').textContent=`◈ ${state.economy.essence}`;$('class-emblem').textContent=icons[pr.id]||'✧';
  $('combat-caption').textContent=pr.name.toUpperCase();$('skill-points').textContent=c.skill_points?`${c.skill_points} SKILL POINT${c.skill_points===1?'':'S'}`:'';
  const hp=h?Math.max(0,h.hp):0,maxHp=h?.max_hp||0,hpPercent=maxHp?hp/maxHp*100:0;
  $('health-label').textContent=`${hp} / ${maxHp} health`;$('health-value').textContent=hp;$('health-maximum').textContent='/ '+maxHp;
  $('health-fill').style.setProperty('--fill',hpPercent+'%');$('health-reservoir').setAttribute('aria-valuenow',String(Math.round(hpPercent)));
  $('health-reservoir').setAttribute('aria-valuetext',`${hp} of ${maxHp} health`);$('health-reservoir').className='reservoir health-reservoir'+(hpPercent<30?' low-health':'');
  $('focus-label').textContent=`${c.focus} / ${maximum} ${r.name}`;$('resource-value').textContent=c.focus;$('resource-maximum').textContent='/ '+maximum;
  $('focus-fill').style.setProperty('--fill',Math.max(0,Math.min(100,c.focus/maximum*100))+'%');$('resource-name').textContent=r.name.toUpperCase();
  $('resource-symbol').textContent=icons[pr.id]||'✧';$('resource-reservoir').setAttribute('aria-label',r.name);
  $('resource-reservoir').setAttribute('aria-valuenow',String(Math.round(c.focus/maximum*100)));$('resource-reservoir').setAttribute('aria-valuetext',`${c.focus} of ${maximum} ${r.name}`);
  $('resource-reservoir').title=r.description;
  const nextLevel=100+(c.level-1)*40,xpPercent=Math.max(0,Math.min(100,c.experience/nextLevel*100));
  $('experience-fill').style.width=xpPercent+'%';$('experience-label').textContent=`${c.experience} / ${nextLevel} EXPERIENCE · LEVEL ${c.level}`;
  $('experience-meter').setAttribute('aria-valuenow',String(Math.round(xpPercent)));$('experience-meter').setAttribute('aria-valuetext',`${c.experience} of ${nextLevel} experience`);
  $('quest-text').textContent=questText(c);$('quest-title').textContent=questTitle(c);
  const storyProgress=c.main_quest<3?c.main_quest:(c.chronicle?.chapter||0),storySteps=c.main_quest<3?3:5;
  const newProgress=storyProgress+':'+storySteps;
  if(newProgress!==progressStamp){progressStamp=newProgress;$('quest-progress').innerHTML=Array.from({length:storySteps},(_,i)=>`<i class="${storyProgress>i?'complete':''}"></i>`).join('');$('quest-progress').setAttribute('aria-label',`${storyProgress} of ${storySteps} story milestones`);}
  $('pause').textContent=paused?'Resume':'Pause';$('pause-indicator').className='pause-indicator'+(paused&&!modalType?'':' hidden');
  const active=learnedActives(),newSlotStamp=c.character.class_id+':'+active.map(s=>s.id).join(',');
  if(newSlotStamp!==slotStamp){
    slotStamp=newSlotStamp;skillButtons.clear();$('skill-slots').replaceChildren();
    for(const [i,skill] of active.entries()){
      const b=document.createElement('button');b.dataset.skill=skill.id;
      b.innerHTML=`<kbd>${i<9?i+1:'•'}</kbd><span class="ability-icon" aria-hidden="true">${skillIcons[skill.effect]||icons[pr.id]||'✧'}</span><strong>${esc(skill.name)}</strong><small></small>`;
      b.onclick=()=>{selectedSkill=skill.id;uiStamp='';drawUI();notice(skill.name+': right-click a target.');};
      $('skill-slots').append(b);skillButtons.set(skill.id,b);
    }
  }
  for(const skill of active){
    const b=skillButtons.get(skill.id),cooldown=c.cooldowns[skill.id]||0,unaffordable=c.focus<skill.focus_cost;
    b.className='slot'+(skill.id===selectedSkill?' selected':'')+(cooldown?' cooling':'')+(unaffordable?' unaffordable':'');
    b.setAttribute('aria-pressed',String(skill.id===selectedSkill));b.style.setProperty('--cooldown',String(Math.min(100,cooldown/(skill.cooldown_ticks||1)*100)));
    const description=cooldown?`${(cooldown/30).toFixed(1)}s`:`${skill.focus_cost} ${r.name.toUpperCase()}`;
    b.querySelector('small').textContent=description;
    b.title=`${skill.name} · ${skill.focus_cost} ${r.name} · ${(skill.cooldown_ticks/30).toFixed(1)}s cooldown
${skill.description}
${cooldown?'Recovering':unaffordable?'Not enough '+r.name:'Ready'} · Select, then right-click a target`;
    b.setAttribute('aria-label',`${skill.name}. ${description}. ${skill.id===selectedSkill?'Selected. ':''}${unaffordable?'Not enough '+r.name+'. ':''}Right click to cast.`);
  }
  const proximity=JSON.stringify([c.region,Math.floor((h?.position.x||0)/700),Math.floor((h?.position.z||0)/700),c.ground_loot.map(l=>l.item.id),locations.map(p=>p.id)]);
  if(proximity!==nearbyStamp){
    nearbyStamp=proximity;$('nearby').replaceChildren();
    if(h){for(const p of locations.filter(p=>Math.hypot(p.position.x-h.position.x,p.position.z-h.position.z)<5500)){
      const b=document.createElement('button');b.textContent=(p.kind==='gate'?'Travel: ':'')+p.name;b.onclick=()=>interactOrMove(p);$('nearby').append(b);
    }
    for(const l of c.ground_loot.filter(l=>Math.hypot(l.position.x-h.position.x,l.position.z-h.position.z)<4000)){
      const b=document.createElement('button');b.textContent='Pick up '+l.item.name;b.onclick=()=>command({type:'interact',target:'loot:'+l.item.id});$('nearby').append(b);
    }}
  }
  $('minimap-caption').textContent=regionName(c.region).toUpperCase()+' · M';
  const enemies=state.units.filter(u=>u.team==='enemy').length;$('local-status').textContent=c.region==='hearthmere'?'SANCTUARY · REST & RETRAIN':`${enemies} hostile${enemies===1?'':'s'} · ${c.ground_loot.length} uncollected item${c.ground_loot.length===1?'':'s'}`;
  drawLocalMap($('mini-map'));
  if(c.dialogue!==dialogueStamp){dialogueStamp=c.dialogue;dialogueUntil=performance.now()+12000;$('dialogue').textContent=c.dialogue;$('dialogue').classList.remove('hidden');}
  uiStamp='drawn';
  if(state.status==='defeat'&&modalType!=='defeat')openPanel('defeat');
}
function questTitle(c){return trackedQuest()?.title||(c.main_quest===3?'Beyond the ashes':'A light in the ashes');}
function drawLocalMap(target){
  if(!target||!state?.campaign)return;const g=target.getContext('2d'),w=target.width,h=target.height,pad=12,mw=state.map.width,mh=state.map.height,sx=(w-pad*2)/mw,sy=(h-pad*2)/mh;
  const cacheKey=mapRevision+':'+w+':'+h;
  let base=localMapCache.get(cacheKey);
  if(!base){
    base=document.createElement('canvas');base.width=w;base.height=h;const background=base.getContext('2d');
    background.fillStyle='#152126';background.fillRect(0,0,w,h);
    for(let z=0;z<mh;z++)for(let x=0;x<mw;x++){background.fillStyle=state.map.blocked[z*mw+x]?'#556055':'#263b39';background.fillRect(pad+x*sx,pad+z*sy,sx+.3,sy+.3);}
    localMapCache.set(cacheKey,base);
  }
  g.clearRect(0,0,w,h);g.drawImage(base,0,0);
  const point=p=>({x:pad+p.x/1000*sx,y:pad+p.z/1000*sy});
  for(const p of locations){const v=point(p.position);g.fillStyle=p.kind==='gate'?'#77c9c2':p.kind==='person'?'#d9c384':'#b7c597';g.beginPath();g.arc(v.x,v.y,p.kind==='gate'?3:2.5,0,Math.PI*2);g.fill();}
  for(const u of state.units){if(u.team!=='enemy')continue;const v=point(u.position);g.fillStyle='#ca7467';g.fillRect(v.x-1.3,v.y-1.3,2.6,2.6);}
  for(const l of state.campaign.ground_loot){const v=point(l.position);g.fillStyle='#ead485';g.fillRect(v.x-1,v.y-1,2,2);}
  const heroUnit=hero();if(heroUnit){const v=point(heroUnit.position);g.fillStyle='#f1e6c7';g.strokeStyle='#080e13';g.lineWidth=1;polygon(g,[[v.x,v.y-5],[v.x+4,v.y+4],[v.x,v.y+2],[v.x-4,v.y+4]],'#f1e6c7','#101619');}
  g.strokeStyle='#9d8d5955';g.strokeRect(pad-.5,pad-.5,mw*sx+1,mh*sy+1);
}

function interactOrMove(p){const h=hero();if(!h)return;if(Math.hypot(h.position.x-p.position.x,h.position.z-p.position.z)>3500){command({type:'move',units:[h.id],target:p.position,attack_move:false});notice('Approach '+p.name+', then interact.');}else{command({type:'interact',target:p.id});terrainKey='';}}
function cast(target){if(!selectedSkill)return;command({type:'cast_skill',skill:selectedSkill,target});}
function setBackgroundInert(value){for(const id of ['hud','quest-panel','actionbar','creator','map-panel'])$(id).inert=value;}
function resetModal(){setBackgroundInert(false);$('modal').classList.add('hidden');modalType='';pausedBeforeModal=false;modalFocus=null;}
function openPanel(type){
  if(!state?.campaign)return;
  if(!modalType){pausedBeforeModal=paused;modalFocus=document.activeElement;}
  modalType=type;setBackgroundInert(true);paused=true;$('modal').classList.remove('hidden');
  const c=state.campaign,pr=profile(),body=$('modal-body');
  $('modal-kicker').textContent=c.character.name+' / '+pr.name;body.replaceChildren();
  const titles={skills:'Your disciplines',inventory:'What you carry',journal:'The story so far',world:'The Ashen Frontier',help:'Walk your own path',defeat:'The road falls silent'};
  $('modal-title').textContent=titles[type]||'Journey';
  if(type==='skills'){
    const resource=resourceInfo();
    body.innerHTML=`<p class="panel-note">Level ${c.level} · ${c.skill_points} unspent skill points · ${c.experience} / ${100+(c.level-1)*40} XP to next level. Learn prerequisites in order. Training is free to reset in Hearthmere.</p><div class="tree-grid">${pr.branches.map(branch=>`<section class="tree"><h3>${esc(branch)}</h3>${pr.skills.filter(s=>s.branch===branch).map(skill=>{
      const learned=c.learned.includes(skill.id),levelLocked=c.level<skill.minimum_level,prerequisite=pr.skills.find(s=>s.id===skill.requires),needsPrior=!!skill.requires&&!c.learned.includes(skill.requires),locked=levelLocked||needsPrior;
      const why=learned?'LEARNED':levelLocked?`REQUIRES LEVEL ${skill.minimum_level}`:needsPrior?`REQUIRES ${esc(prerequisite?.name||skill.requires)}`:c.skill_points?'1 SKILL POINT':'NO UNSPENT POINTS';
      return `<button class="skill-node ${learned?'learned':locked?'locked':''}" data-learn="${esc(skill.id)}" ${learned||locked||!c.skill_points?'disabled':''}><strong>${esc(skill.name)}</strong><p>${esc(skill.description)}</p><small>${why} · ${skill.kind.toUpperCase()}${skill.kind==='active'?`<br>${skill.focus_cost} ${esc(resource.name)} · ${(skill.cooldown_ticks/30).toFixed(1)}s cooldown`:''}</small></button>`;
    }).join('')}</section>`).join('')}</div><p class="fine">These are original playable Direvanity adaptations. The source-game ability inventories are separate research.</p><button id="respec" ${c.region!=='hearthmere'?'disabled':''}>Retrain in Hearthmere</button>${c.region!=='hearthmere'?'<p class="fine">Return to the sanctuary to retrain your disciplines.</p>':''}`;
    body.querySelectorAll('[data-learn]').forEach(b=>b.onclick=()=>{if(command({type:'learn_skill',skill:b.dataset.learn}))openPanel('skills');});
    $('respec').onclick=()=>{if(command({type:'respec'})){selectedSkill=learnedActives()[0]?.id||'';openPanel('skills');}};
  }else if(type==='inventory'){
    const h=hero(),equipped=Object.values(c.equipped).length;
    body.innerHTML=`<div class="inventory-summary"><div class="stat-tile"><span>CARRIED ITEMS</span><strong>${c.inventory.length}</strong></div><div class="stat-tile"><span>EQUIPPED</span><strong>${equipped}</strong></div><div class="stat-tile"><span>COINS</span><strong>${state.economy.essence}</strong></div></div><p class="panel-note">Compare an item with the equipment in its slot. Replaced equipment stays in your pack.</p><div class="inventory-toolbar"><label>Search your belongings<input id="inventory-search" type="search" placeholder="Name, slot or rarity…"></label><label>Arrange by<select id="inventory-sort"><option value="equipped">Equipped first</option><option value="power">Highest power</option><option value="name">Name</option><option value="rarity">Rarity</option></select></label></div><div id="inventory-items"></div><p id="inventory-count" class="fine"></p>`;
    const drawItems=()=>{
      const query=$('inventory-search').value.trim().toLowerCase(),sort=$('inventory-sort').value;
      const items=c.inventory.filter(i=>`${i.name} ${i.slot} ${i.rarity}`.toLowerCase().includes(query)).slice();
      items.sort((a,b)=>sort==='power'?b.power-a.power||a.id-b.id:sort==='name'?a.name.localeCompare(b.name)||a.id-b.id:sort==='rarity'?a.rarity.localeCompare(b.rarity)||b.power-a.power:(Number(c.equipped[b.slot]===b.id)-Number(c.equipped[a.slot]===a.id))||b.power-a.power||a.id-b.id);
      $('inventory-items').innerHTML=items.length?items.map(item=>{
        const isEquipped=c.equipped[item.slot]===item.id,previous=c.inventory.find(i=>i.id===c.equipped[item.slot]),difference=item.power-(previous?.power||0);
        return `<div class="inventory-row ${isEquipped?'equipped':''}" data-rarity="${esc(item.rarity)}"><div class="inventory-item"><span class="item-emblem" aria-hidden="true">${item.slot==='weapon'?'⚔':'◈'}</span><div><strong>${esc(item.name)}</strong><p>${esc(item.rarity.toUpperCase())} · ${esc(item.slot.toUpperCase())} · +${item.power} damage</p><span class="item-comparison ${difference<0?'negative':''}">${isEquipped?'Currently equipped':`${difference>=0?'+':''}${difference} damage compared with ${esc(previous?.name||'an empty slot')}`}</span></div></div><button data-equip="${item.id}" ${isEquipped?'disabled':''}>${isEquipped?'Equipped':'Equip'}</button></div>`;
      }).join(''):`<p>${query?'Nothing in your pack matches this search.':'Your pack is empty. Search the wayfarer’s cache or collect items from fallen enemies.'}</p>`;
      $('inventory-count').textContent=`${items.length} of ${c.inventory.length} items shown`;
      $('inventory-items').querySelectorAll('[data-equip]').forEach(b=>b.onclick=()=>{if(command({type:'equip',item:Number(b.dataset.equip)}))openPanel('inventory');});
    };
    $('inventory-search').oninput=drawItems;$('inventory-sort').onchange=drawItems;drawItems();
  }else if(type==='journal'){
    drawJournal(body,c);
  }else if(type==='world'){
    drawWorldPanel(body,c);
  }else if(type==='defeat'){
    body.innerHTML='<p>Your character has fallen. Load your last save or begin a new journey.</p><div class="journal-actions"><button id="recover-save">Load saved character</button><button id="defeat-new">Return to character creator</button></div>';
    $('recover-save').onclick=()=>{closePanel();load();};$('defeat-new').onclick=returnToCreator;
  }else{
    body.innerHTML='<dl class="control-list"><dt>Left click</dt><dd>Move, approach a person, open a cache, collect loot or use a road. Click an enemy to advance into combat.</dd><dt>Right click</dt><dd>Cast the selected learned active skill at the cursor.</dd><dt>1–9</dt><dd>Select a learned active skill. Right click to cast. All learned skills also remain available as buttons.</dd><dt>Space</dt><dd>Pause or resume. Panels pause the world and restore your previous pause state.</dd><dt>K / I / J / M</dt><dd>Skills, inventory, journal and overworld.</dd><dt>Mouse wheel</dt><dd>Zoom. The camera follows your character.</dd><dt>Escape</dt><dd>Close the current panel.</dd><dt>Motion</dt><dd>The motion button reduces decorative HUD animation. Your system’s reduced-motion preference is always respected.</dd></dl><p class="panel-note">Offline and yours to keep. Saves contain your character, skills, equipment, quest progress and every visited region. Export creates an exact portable save that can be imported into another character slot.</p><p>Speak with the archivist, recover the Concord, and return. Rest at Iven or the wayside shrine. Inspect your journal for the next lead.</p>';
  }
  uiStamp='';drawUI();$('close-modal').focus();
}
function drawJournal(body,c){
  if(journal.length){
    body.innerHTML=journal.map(q=>`<article class="journal-entry ${q.status==='complete'?'complete':''}"><span class="journal-label">${esc(q.kind.toUpperCase())} · ${esc(q.status.toUpperCase())}</span><h3>${esc(q.title)}</h3><p>${esc(q.objective)}</p></article>`).join('')+`<article class="journal-entry"><span class="journal-label">LAST CONVERSATION</span><h3>Words worth remembering</h3><p>${esc(c.dialogue)}</p></article>`;
    const expedition=c.chronicle?.expedition;
    if(expedition){const details=document.createElement('section');details.className='panel-note';details.innerHTML=`<strong>Expedition record</strong><br>${expedition.completed} completed · best depth ${expedition.best_depth} · ${expedition.attempts} attempts${expedition.active?`<br>Current depth ${expedition.active.depth} · ${esc(expedition.active.modifier)} · ${expedition.active.kills} / ${expedition.active.enemies_total} enemies defeated`:''}`;body.append(details);}
    return;
  }
  body.innerHTML=`<article class="journal-entry ${c.main_quest===3?'complete':''}"><span class="journal-label">MAIN STORY · ${c.main_quest===3?'COMPLETED':'IN PROGRESS'}</span><h3>The Ashen Concord</h3><p>${esc(questText(c))}</p></article><article class="journal-entry ${c.flags.includes('satchel_returned')?'complete':''}"><span class="journal-label">SIDE STORY</span><h3>A quartermaster’s burden</h3><p>${c.flags.includes('satchel_returned')?'Iven’s satchel is returned.':c.flags.includes('satchel_found')?'Return the recovered satchel to Iven in Hearthmere.':'Speak with Quartermaster Iven. His missing satchel lies beside the southern moor trail.'}</p></article><article class="journal-entry"><span class="journal-label">LAST CONVERSATION</span><h3>Words worth remembering</h3><p>${esc(c.dialogue)}</p></article>`;
}

function drawWorldPanel(body,c){
  const regionIds=worldAtlas.length?worldAtlas.map(r=>r.id):['hearthmere','hollow_moor','forgotten_sanctum'];
  const width=900,height=regionIds.length>3?500:190;
  const positions={hearthmere:[140,105],hollow_moor:[430,90],forgotten_sanctum:[735,90],ember_coast:[160,265],glasswood:[450,265],drowned_archive:[735,265],expedition:[450,420]};
  regionIds.forEach((id,i)=>{if(!positions[id])positions[id]=[140+(i%3)*290,100+Math.floor(i/3)*165];});
  const links=worldAtlas.length?worldAtlas.flatMap(r=>(r.connections||[]).filter(to=>regionIds.includes(to)).map(to=>[r.id,to])).filter(([a,b],i,all)=>all.findIndex(([x,y])=>(x===a&&y===b)||(x===b&&y===a))===i):[['hearthmere','hollow_moor'],['hollow_moor','forgotten_sanctum']];
  const graph=`<svg class="world-route" viewBox="0 0 ${width} ${height}" role="img" aria-label="Overworld: Hearthmere connects to the Hollow Moor and Forgotten Sanctum${c.chronicle?', with the chronicle continuing through the Ember Coast, Glasswood and Drowned Archive':''}"><defs><pattern id="map-grid" width="28" height="28" patternUnits="userSpaceOnUse"><path d="M28 0H0V28" fill="none" stroke="#c4b573" stroke-opacity=".04"/></pattern></defs><rect width="100%" height="100%" fill="url(#map-grid)"/>${links.map(([a,b])=>`<path d="M${positions[a][0]},${positions[a][1]} Q${(positions[a][0]+positions[b][0])/2},${positions[a][1]-35} ${positions[b][0]},${positions[b][1]}" fill="none" stroke="#9b8b5f" stroke-width="2" stroke-dasharray="3 6"/>`).join('')}${regionIds.map((id,i)=>{const [x,y]=positions[id],current=id===c.region;return `<g transform="translate(${x} ${y})"><circle r="${current?24:18}" fill="${current?'#75603a':'#2b3c36'}" stroke="${current?'#e4c47f':'#8d8b65'}" stroke-width="${current?2:1}"/><text y="6" text-anchor="middle" fill="#e7d6aa" font-size="23">${['♜','❧','◇','≈','❧','▥','✧'][i]}</text><text y="45" text-anchor="middle" fill="${current?'#f1dcaa':'#c8c7b1'}" font-size="16">${esc(regionName(id))}</text><text y="62" text-anchor="middle" fill="#97ab9c" font-size="9">${current?'YOU ARE HERE':c.visited.includes(id)?'VISITED':'UNDISCOVERED'}</text></g>`;}).join('')}</svg>`;
  body.innerHTML=`<div class="map-section-title"><h3>The overworld</h3><span>YOUR JOURNEY ACROSS THE FRONTIER</span></div><p class="panel-note">Travel uses the marked roads and passages in the world. Defeated enemies and uncollected loot remain persistent in authored regions.</p>${graph}<div class="world-regions">${regionIds.map((id,i)=>`<section class="region-card ${id===c.region?'current':''}"><span class="symbol">${['♜','❧','◇','≈','❧','▥','✧'][i]}</span><h3>${regionName(id)}</h3><p>${regionDescriptions[id]}</p><strong>${id===c.region?'YOU ARE HERE':c.visited.includes(id)?'VISITED':'UNDISCOVERED'}</strong></section>`).join('')}</div><div class="map-section-title"><h3>${regionName(c.region)} · local map</h3><span>ROADS · PEOPLE · DISCOVERIES</span></div><div class="local-map-layout"><div><canvas id="full-local-map" class="local-map-full" width="660" height="460" aria-label="Current region map"></canvas><p class="map-key"><span>You</span><span>People / finds</span><span>Hostiles</span><span>Roads</span></p></div><div id="map-locations" class="map-locations"></div></div><p class="fine">Select a place below to approach it. When close enough, select it again to interact. The panel closes and the world resumes from its previous pause state.</p>`;
  drawLocalMap($('full-local-map'));
  for(const point of locations){const b=document.createElement('button');b.innerHTML=`${esc(point.name)} <small>${esc(point.kind.toUpperCase())}</small>`;b.onclick=()=>{closePanel();interactOrMove(point);};$('map-locations').append(b);}
}

function closePanel(){const focus=modalFocus;setBackgroundInert(false);$('modal').classList.add('hidden');modalType='';paused=pausedBeforeModal;modalFocus=null;last=performance.now();acc=0;focus?.focus();}
canvas.addEventListener('click',e=>{if(!playing||modalType||!hero())return;const h=hero();let closest;for(const p of locations){const s=screen(p.position),d=Math.hypot(e.clientX-s.x,e.clientY-(s.y-12));if(d<35&&(!closest||d<closest.d))closest={p,d};}if(closest){interactOrMove(closest.p);return;}for(const loot of state.campaign.ground_loot){const s=screen(loot.position);if(Math.hypot(e.clientX-s.x,e.clientY-s.y)<24){if(!command({type:'interact',target:'loot:'+loot.item.id}))command({type:'move',units:[h.id],target:loot.position,attack_move:false});return;}}const target=worldPoint(e.clientX,e.clientY);command({type:'move',units:[h.id],target,attack_move:true});});canvas.addEventListener('contextmenu',e=>{e.preventDefault();if(playing&&!modalType)cast(worldPoint(e.clientX,e.clientY));});canvas.addEventListener('wheel',e=>{e.preventDefault();zoom=Math.max(.5,Math.min(2.2,zoom*Math.exp(-e.deltaY*.001)));},{passive:false});
$('mini-map-open').onclick=()=>openPanel('world');$('motion-toggle').onclick=changeMotion;
try{motionReduced=localStorage.getItem('direvanity-reduced-motion')==='true';}catch{}
if(motionReduced){motionReduced=false;changeMotion();}
$('begin').onclick=start;$('continue').onclick=load;$('import-save').onclick=()=>$('import-file').click();$('import-file').onchange=e=>importSave(e.target.files[0]);$('class-search').oninput=()=>catalogue&&drawClasses();for(const id of ['character-name','body','hair-style','skin','hair','eyes','accent','stature'])$(id).addEventListener('input',()=>{$('stature-value').textContent=$('stature').value+'%';drawPortrait();});$('pause').onclick=()=>{paused=!paused;last=performance.now();acc=0;drawUI();};$('save').onclick=()=>save();$('load').onclick=load;$('export').onclick=()=>{const raw=output(api.save_game()),blob=new Blob([raw],{type:'application/json'}),url=URL.createObjectURL(blob),a=document.createElement('a');a.href=url;a.download='direvanity-character-save.json';a.click();setTimeout(()=>URL.revokeObjectURL(url),1000);notice('Exported complete character and campaign save.');};for(const [id,type]of[['skills','skills'],['inventory','inventory'],['journal','journal'],['world-map','world'],['help','help']])$(id).onclick=()=>openPanel(type);$('close-modal').onclick=closePanel;$('new-character').onclick=returnToCreator;document.addEventListener('keydown',e=>{if(modalType&&e.key==='Tab'){const controls=[...$('modal').querySelectorAll('button:not([disabled]),a[href],input,select,textarea,[tabindex="0"]')].filter(el=>el.offsetParent!==null);if(controls.length){const first=controls[0],last=controls[controls.length-1];if(e.shiftKey&&document.activeElement===first){e.preventDefault();last.focus();}else if(!e.shiftKey&&document.activeElement===last){e.preventDefault();first.focus();}}return;}if(!playing||['INPUT','SELECT','TEXTAREA'].includes(e.target.tagName))return;if(e.key==='Escape'&&modalType){closePanel();return;}if(modalType)return;const key=e.key.toLowerCase();if(key===' '){e.preventDefault();$('pause').click();}else if({k:'skills',i:'inventory',j:'journal',m:'world'}[key])openPanel({k:'skills',i:'inventory',j:'journal',m:'world'}[key]);else if(/^[1-9]$/.test(key)){const s=learnedActives()[Number(key)-1];if(s){selectedSkill=s.id;uiStamp='';notice(s.name+' selected.');}}});document.addEventListener('visibilitychange',()=>{if(document.hidden&&playing){paused=true;drawUI();}last=performance.now();acc=0;});
function frame(now){const dt=(now-last)/1000;last=now;if(playing&&api&&state?.status==='playing'&&!paused&&!document.hidden){acc+=dt;const ticks=Math.floor(acc*30);if(ticks){api.step(ticks);acc-=ticks/30;refresh();}}if(playing){render();drawUI();}requestAnimationFrame(frame);}
(async()=>{try{const bytes=Uint8Array.from(atob(DREAD_WASM_BASE64),c=>c.charCodeAt(0));const instance=await WebAssembly.instantiate(bytes,{});api=instance.instance.exports;catalogue=JSON.parse(output(api.class_catalogue()));worldAtlas=api.campaign_atlas?JSON.parse(output(api.campaign_atlas())):[];for(const region of worldAtlas){regionNames[region.id]=region.name;regionDescriptions[region.id]=region.description;}choose(chosen);$('begin').disabled=false;$('begin').textContent='BEGIN YOUR JOURNEY';$('creator-status').textContent=`${catalogue.classes.length} classes · offline single-player · local saves`;listSaves();document.documentElement.dataset.engineReady='true';requestAnimationFrame(frame);}catch(e){$('creator-status').textContent='The engine could not start: '+e.message;}})();
})();
