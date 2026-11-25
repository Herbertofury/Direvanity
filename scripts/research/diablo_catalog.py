#!/usr/bin/env python3
"""Extract factual Diablo skill labels and tree structure from versioned sources.

Only labels, identifiers and structural fields enter the published catalogue;
full descriptions, art and raw source documents remain outside the repository.
Requires beautifulsoup4 for official HTML tables. Raw input directory is explicit.
"""
from __future__ import annotations
import argparse
import collections
import csv
import hashlib
import json
import re
from pathlib import Path
from urllib.parse import urljoin
from bs4 import BeautifulSoup

DATE = "2026-11-25"
D1_COMMIT = "e1c377775a206274f73b5f01c84a967d06e74d54"
D2_COMMIT = "51e7826e67515f4a84583ebaa882339aeeb99758"
D4_COMMIT = "75e619f0d90a0951945747f45feef59c076a5db0"
CLASSES = ["Warrior", "Rogue", "Sorcerer", "Monk", "Bard", "Barbarian", "Amazon", "Assassin", "Necromancer", "Paladin", "Druid", "Crusader", "Demon Hunter", "Witch Doctor", "Wizard", "Blood Knight", "Tempest", "Spiritborn", "Warlock"]


def slug(value):
    return re.sub(r"[^a-z0-9]+", "-", value.lower()).strip("-")


def source_record(path, url, **extra):
    return {"url": url, "retrieved": DATE, "sha256": hashlib.sha256(path.read_bytes()).hexdigest(), **extra}


def extract_legacy(raw):
    out=[]
    inactive={"DoomSerpents", "BloodRitual", "Invisibility", "Etherealize"}
    class_skill={"ItemRepair":"Warrior", "TrapDisarm":"Rogue", "StaffRecharge":"Sorcerer", "Rage":"Barbarian"}
    for file,game,version in [("d1-spells.tsv","Diablo I","classic table"),("d1-hf-spells.tsv","Diablo: Hellfire","expansion table")]:
        path=raw/file
        url=f"https://github.com/diasurgical/devilutionX/blob/{D1_COMMIT}/"+("assets/txtdata/spells/spelldat.tsv" if game=="Diablo I" else "mods/hf/txtdata/spells/spelldat.tsv")
        rows=list(csv.DictReader(path.read_text().splitlines(),delimiter="\t"))
        records=[]
        for row in rows:
            name=row["name"]
            unavailable=row["id"] in inactive or (game=="Diablo I" and row["id"]=="Rage")
            typ="inactive_data_entry" if unavailable else "class_skill" if row["id"] in class_skill else "rune_item_effect" if row["id"].startswith("RuneOf") else "spell"
            records.append({"id":slug(game)+":"+row["id"],"name":name,"type":typ,"class":class_skill.get(row["id"],"shared"),"tree":"spellbook / consumables / innate skills", "availability":"unused or unavailable in this edition" if unavailable else "table entry; acquisition depends on book, staff, scroll, rune or class", "book_level":int(row["bookLevel"]),"staff_level":int(row["staffLevel"]),"tags":row["flags"].split(","),"source_url":url})
        out.append({"game":game,"version":version,"classes":["Warrior","Rogue","Sorcerer"]+(["Monk","Bard","Barbarian"] if game!="Diablo I" else []),"sources":[source_record(path,url,source_type="source-port table",commit=D1_COMMIT)],"records":records,"coverage":{"table_rows":len(rows),"included_rows":len(records),"inactive_rows":sum(x["type"]=="inactive_data_entry" for x in records),"complete_for_scope":True,"scope":"All rows in the pinned spell tables; inactive data is explicitly distinguished from usable skills. Class selection restrictions are separate.","notes":["Bard and Barbarian are hidden Hellfire test classes.","Identify is shared through consumables and innate to Bard; Search is a shared Hellfire spell and innate to Monk.","This is a preserved source-port data snapshot, not an unmodified original executable audit."]}})
    return out


