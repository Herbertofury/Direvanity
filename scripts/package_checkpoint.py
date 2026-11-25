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
parser.add_argument('--artifact', type=Path, action='append', default=[],
                    help='Explicit release artifact to include under release/; repeat for each ZIP or checksum file.')
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
for name in ['web-build.json','core-tests.txt','native-smoke.png','native-smoke-receipt.json',
             'verification.json','spatial-benchmark.json','battle-smoke.json','extracted-native-receipt.json',
             'browser-live-receipt.json']:
    path = root/'dist'/name
    if path.is_file(): files.append((path,'verification/'+name))
if args.native:
    native = args.native.resolve()
    if not native.is_file(): raise SystemExit('Native binary is missing.')
    files.append((native,'play/'+native.name))
for artifact in args.artifact:
    artifact = artifact.resolve()
    if not artifact.is_file(): raise SystemExit(f'Release artifact is missing: {artifact}')
    files.append((artifact,'release/'+artifact.name))
archive_names = set()
for _, name in files:
    if name in archive_names:
        raise SystemExit(f'Archive name collision: {name}')
    archive_names.add(name)
receipt = {'schema':1,'repository':'https://github.com/Herbertofury/Dreadmarch',
           'source_commit':git('rev-parse','HEAD'),'dirty_source':bool(dirty),
           'pending_changes':dirty.splitlines(),'files':[]}
guide = ['DREADMARCH - PLAYABLE ENGINE FOUNDATION', '',
         'Play online: https://herbertofury.github.io/Dreadmarch/',
         'Play offline: open play/Dreadmarch-playtest.html in a modern desktop browser.',
         'The browser and native clients use the same Rust game simulation.', '',
         'NATIVE PACKAGES']
for artifact in args.artifact:
    if artifact.name == 'Dreadmarch-windows-x64.zip':
        guide.append('Windows: unpack release/Dreadmarch-windows-x64.zip, then run dreadmarch.exe.')
        guide.append('If VCRUNTIME140.dll is missing, install Microsoft Visual C++ x64 runtime: https://aka.ms/vc14/vc_redist.x64.exe')
    if artifact.name == 'Dreadmarch-linux-x64.zip':
        guide.append('Linux: unpack release/Dreadmarch-linux-x64.zip, then run ./dreadmarch.')
        guide.append('Linux requires glibc 2.35+, X11 runtime libraries and a working graphics driver.')
if args.native:
    guide.append('Included native executable: play/'+args.native.name)
guide += ['', 'CONTROLS',
          'Both: left click/drag to select, right click to move; Shift adds to selection.',
          'Native: Space targets Soulfire, 1/2 recruit, P pauses, Tab selects the army, F1 opens help.',
          'Browser: Q targets Soulfire, V/B recruit, Space pauses, A selects the army; open Field manual.',
          '', 'SOURCE AND EVIDENCE',
          'Complete source is in source/. Begin with source/README.md and source/docs/STATUS.md.',
          'The vision and roadmap distinguish the production ambition from this geometric prototype.',
          'verification/ contains the observed test, rendering and package evidence.',
          'CHECKPOINT.json records the source revision and SHA-256 of every included payload.',
          'Release ZIPs include the project licenses and available dependency notices.', '']
guide_data = '\n'.join(guide).encode('utf-8')
args.output.parent.mkdir(parents=True,exist_ok=True)
with zipfile.ZipFile(args.output,'w',zipfile.ZIP_DEFLATED,compresslevel=9) as archive:
    for path,name in files:
        data=path.read_bytes()
        # Passing only a name to writestr discards executable permissions. Keep
        # filesystem metadata while hashing the exact bytes written to the ZIP.
        info = zipfile.ZipInfo.from_file(path,arcname=name)
        archive.writestr(info,data,compress_type=zipfile.ZIP_DEFLATED,compresslevel=9)
        receipt['files'].append({'path':name,'bytes':len(data),
                                 'sha256':hashlib.sha256(data).hexdigest(),
                                 'unix_mode':format((info.external_attr >> 16) & 0o7777,'04o')})
    guide_info = zipfile.ZipInfo('START-HERE.txt')
    guide_info.create_system = 3
    guide_info.external_attr = 0o100644 << 16
    archive.writestr(guide_info,guide_data,compress_type=zipfile.ZIP_DEFLATED,compresslevel=9)
    receipt['files'].append({'path':'START-HERE.txt','bytes':len(guide_data),
                            'sha256':hashlib.sha256(guide_data).hexdigest(),'unix_mode':'0644'})
    archive.writestr('CHECKPOINT.json',json.dumps(receipt,indent=2)+'\n')
with zipfile.ZipFile(args.output) as archive:
    if archive.testzip() is not None: raise SystemExit('Archive integrity check failed.')
data=args.output.read_bytes()
result={'file':str(args.output),'bytes':len(data),'sha256':hashlib.sha256(data).hexdigest(),
        'md5':hashlib.md5(data).hexdigest(),'entries':len(receipt['files'])+1,
        'source_commit':receipt['source_commit'],'dirty_source':receipt['dirty_source']}
args.output.with_suffix('.manifest.json').write_text(json.dumps(result,indent=2)+'\n',encoding='utf-8')
print(json.dumps(result,indent=2))
