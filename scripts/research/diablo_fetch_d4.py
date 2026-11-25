#!/usr/bin/env python3
"""Cache precisely referenced current D4 skill data; never import full text into the catalogue."""
import argparse
import concurrent.futures
import json
import urllib.request
from pathlib import Path

COMMIT = "75e619f0d90a0951945747f45feef59c076a5db0"
CLASSES = ["Barbarian", "Druid", "Necromancer", "Paladin_NEW", "Rogue", "Sorcerer", "Spiritborn", "Warlock"]
BASE = f"https://raw.githubusercontent.com/DiabloTools/d4data/{COMMIT}/json/"


def fetch(raw, remote):
    name = "d4-" + remote.rsplit("/", 1)[-1]
    p = raw / name
    if not p.exists():
        data = urllib.request.urlopen(BASE + remote, timeout=60).read()
        json.loads(data)
        tmp = p.with_suffix(p.suffix + ".tmp")
        tmp.write_bytes(data)
        tmp.replace(p)
    return name


def main(raw):
    raw.mkdir(parents=True, exist_ok=True)
    for cl in CLASSES:
        fetch(raw, f"base/meta/SkillKit/{cl}.skl.json")
    fetch(raw, "base/meta/GameBalance/SkillTreeRewards.gam.json")
    rewards = {x["tHeader"]["szName"]:x for x in json.loads((raw / "d4-SkillTreeRewards.gam.json").read_text())["ptData"][0]["tEntries"]}
    powers = set()
    for cl in CLASSES:
        kit = json.loads((raw / f"d4-{cl}.skl.json").read_text())
        powers.update(x["snoPower"]["name"] for x in kit["arActiveSkillEntries"])
        for node in kit["arNodes"]:
            if node["gbidReward"]:
                reward = rewards[node["gbidReward"]["name"]]
                if reward.get("snoPower"):
                    powers.add(reward["snoPower"]["name"])
    paths = [f"base/meta/Power/{p}.pow.json" for p in sorted(powers)]
    english={x["path"].lower():x["path"] for x in json.loads((raw/"d4-en-meta-tree.json").read_text())["tree"]}
    paths += ["enUS_Text/meta/"+english.get(f"StringList/Power_{p}.stl.json".lower(),f"StringList/Power_{p}.stl.json") for p in sorted(powers)]
    paths += [f"base/meta/PlayerClass/{p.replace('_NEW','')}.pcl.json" for p in CLASSES]
    failures = []
    with concurrent.futures.ThreadPoolExecutor(max_workers=8) as pool:
        futures = {pool.submit(fetch,raw,p):p for p in paths}
        for i, future in enumerate(concurrent.futures.as_completed(futures), 1):
            try: future.result()
            except Exception as e: failures.append({"path":futures[future], "error":str(e)})
            if i % 50 == 0: print(f"{i}/{len(paths)} sources inspected", flush=True)
    manifest={"commit":COMMIT,"power_count":len(powers),"paths":paths,"failures":failures}
    (raw/"d4-fetch-manifest.json").write_text(json.dumps(manifest,indent=2)+"\n")
    print(json.dumps({k:v for k,v in manifest.items() if k!="paths"},indent=2),flush=True)


if __name__=="__main__":
    p=argparse.ArgumentParser();p.add_argument("--raw",type=Path,required=True);args=p.parse_args();main(args.raw)