def extract_d2(raw):
    source="https://classic.battle.net/diablo2exp/skills/alphaskills.shtml"
    soup=BeautifulSoup((raw/"d2-alpha.html").read_bytes(),"html.parser")
    rows=[]
    for a in soup.find_all("a",href=True):
        text=str(a.next_sibling)
        m=re.match(r"\s*\(([^:]+):\s*([^)]+)\)",text)
        if not m or ".shtml#" not in a["href"]:continue
        rows.append({"id":"diablo2-lod:"+a["href"].split("#")[-1],"name":a.get_text(" ",strip=True),"class":m[1],"tree":m[2],"type":"class_skill","source_url":urljoin(source,a["href"])})
    assert len(rows)==210, len(rows)
    classes={"ama":"Amazon","sor":"Sorceress","nec":"Necromancer","pal":"Paladin","bar":"Barbarian","dru":"Druid","ass":"Assassin","war":"Warlock"}
    skills=json.loads((raw/"d2-skills.json").read_text())
    desc={v["skilldesc"]:v for v in json.loads((raw/"d2-skilldesc.json").read_text()).values() if v.get("skilldesc")}
    strings=json.loads((raw/"d2-strings.json").read_text())
    lookup={x["name"].lower():x for x in rows}
    tree_names={"ama":["Bow and Crossbow Skills","Passive and Magic Skills","Javelin and Spear Skills"],"sor":["Fire Spells","Lightning Spells","Cold Spells"],"nec":["Curses","Poison and Bone Spells","Summoning Spells"],"pal":["Combat Skills","Offensive Auras","Defensive Auras"],"bar":["Combat Skills","Combat Masteries","Warcries"],"dru":["Summoning","Shape Shifting","Elemental"],"ass":["Traps","Shadow Disciplines","Martial Arts"],"war":["Demon","Eldritch","Chaos"]}
    current=[]
    for v in skills.values():
        if v.get("charclass") not in classes:continue
        d=desc[v["skilldesc"]]
        name=strings.get(d.get("str name"),v["skill"])
        if isinstance(name,dict):name=name.get("enUS",v["skill"])
        original_label=name;correction=None
        if v["skill"]=="Plague Poppy":
            name="Poison Creeper";correction="https://classic.battle.net/diablo2exp/skills/druid-summoning.shtml"
        elif v["skill"]=="Blade Mastery":
            name="Blade Mastery";correction="https://news.blizzard.com/en-us/article/23788293/diablo-ii-resurrected-patch-2-4-ladder-now-live"
        canonical_labels={"Levitate":"Levitation Mastery","Hex Bane":"Hex: Bane","Hex Siphon":"Hex: Siphon","Hex Purge":"Hex: Purge","Sigil Lethargy":"Sigil: Lethargy","Sigil Rancor":"Sigil: Rancor","Sigil Death":"Sigil: Death","Miasma Chains":"Miasma Chain","Pole Arm Mastery":"Polearm Mastery"}
        if name in canonical_labels:
            name=canonical_labels[name];correction="https://d2r.org/data/skills"
        ref=lookup.get(name.lower()) or lookup.get(v["skill"].lower())
        tree=tree_names[v["charclass"]][d["SkillPage"]-1]
        req=[v[k] for k in ["reqskill1","reqskill2","reqskill3"] if v.get(k)]
        synergy=[strings.get(d[k],d[k]) for k in d if k.startswith("dsc3textb") and d[k] in strings and d[k]!=d.get("str name")]
        typ="aura" if "Aura" in tree else "passive" if v.get("passivestate") or v.get("passive",0) else "active"
        current.append({"id":"diablo2r:"+str(v["*Id"]),"name":name,"internal_name":v["skill"],"class":classes[v["charclass"]],"tree":tree,"tree_page":d.get("SkillPage"),"tree_row":d.get("SkillRow"),"tree_column":d.get("SkillColumn"),"type":typ,"required_level":v.get("reqlevel",1),"max_base_rank":v.get("maxlvl",20),"prerequisites":req,"synergy_labels":list(dict.fromkeys(synergy)),"source_url":f"https://github.com/blizzhackers/d2data/blob/{D2_COMMIT}/json/skills.json"})
        if correction:current[-1].update({"source_localization_label":original_label,"name_correction_source":correction,"name_correction_note":"Pinned locale table is incorrect or historical; authoritative label is applied explicitly."})
        if typ=="aura" and v.get("passivestate"):current[-1]["also_has_passive_component"]=True
    counts=collections.Counter(x["class"] for x in current)
    assert len(current)==240 and all(n==30 for n in counts.values()),counts
    assert collections.Counter(x["type"] for x in current)=={"active":190,"passive":30,"aura":20}
    current_names={}
    for item in current:
        for name in [item["name"],item["internal_name"],item.get("source_localization_label",item["name"])]:current_names[(item["class"],slug(name).replace("-",""))]=item
    for row in rows:
        normalized=slug(row["name"]).replace("-","")
        normalized={"raiseskeletonmage":"raiseskeletalmage","skeletalmage":"raiseskeletalmage","poisoncreeper":"plaguepoppy"}.get(normalized,normalized)
        match=current_names.get((row["class"],normalized))
        if match:row["type"]=match["type"]
        else:row["type"]="unresolved_active_or_passive"
    assert not [x for x in rows if x["type"]=="unresolved_active_or_passive"],[x["name"] for x in rows if x["type"]=="unresolved_active_or_passive"]
    return [{"game":"Diablo II: Lord of Destruction","version":"official legacy Arreat Summit named-skill index","classes":[classes[x] for x in classes if x!="war"],"sources":[source_record(raw/"d2-alpha.html",source,source_type="official guide")],"records":rows,"coverage":{"expected_named_skills":210,"included_named_skills":len(rows),"complete_for_scope":True,"scope":"Every named skill in official legacy index; historical balance values are not transcribed."}}, {"game":"Diablo II: Resurrected / Reign of the Warlock","version":"pinned d2data snapshot; source repository identifies D2R data, compare current official patch notes independently","classes":list(classes.values()),"sources":[source_record(raw/f"d2-{name}.json",f"https://github.com/blizzhackers/d2data/blob/{D2_COMMIT}/json/{remote}.json",source_type="client-data extraction",commit=D2_COMMIT) for name,remote in [("skills","skills"),("skilldesc","skilldesc"),("strings","localestrings-eng")]],"records":current,"coverage":{"expected_class_skills":240,"included_class_skills":len(current),"per_class":dict(counts),"complete_for_scope":True,"scope":"All 8 x 30 class skill rows, prerequisites and tree grid positions in the pinned data. Skill values remain external."}}]


