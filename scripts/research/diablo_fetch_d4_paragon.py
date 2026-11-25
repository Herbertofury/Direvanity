#!/usr/bin/env python3
"""Cache all Paragon family entries and available English labels at a pinned commit."""
import argparse
import concurrent.futures
import json
from pathlib import Path
from diablo_fetch_d4 import fetch,COMMIT


def main(raw):
    english={x["path"] for x in json.loads((raw/"d4-en-meta-tree.json").read_text())["tree"]}
    paths=[];missing_labels=[]
    for family in ["ParagonBoard","ParagonNode","ParagonGlyph"]:
        tree=json.loads((raw/f"d4-{family}-tree.json").read_text());assert not tree["truncated"]
        for item in tree["tree"]:
            if item["type"]!="blob" or item["path"].startswith("Axe Bad Data"):continue
            stem=item["path"].split(".")[0]
            paths.append(f"base/meta/{family}/{item['path']}")
            en=f"StringList/{family}_{stem}.stl.json"
            if en in english:paths.append("enUS_Text/meta/"+en)
            else:missing_labels.append(en)
    failures=[]
    with concurrent.futures.ThreadPoolExecutor(max_workers=16) as pool:
        futures={pool.submit(fetch,raw,p):p for p in paths}
        for i,future in enumerate(concurrent.futures.as_completed(futures),1):
            try:future.result()
            except Exception as e:failures.append({"path":futures[future],"error":str(e)})
            if i%100==0:print(f"Paragon {i}/{len(paths)}",flush=True)
    manifest={"commit":COMMIT,"paths":paths,"missing_label_paths":missing_labels,"failures":failures}
    (raw/"d4-paragon-fetch-manifest.json").write_text(json.dumps(manifest,indent=2)+"\n")
    print(json.dumps({"files":len(paths),"missing_labels":missing_labels,"failures":failures},indent=2),flush=True)


if __name__=="__main__":
    parser=argparse.ArgumentParser();parser.add_argument("--raw",type=Path,required=True);args=parser.parse_args();main(args.raw)
