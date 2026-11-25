#!/usr/bin/env python3
"""Cache powers explicitly referenced by each playable class's mechanic data."""
import argparse
import concurrent.futures
import json
from pathlib import Path
from diablo_fetch_d4 import fetch,COMMIT,CLASSES


def walk(value,path=""):
    if isinstance(value,dict):
        if value.get("groupName")=="Power":yield path,value
        else:
            for key,v in value.items():yield from walk(v,path+"/"+key)
    elif isinstance(value,list):
        for i,v in enumerate(value):yield from walk(v,path+"/"+str(i))


def main(raw):
    english={x["path"].lower():x["path"] for x in json.loads((raw/"d4-en-meta-tree.json").read_text())["tree"]}
    refs={};paths=set();missing=[]
    for cl in CLASSES:
        display=cl.replace("_NEW","");p=json.loads((raw/f"d4-{display}.pcl.json").read_text());key="t"+{"Barbarian":"Barb","Necromancer":"Necro"}.get(display,display)+"Data"
        refs[display]=[{"path":path,"power":value["name"],"sno":value["__raw__"]} for path,value in walk(p[key],key)]
        if display=="Sorcerer":
            kit=json.loads((raw/"d4-Sorcerer.skl.json").read_text())
            for row in kit["arActiveSkillEntries"]:
                power=json.loads((raw/f"d4-{row['snoPower']['name']}.pow.json").read_text());enchant=power.get("snoEnchantmentPower")
                if enchant:refs[display].append({"path":"enchantment/"+power["szInternalName"],"power":enchant["name"],"sno":enchant["__raw__"]})
        for ref in refs[display]:
            name=ref["power"];paths.add(f"base/meta/Power/{name}.pow.json")
            key=f"StringList/Power_{name}.stl.json".lower()
            if key in english:paths.add("enUS_Text/meta/"+english[key])
            else:missing.append({"class":display,"power":name})
    failures=[]
    with concurrent.futures.ThreadPoolExecutor(max_workers=8) as pool:
        futures={pool.submit(fetch,raw,path):path for path in sorted(paths)}
        for i,future in enumerate(concurrent.futures.as_completed(futures),1):
            try:future.result()
            except Exception as e:failures.append({"path":futures[future],"error":str(e)})
            if i%50==0:print(f"Class mechanics {i}/{len(paths)}",flush=True)
    manifest={"commit":COMMIT,"references":refs,"paths":sorted(paths),"missing_label_files":missing,"failures":failures}
    (raw/"d4-mechanics-fetch-manifest.json").write_text(json.dumps(manifest,indent=2)+"\n")
    print(json.dumps({"references":{k:len(v) for k,v in refs.items()},"missing_label_files":missing,"failures":failures},indent=2),flush=True)


if __name__=="__main__":
    parser=argparse.ArgumentParser();parser.add_argument("--raw",type=Path,required=True);args=parser.parse_args();main(args.raw)