def extract_d3(raw):
    records=[];sources=[];counts={}
    for cl in ["barbarian","crusader","demon-hunter","monk","necromancer","witch-doctor","wizard"]:
        counts[cl]={}
        for typ in ["active","passive"]:
            path=raw/f"d3-{cl}-{typ}.html";url=f"https://us.diablo3.blizzard.com/en-us/class/{cl}/{typ}/"
            sources.append(source_record(path,url,source_type="official skill table"))
            soup=BeautifulSoup(path.read_bytes(),"html.parser");rows=[]
            for tr in soup.select("table tbody tr"):
                a=tr.select_one(".skill-details h3 a")
                if not a:continue
                cat=tr.select_one(".skill-category");level=tr.select_one(".column-level h3")
                runes=[{"name":x.get_text(" ",strip=True),"rune_id":x["data-d3tooltip"].split("/")[-1],"source_key":x["data-d3tooltip"]} for x in tr.select(".rune-list .tip")]
                rec={"id":f"diablo3:{cl}:{a['href'].split('/')[-1]}","name":a.get_text(" ",strip=True),"class":cl.replace("-"," ").title(),"tree":cat.get_text(" ",strip=True) if cat else "Passive", "type":typ,"required_level":int(level.get_text(strip=True)) if level else None,"variants":runes,"source_url":urljoin(url,a["href"])}
                if typ=="active":assert len(runes)==5,rec
                rows.append(rec)
            counts[cl][typ]=len(rows);counts[cl]["runes"]=counts[cl].get("runes",0)+sum(len(x["variants"]) for x in rows);records+=rows
    totals=collections.Counter(x["type"] for x in records);runes=sum(len(x["variants"]) for x in records)
    assert totals["active"]==163 and totals["passive"]==131 and runes==815,(totals,runes)
    return {"game":"Diablo III / Reaper of Souls / Rise of the Necromancer","version":"official web tables retrieved "+DATE,"classes":[x.replace("-"," ").title() for x in counts],"sources":sources,"records":records,"coverage":{"active_skills":totals["active"],"passives":totals["passive"],"rune_variants":runes,"per_class":counts,"complete_for_scope":True,"scope":"All seven official active/passive tables and all five named runes for each active skill. Item legendary powers and follower skills are separate systems, not silently counted as player class tree nodes."}}


def d4_url(remote):
    return f"https://github.com/DiabloTools/d4data/blob/{D4_COMMIT}/json/{remote}"


def d4_label_path(raw, family, name):
    path=raw/f"d4-{family}_{name}.stl.json"
    if not path.exists():
        path=next((x for x in raw.glob("d4-*.stl.json") if x.name.lower()==path.name.lower()),path)
    return path


def d4_labels(raw, family, name):
    path=d4_label_path(raw,family,name)
    if not path.exists():return {}
    return {x["szLabel"]:x["szText"] for x in json.loads(path.read_text())["arStrings"]}


