#!/usr/bin/env python3
"""Package complete source, playable browser build, optional native binary, and evidence."""
from pathlib import Path
import argparse
import hashlib
import json
import subprocess
import zipfile

root = Path(__file__).resolve().parents[1]
parser = argparse.ArgumentParser()
parser.add_argument('--native', type=Path)
parser.add_argument('--allow-dirty', action='store_true')
parser.add_argument('--output', type=Path, default=root/'dist/Dreadmarch-checkpoint.zip')
args = parser.parse_args()
def git(*arguments):
    return subprocess.check_output(['git',*arguments],cwd=root).decode().strip()
dirty = git('status','--porcelain')
if dirty and not args.allow_dirty:
    raise SystemExit('Uncommitted source changes: checkpoint first or explicitly pass --allow-dirty.')
source_names = git('ls-files','--cached','--others','--exclude-standard','-z').split('\0')
files = [(root/name,'source/'+name) for name in sorted(set(source_names)) if name]
browser = root/'dist/Dreadmarch-playtest.html'
if not browser.is_file():
    raise SystemExit('Build the browser playtest with scripts/build_web.py first.')
files.append((browser,'play/Dreadmarch-playtest.html'))
for name in ['web-build.json','core-tests.txt','native-smoke.png','native-smoke-receipt.json','verification.json']:
    path = root/'dist'/name
    if path.is_file(): files.append((path,'verification/'+name))
if args.native:
    native = args.native.resolve()
    if not native.is_file(): raise SystemExit('Native binary is missing.')
    files.append((native,'play/'+native.name))
receipt = {'schema':1,'repository':'https://github.com/Herbertofury/Dreadmarch',
           'source_commit':git('rev-parse','HEAD'),'dirty_source':bool(dirty),
           'pending_changes':dirty.splitlines(),'files':[]}
args.output.parent.mkdir(parents=True,exist_ok=True)
with zipfile.ZipFile(args.output,'w',zipfile.ZIP_DEFLATED,compresslevel=9) as archive:
    for path,name in files:
        data=path.read_bytes()
        archive.writestr(name,data)
        receipt['files'].append({'path':name,'bytes':len(data),'sha256':hashlib.sha256(data).hexdigest()})
    archive.writestr('CHECKPOINT.json',json.dumps(receipt,indent=2)+'\n')
with zipfile.ZipFile(args.output) as archive:
    if archive.testzip() is not None: raise SystemExit('Archive integrity check failed.')
data=args.output.read_bytes()
result={'file':str(args.output),'bytes':len(data),'sha256':hashlib.sha256(data).hexdigest(),
        'md5':hashlib.md5(data).hexdigest(),'entries':len(files)+1,
        'source_commit':receipt['source_commit'],'dirty_source':receipt['dirty_source']}
args.output.with_suffix('.manifest.json').write_text(json.dumps(result,indent=2)+'\n',encoding='utf-8')
print(json.dumps(result,indent=2))
