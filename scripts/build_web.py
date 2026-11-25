#!/usr/bin/env python3
"""Build the real Rust simulation into a completely offline playable HTML file."""
from pathlib import Path
import argparse
import base64
import hashlib
import json
import os
import shutil
import subprocess

root = Path(__file__).resolve().parents[1]
parser = argparse.ArgumentParser()
parser.add_argument('--skip-build', action='store_true')
parser.add_argument('--target-dir', type=Path, default=root / 'target')
args = parser.parse_args()
target = args.target_dir.resolve()
if not args.skip_build:
    env = dict(os.environ, CARGO_TARGET_DIR=str(target))
    cargo = shutil.which('cargo') or str(Path.home() / '.cargo/bin/cargo')
    subprocess.run([cargo, 'build', '--locked', '-p', 'dread-wasm', '--target',
                    'wasm32-unknown-unknown', '--profile', 'wasm-release'], cwd=root, env=env, check=True)
wasm = target / 'wasm32-unknown-unknown/wasm-release/dread_wasm.wasm'
data = wasm.read_bytes()
html = (root/'web/index.template.html').read_text(encoding='utf-8')
for key, value in {
    '__DREAD_CSS__': (root/'web/style.css').read_text(encoding='utf-8'),
    '__DREAD_JS__': (root/'web/game.js').read_text(encoding='utf-8'),
    '__DREAD_WASM_BASE64__': base64.b64encode(data).decode(),
}.items():
    html = html.replace(key, value)
dist = root/'dist'
dist.mkdir(exist_ok=True)
output = dist/'Dreadmarch-playtest.html'
output.write_text(html, encoding='utf-8')
receipt = {'wasm_bytes':len(data), 'wasm_sha256':hashlib.sha256(data).hexdigest(),
           'html_bytes':output.stat().st_size,'html_sha256':hashlib.sha256(output.read_bytes()).hexdigest()}
(dist/'web-build.json').write_text(json.dumps(receipt, indent=2)+'\n', encoding='utf-8')
print(json.dumps({'output':str(output),**receipt},indent=2))

# The character-led campaign is the primary experience; retain the skirmish build.
rpg = (root/'web/rpg.template.html').read_text(encoding='utf-8')
for key, value in {
    '__RPG_CSS__': (root/'web/rpg.css').read_text(encoding='utf-8'),
    '__RPG_JS__': (root/'web/rpg.js').read_text(encoding='utf-8'),
    '__DREAD_WASM_BASE64__': base64.b64encode(data).decode(),
}.items():
    rpg = rpg.replace(key, value)
rpg_output = dist/'Dreadmarch-campaign.html'
rpg_output.write_text(rpg, encoding='utf-8')
campaign_receipt = {'campaign': str(rpg_output), 'bytes': rpg_output.stat().st_size,
                    'sha256': hashlib.sha256(rpg_output.read_bytes()).hexdigest(),
                    'wasm_sha256': hashlib.sha256(data).hexdigest()}
(dist/'campaign-build.json').write_text(json.dumps(campaign_receipt, indent=2)+'\n', encoding='utf-8')
print(json.dumps(campaign_receipt))