def extract_d4(raw):
    classes=["Barbarian","Druid","Necromancer","Paladin_NEW","Rogue","Sorcerer","Spiritborn","Warlock"]
    rewards={x["tHeader"]["szName"]:x for x in json.loads((raw/"d4-SkillTreeRewards.gam.json").read_text())["ptData"][0]["tEntries"]}
    records=[];trees=[];counts={};unresolved=[];sources=[]
    for cl in classes:
        display=cl.replace("_NEW","");path=raw/f"d4-{cl}.skl.json";kit=json.loads(path.read_text());tree_url=d4_url(f"base/meta/SkillKit/{cl}.skl.json")
        sources.append(source_record(path,tree_url,source_type="client-data extraction",commit=D4_COMMIT))
        by_power=collections.defaultdict(list)
        nodes=[]
        for index,node in enumerate(kit["arNodes"]):
            key=node["gbidReward"]["name"] if node["gbidReward"] else None
            reward=rewards.get(key)
            rec={"index":index,"node_id":node["dwID"],"reward":key,"required_player_level":node["dwNodeRequiredPlayerLevel"],"exclusive_group":node["nExclusiveGroupId"],"root_type":node["eRootNodeType"],"position":node["vPosition"],"rank_modifiers":node["arSkillTreeSkillRankMods"],"adjacent_indices":[x["nIndexNode"] for x in node["arConnections"]]}
            if reward and reward.get("snoPower"):
                power=reward["snoPower"]["name"];rec["power"]=power;rec["reward_type"]=reward["eType"];rec["max_ranks"]=reward["dwMaxTalentRanks"];by_power[power].append((index,reward))
            nodes.append(rec)
        edges=[{"from_index":x["nIndexNodeA"],"to_index":x["nIndexNodeB"]} for x in kit["arConnections"]]
        assert all(0<=x[k]<len(nodes) for x in edges for k in ["from_index","to_index"])
        trees.append({"class":display,"nodes":nodes,"edges":edges,"source_url":tree_url})
        active={x["snoPower"]["name"]:x for x in kit["arActiveSkillEntries"]}
        powers=sorted(set(active)|set(by_power));class_records=[]
        for name in powers:
            pp=raw/f"d4-{name}.pow.json";power=json.loads(pp.read_text());labels=d4_labels(raw,"Power",name);mods={x["szName"]:x for x in power["arMods"]}
            title=labels.get("name")
            if not title:unresolved.append({"class":display,"power":name,"reason":"missing localized power name"})
            primary=power.get("tPrimaryTag",{}).get("gbidSkillTag")
            tree=primary["name"].removeprefix("Skill_Primary_") if primary else "Class mechanic"
            variants=[];archived=[]
            used=set()
            for index,reward in by_power[name]:
                if reward["eType"]!=1:continue
                mod=mods.get(reward["szPowerMod"]);modid=mod["dwModId"] if mod else None;labelkey=f"Mod{modid}_Name";label=labels.get(labelkey)
                if not label:unresolved.append({"class":display,"power":name,"node":index,"reward":reward["tHeader"]["szName"],"reason":"missing localized current modifier name"})
                used.add(modid)
                variants.append({"name":label or reward["tHeader"]["szName"],"mod_id":modid,"tree_node_index":index,"max_ranks":reward["dwMaxTalentRanks"],"variant_type":mod.get("eModType") if mod else None,"reward":reward["tHeader"]["szName"],"source_label":labelkey})
            for key,value in labels.items():
                m=re.fullmatch(r"Mod(\d+)_Name",key)
                if m and int(m[1]) not in used:archived.append({"name":value,"mod_id":int(m[1]),"availability":"localized label present but not a selectable node in this current class tree"})
            class_records.append({"id":"diablo4:"+str(power["__snoID__"]),"name":title or name,"internal_name":name,"class":display,"tree":tree,"type":"active" if name in active else "tree_power","required_level":active.get(name,{}).get("nLevelReq"),"skill_tags":[x["gbidSkillTag"]["name"] for x in power["arSkillTags"] if x.get("gbidSkillTag")],"tree_node_indices":[i for i,_ in by_power[name]],"variants":variants,"unreferenced_localized_modifiers":archived,"source_url":d4_url(f"base/meta/Power/{name}.pow.json"),"label_source_url":d4_url("enUS_Text/meta/StringList/"+d4_label_path(raw,"Power",name).name.removeprefix("d4-"))})
        records+=class_records
        counts[display]={"active_skillkit_entries":len(active),"tree_unlock_rewards":sum(n.get("reward_type")==0 for n in nodes),"tree_modifiers":sum(len(x["variants"]) for x in class_records),"all_graph_nodes":len(nodes),"graph_edges":len(edges),"structural_nodes":sum(n["reward"] is None for n in nodes)}
    sources.append(source_record(raw/"d4-SkillTreeRewards.gam.json",d4_url("base/meta/GameBalance/SkillTreeRewards.gam.json"),source_type="client-data extraction",commit=D4_COMMIT))
    return {"game":"Diablo IV / Vessel of Hatred / Lord of Hatred","version":"3.2.4.74043 (live snapshot; 3.3.0 announced for December 1, 2026)","classes":[x.replace("_NEW","") for x in classes],"sources":sources,"records":records,"skill_tree_graphs":trees,"coverage":{"active_skillkit_entries":sum(x["active_skillkit_entries"] for x in counts.values()),"tree_unlock_rewards":sum(x["tree_unlock_rewards"] for x in counts.values()),"tree_modifiers":sum(x["tree_modifiers"] for x in counts.values()),"all_graph_nodes":sum(x["all_graph_nodes"] for x in counts.values()),"standalone_passive_nodes_in_current_skill_trees":0,"per_class":counts,"unresolved":unresolved,"complete_for_scope":not unresolved,"scope":"All active entries, every skill-tree graph node/edge, and every selectable modifier name from all eight current class SkillKits. Extra localized modifier labels are separated from current nodes.","notes":["Blizzard removed standalone Passive and Key Passive nodes from the class skill trees in 3.0; this is intentional, not omitted extraction.","Warlock's SkillKit has 28 active entries but 24 tree unlocks: the extra four entries belong to its Greater Demon class mechanic.","Historical pre-3.0 passive trees are not falsely presented as current. Class mechanics and Paragon are additional catalogue sections."]}}


