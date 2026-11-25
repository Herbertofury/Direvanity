#!/usr/bin/env python3
"""Read the public data backing diablo.tv's builder, exhausting its pagination.

The site publishes a client API key in its JS. No user credentials, account,
sign-in, or write operation are used. The key is never logged or committed.
Only factual names/IDs/relationships from cached results enter our catalogue.
"""
import argparse
import json
import re
import urllib.request
from pathlib import Path


def main(raw):
    script=(raw/"di-tv-db-7pQIP-xW.js").read_text()
    base,key=re.search(r's\("(https:[^"]+)","(eyJ[^"]+)"',script).groups()
    headers={"apikey":key,"Authorization":"Bearer "+key,"Prefer":"count=exact"}
    manifest=[]
    for table,fields,order in [("classes","id,name","id.asc"),("skillbook","*","is_new.desc.nullslast,item_id.desc"),("paragons_with_tree","id,tree_id,tree_name,name","id.asc")]:
        offset=0;total=0
        while True:
            path=raw/f"di-tv-{table}-{offset}.json"
            if not path.exists():
                url=f"{base}/rest/v1/{table}?select={fields}&order={order}&offset={offset}&limit=1000"
                with urllib.request.urlopen(urllib.request.Request(url,headers=headers),timeout=60) as response:
                    data=response.read();content_range=response.headers.get("Content-Range")
                path.write_bytes(data)
            else: content_range=None
            data=json.loads(path.read_text());count=len(data);total+=count
            manifest.append({"table":table,"offset":offset,"returned":count,"content_range":content_range,"cache":path.name})
            print(f"{table}: offset {offset}, {count} records",flush=True)
            if count<1000:break
            offset+=1000
        print(f"{table}: total {total}",flush=True)
    (raw/"di-tv-pagination.json").write_text(json.dumps(manifest,indent=2)+"\n")


if __name__=="__main__":
    parser=argparse.ArgumentParser();parser.add_argument("--raw",type=Path,required=True);args=parser.parse_args();main(args.raw)