def extract_immortal(raw):
    classes={x["id"]:x["name"] for x in json.loads((raw/"di-tv-classes-0.json").read_text())}
    pages=json.loads((raw/"di-tv-pagination.json").read_text());allrows=[]
    for page in pages:
        if page["table"]=="skillbook":allrows+=json.loads((raw/page["cache"]).read_text())
    assert len({x["id"] for x in allrows})==len(allrows)
    wh=BeautifulSoup((raw/"di-wowhead-skills.html").read_bytes(),"html.parser")
    whdata=json.loads(wh.find("script",id="data.page.listPage.listviews").string)[0]["data"]
    whlookup={x["id"]:x for x in whdata if x.get("playerClass")}
    source="https://diablo.tv/builder";whurl="https://www.wowhead.com/diablo-immortal/skills"
    sources=[source_record(raw/page["cache"],source,source_type="community maintained public builder database",table=page["table"],offset=page["offset"],returned=page["returned"],content_range=page["content_range"]) for page in pages]
    sources.append(source_record(raw/"di-wowhead-skills.html",whurl,source_type="datamined database; capped historical index"))
    records=[];variants=[];counts={}
    extras_primary={109000,109022,110001,110002}
    for row in allrows:
        cl=classes[row["career"]]
        if row["item_id"]!=-1:
            variants.append({"id":"diablo-immortal:essence:"+str(row["id"]),"name":row["item_name"],"class":cl,"base_skill_name":row["name"].strip(),"base_skill_id":row["base_skill_id"],"item_id":row["item_id"],"slot_id":row["slot"],"result_skill_id":row["result_skill_id"],"type":"legendary_essence_variant","source_url":source})
            continue
        whrow=whlookup.get(row["base_skill_id"],{})
        typ="primary" if whrow.get("skillType")==1 or row["base_skill_id"] in extras_primary else "passive" if row["base_skill_id"] in [108000,110000] else "transformed_skill" if "(Were" in row["name"] else "active"
        records.append({"id":"diablo-immortal:"+str(row["base_skill_id"]),"name":row["name"].strip(),"class":cl,"tree":"class skill loadout","type":typ,"required_level":whrow.get("level"),"source_url":source,"source_record_id":row["id"],"crosscheck_url":whurl if whrow else None})
    for row in whdata:
        if row.get("playerClass") and row.get("level") is None and row.get("name"):
            records.append({"id":"diablo-immortal:"+str(row["id"]),"name":row["name"].strip(),"class":classes[row["playerClass"]],"tree":"primary attack ultimate","type":"ultimate","source_url":whurl})
    article_urls={"druid":"https://news.blizzard.com/en-us/article/24216435/introducing-diablo-immortals-newest-class-druid","warlock":"https://news.blizzard.com/en-us/article/24277443/introducing-diablo-immortals-newest-class-warlock","tempest":"https://news.blizzard.com/en-us/article/24096019/command-wind-and-sea-as-the-tempest","blood-knight":"https://news.blizzard.com/en-us/article/23967323/introducing-the-newest-diablo-immortal-class-blood-knight"}
    article_text={}
    for cl,url in article_urls.items():
        path=raw/f"di-{cl}.html";article_text[cl]=BeautifulSoup(path.read_bytes(),"html.parser").get_text(" ",strip=True);sources.append(source_record(path,url,source_type="official class introduction"))
    extras=[
      ("druid","Fang and Claw","ultimate","Ferocious Strike"),("druid","Nature’s Wrath","ultimate","Landslide"),
      ("druid","Swipe","transformed_primary","Werewolf"),("druid","Maul","transformed_primary","Werebear"),
      ("druid","Revert","transformed_utility","Werewolf"),("druid","Revert","transformed_utility","Werebear"),
      ("druid","Primal Power","class_resource","class mechanic"),("druid","Ursine Endurance","transformed_passive","Werebear"),
      ("warlock","Lunatic Legion","ultimate","Lunatic Rush"),("warlock","Rain of Chaos","ultimate","Hellfire Bolt"),
      ("warlock","Pummeling Eruption","pet_skill","Soulgorger"),("warlock","Voracious Sacrifice","pet_passive","Soulgorger"),("warlock","Ferocious Leap","pet_command","Soulgorger"),("warlock","Devour","pet_mechanic","Soulgorger"),
      ("blood-knight","Blood Rush","transformed_skill","Abomination"),("blood-knight","Shattering Fists","transformed_skill","Abomination")]
    for cl,name,typ,parent in extras:
        assert name in article_text[cl],(cl,name)
        records.append({"id":"diablo-immortal:official:"+cl+":"+slug(name)+":"+slug(parent),"name":name,"class":cl.replace("-"," ").title(),"tree":parent,"type":typ,"source_url":article_urls[cl]})
    for name in ["Bloodlord","Demonic Soldier","Rasplet Hulk"]:
        assert name in article_text["warlock"]
        variants.append({"id":"diablo-immortal:portal:"+slug(name),"name":name,"class":"Warlock","base_skill_name":"Demonic Portal","base_skill_id":110003,"type":"summon_variant","source_url":article_urls["warlock"]})
    for name in ["Spiteful Sacrifice","Devouring Darkness","Siphon Life"]:
        variants.append({"id":"diablo-immortal:sacrifice:"+slug(name),"name":"Sacrificial Power","class":"Warlock","base_skill_name":name,"type":"class_mechanic_variant","source_url":article_urls["warlock"]})
    for cl in classes.values():
        values=[x for x in records if x["class"]==cl];counts[cl]=dict(collections.Counter(x["type"] for x in values));counts[cl]["essence_variants"]=sum(x["class"]==cl and x["type"]=="legendary_essence_variant" for x in variants)
        assert counts[cl].get("primary")==2 and counts[cl].get("active")==13,(cl,counts[cl])
        assert counts[cl].get("ultimate")==2,(cl,counts[cl])
    paragons=[]
    for row in json.loads((raw/"di-tv-paragons_with_tree-0.json").read_text()):
        paragons.append({"id":"diablo-immortal:paragon:"+str(row["id"]),"name":row["name"],"tree":row["tree_name"],"tree_id":row["tree_id"],"class":"shared","type":"paragon_skill","source_url":source})
    assert len(allrows)==1547 and len(variants)==1395 and len(paragons)==136,(len(allrows),len(variants),len(paragons))
    return {"game":"Diablo Immortal","version":"public builder snapshot retrieved "+DATE+"; latest skillbook row timestamp "+max(x["created_at"] for x in allrows),"classes":list(classes.values()),"sources":sources,"records":records,"variants":variants,"paragon_records":paragons,"coverage":{"source_skillbook_rows":len(allrows),"source_base_rows":sum(x["item_id"]==-1 for x in allrows),"primary_attacks":20,"normal_active_skills":130,"ultimates":20,"class_and_form_records":len(records)-170,"essence_variants":1389,"additional_class_mechanic_variants":6,"paragon_records":len(paragons),"paragon_groups":dict(collections.Counter(x["tree"] for x in paragons)),"per_class":counts,"pagination":pages,"complete_for_scope":True,"live_build_equivalence_verified":False,"scope":"Every row from all pages of the public skillbook and Paragon data, all ten classes' two primaries and thirteen equipped skills, twenty ultimates, and named transformation/pet subskills from official introductions.","unresolved":["The community database does not expose a game-build identifier; enumeration is complete for this retrieved snapshot, not an executable-level proof of every currently obtainable essence.","Wowhead's unfiltered page stops at 1,000 entries and lacks Druid/Warlock. It is used only to cross-check the first eight classes and add their named ultimates, not as an exhaustive variant source.","Some Tempest unlock levels differ between its older official introduction and the datamined index; required_level remains source-version information, not a current progression guarantee."],"notes":["Base skillbook has 158 rows: 150 primary/equipped skills, Mist-Touched, Soulgorger, and six transformed Druid skills.","Source descriptions, artwork and numerical balance text are not redistributed."]}}


def add_d4_mechanics(raw,game):
    manifest=json.loads((raw/"d4-mechanics-fetch-manifest.json").read_text());assert not manifest["failures"]
    records=[];unresolved=[]
    for cl,refs in manifest["references"].items():
        own={r["power"]:d4_labels(raw,"Power",r["power"]) for r in refs}
        for ref in refs:
            power=ref["power"];labels=own[power];name=labels.get("name");derived=False
            if not name and cl=="Barbarian":
                stem=power.removesuffix("_Rank10").removesuffix("_Bonus")
                name=own.get(stem,{}).get("name",stem)+" — rank 10 bonus";derived=True
            elif not name and cl=="Sorcerer":
                origin=ref["path"].split("/",1)[1];name=d4_labels(raw,"Power",origin).get("name",origin)+" Enchantment";derived=True
            elif not name and cl=="Paladin":
                idx=int(ref["path"].split("/")[2])+1
                ui=json.loads((raw/"d4-PaladinOath.stl.json").read_text());name=next(x["szText"] for x in ui["arStrings"] if x["szLabel"]==f"Oath{idx}Name")
            elif not name and cl=="Spiritborn":
                animal=power.split("_")[1].title();tier="Primary" if "_sun_" in power else "Secondary";name=f"{animal} {tier} Spirit";derived=True
            if not name:unresolved.append({"class":cl,"power":power});name=power
            rec={"id":"diablo4:mechanic:"+str(ref["sno"]),"name":name,"internal_name":power,"class":cl,"type":"class_mechanic_power","relation_path":ref["path"],"display_label_is_derived":derived,"source_url":d4_url(f"base/meta/PlayerClass/{cl}.pcl.json"),"power_source_url":d4_url(f"base/meta/Power/{power}.pow.json")}
            records.append(rec)
        path=raw/f"d4-{cl}.pcl.json";game["sources"].append(source_record(path,d4_url(f"base/meta/PlayerClass/{cl}.pcl.json"),source_type="client-data extraction",commit=D4_COMMIT))
    game["class_mechanic_records"]=records
    game["coverage"]["class_mechanic_power_references"]=len(records)
    game["coverage"]["class_mechanics_per_class"]={cl:len(refs) for cl,refs in manifest["references"].items()}
    game["coverage"]["unresolved"]+=unresolved


def add_d4_paragon(raw,game):
    manifest=json.loads((raw/"d4-paragon-fetch-manifest.json").read_text());assert not manifest["failures"]
    boards=[];node_usage=collections.defaultdict(set);unresolved=[]
    for cl in game["classes"]:
        pcl=json.loads((raw/f"d4-{cl}.pcl.json").read_text())
        for ref in pcl["arAvailableParagonBoards"]:
            key=ref["name"];board=json.loads((raw/f"d4-{key}.pbd.json").read_text());labels=d4_labels(raw,"ParagonBoard",key);name=labels.get("Name") or labels.get("name")
            if not name:unresolved.append({"kind":"paragon_board","key":key})
            entries=[]
            for i,value in enumerate(board["arEntries"]):
                if value:
                    entries.append({"cell_index":i,"node":value["name"]});node_usage[value["name"]].add(cl)
            boards.append({"id":"diablo4:board:"+str(board["__snoID__"]),"name":name or key,"internal_name":key,"class":cl,"grid_width":board["nWidth"],"grid_cell_count":len(board["arEntries"]),"occupied_cells":entries,"type":"paragon_board","source_url":d4_url(f"base/meta/ParagonBoard/{key}.pbd.json")})
    nodes=[];glyphs=[]
    for family,ext,out in [("ParagonNode","pgn",nodes),("ParagonGlyph","gph",glyphs)]:
        tree=json.loads((raw/f"d4-{family}-tree.json").read_text());assert not tree["truncated"]
        for item in tree["tree"]:
            if item["type"]!="blob" or item["path"].startswith("Axe Bad Data"):continue
            key=item["path"].split(".")[0];value=json.loads((raw/f"d4-{item['path']}").read_text());labels=d4_labels(raw,family,key);name=labels.get("Name") or labels.get("name")
            rec={"id":"diablo4:"+ext+":"+str(value["__snoID__"]),"name":name,"internal_name":key,"type":"paragon_node" if family=="ParagonNode" else "paragon_glyph","source_url":d4_url(f"base/meta/{family}/{item['path']}")}
            if family=="ParagonNode":
                stat=not name and ("_Magic_" in key or "_Normal_" in key)
                if not name and not stat:unresolved.append({"kind":"paragon_node","key":key})
                rec.update({"name_kind":"localized" if name else "unnamed attribute node" if stat else "unresolved","rarity_override":value["eRarityOverride"],"passive_power":value["snoPassivePower"]["name"] if value["snoPassivePower"] else None,"is_socket":bool(value["bHasSocket"]),"is_gate":bool(value["bIsGate"]),"referenced_by_classes":sorted(node_usage[key]),"availability":"referenced by an available player board" if key in node_usage and node_usage[key] else "data entry not referenced by a current available player board"})
            else:
                if not name:unresolved.append({"kind":"paragon_glyph","key":key})
                rec.update({"usable_by_class_flags":value["fUsableByClass"],"affix_references":[x["name"] for x in value["arAffixes"] if x],"rarity":value["eRarity"]})
            out.append(rec)
    assert len(boards)==80 and len(nodes)==566 and len(glyphs)==161,(len(boards),len(nodes),len(glyphs))
    known={x["internal_name"] for x in nodes};assert all(x["node"] in known for b in boards for x in b["occupied_cells"])
    game["paragon_boards"]=boards;game["paragon_nodes"]=nodes;game["paragon_glyphs"]=glyphs
    game["coverage"]["paragon"]={"available_boards":len(boards),"board_grid_occupied_cells":sum(len(x["occupied_cells"]) for x in boards),"node_definitions":len(nodes),"localized_named_nodes":sum(bool(x["name"]) for x in nodes),"unnamed_stat_nodes":sum(not x["name"] for x in nodes),"node_definitions_referenced_by_available_boards":sum(bool(x["referenced_by_classes"]) for x in nodes),"glyph_definitions":len(glyphs),"unresolved":unresolved,"complete_for_scope":not unresolved,"scope":"All 80 available class boards and every occupied grid cell; all 566 non-placeholder ParagonNode definitions and 161 non-placeholder glyph definitions. Unreferenced definitions are retained and labelled. Magic/normal stat nodes without custom localized names are explicitly unnamed; class-eligibility flags are preserved without inventing an enum mapping."}
    game["coverage"]["unresolved"]+=unresolved


def build(raw):
    games=extract_legacy(raw)+extract_d2(raw)+[extract_d3(raw),extract_d4(raw),extract_immortal(raw)]
    add_d4_mechanics(raw,games[5]);add_d4_paragon(raw,games[5])
    d4paths=set()
    for name in ["d4-fetch-manifest.json","d4-mechanics-fetch-manifest.json","d4-paragon-fetch-manifest.json"]:d4paths.update(json.loads((raw/name).read_text())["paths"])
    d4paths.update(f"base/meta/SkillKit/{cl}.skl.json" for cl in ["Barbarian","Druid","Necromancer","Paladin_NEW","Rogue","Sorcerer","Spiritborn","Warlock"])
    d4paths.update(["base/meta/GameBalance/SkillTreeRewards.gam.json","enUS_Text/meta/StringList/PaladinOath.stl.json"])
    manifest=[source_record(raw/("d4-"+path.rsplit("/",1)[1]),d4_url(path),bytes=(raw/("d4-"+path.rsplit("/",1)[1])).stat().st_size) for path in sorted(d4paths)]
    licenses=[source_record(raw/"d1-LICENSE.md",f"https://github.com/diasurgical/devilutionX/blob/{D1_COMMIT}/LICENSE.md",license_name="Sustainable Use License 1.0",scope="Source-port repository license; do not treat it as a permissively licensed commercial engine foundation."),source_record(raw/"d2-LICENSE",f"https://github.com/blizzhackers/d2data/blob/{D2_COMMIT}/LICENSE",license_name="MIT",scope="Repository notice, copyright (c) 2020 blizzhackers; underlying game content rights are not established by this notice."),source_record(raw/"d4-LICENSE",f"https://github.com/DiabloTools/d4data/blob/{D4_COMMIT}/LICENSE",license_name="MIT",scope="Repository notice, copyright (c) 2023 blizzhackers; underlying game content rights are not established by this notice."),{"url":"https://news.blizzard.com/","license_name":"No open-content redistribution license established by this research","scope":"Official pages are used as factual references; their prose/art are not imported."},{"url":"https://diablo.tv/builder","license_name":"Public database; redistribution license not determined","scope":"Factual labels/IDs/relationships only; no descriptions or artwork."}]
    return {"schema":1,"research_date":DATE,"purpose":"Source-backed factual class/ability/tree index for original Dreadmarch design synthesis, separate from implemented game content.","unique_class_identities":CLASSES,"aliases":{"Sorceress":"Sorcerer"},"source_license_ledger":licenses,"d4_input_manifest":manifest,"games":games,"coverage_state":"source_enumeration_complete_with_explicit_version_limits","remaining":["An official or lawful current-build Immortal data export is needed to prove live-build equivalence of the community essence/Paragon snapshot.","Historical pre-3.0 Diablo IV passive trees, all prior balance revisions, global item affixes and follower/monster-only powers are outside this current class-ability catalogue; their omission must not be described as all historical game content."]}


if __name__=="__main__":
    p=argparse.ArgumentParser();p.add_argument("--raw",type=Path,required=True);p.add_argument("--output",type=Path,default=Path("data/research/diablo-catalog.json"));a=p.parse_args()
    data=build(a.raw);a.output.parent.mkdir(parents=True,exist_ok=True);a.output.write_text(json.dumps(data,indent=2,ensure_ascii=False)+"\n")
    print(json.dumps({"output":str(a.output),"games":[{"game":x["game"],"records":len(x["records"]),"coverage":x["coverage"]} for x in data["games"]]},indent=2))
