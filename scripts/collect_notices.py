#!/usr/bin/env python3
"""Collect complete, locked Rust dependency notices without network access.

Run `cargo fetch --locked` once before this script in a fresh checkout. Missing
license files from published crates are supplied by reviewed, commit-pinned
copies in third_party/notices; the collector never substitutes generic license
text for a package's own notice. It fails if a locked package or a required
cached notice is unavailable.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import tomllib


NOTICE_NAME = re.compile(
    r"(^|[._-])(licen[cs](?:e|ing)|copying|copyright|notice|unlicen[cs]e|ofl|authors|patents)([._-]|$)",
    re.IGNORECASE,
)
SOURCE_SUFFIXES = {".rs", ".c", ".h", ".cpp", ".js", ".ts", ".py", ".stderr"}


def digest(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def read_text(path: Path) -> str:
    # Decode explicitly so the Windows build does not depend on its code page.
    return path.read_bytes().decode("utf-8-sig")


def contained_file(root: Path, relative: str) -> Path:
    path = (root / relative).resolve()
    if not path.is_relative_to(root.resolve()):
        raise ValueError(f"Notice path escapes its declared source: {relative}")
    return path


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", default="dist/THIRD_PARTY_NOTICES.txt")
    parser.add_argument(
        "--allow-incomplete", action="store_true",
        help="Write an explicitly incomplete bundle without a failing exit status; never suppresses the exception list",
    )
    parser.add_argument(
        "--metadata", type=Path,
        help="Read an existing cargo metadata --locked --offline --format-version 1 result",
    )
    args = parser.parse_args()
    root = Path(__file__).resolve().parents[1]
    lock_bytes = (root / "Cargo.lock").read_bytes()
    lock = tomllib.loads(lock_bytes.decode("utf-8"))
    if args.metadata:
        metadata = json.loads(read_text(args.metadata))
    else:
        result = subprocess.run(
            [os.environ.get("CARGO", "cargo"), "metadata", "--locked", "--offline", "--format-version", "1"],
            cwd=root, check=False, capture_output=True, text=True, encoding="utf-8",
        )
        if result.returncode:
            print(result.stderr, file=sys.stderr)
            print("Materialize the exact lock first with `cargo fetch --locked`.", file=sys.stderr)
            return result.returncode
        metadata = json.loads(result.stdout)

    packages = {
        (p["name"], p["version"], p.get("source")): p
        for p in metadata["packages"]
    }
    cache_root = root / "third_party" / "notices"
    cache = json.loads(read_text(cache_root / "index.json"))
    if cache.get("schema") != 1:
        raise ValueError("Unsupported notice cache schema")
    cached_records: dict[str, list[dict]] = {}
    reviewed_exceptions = {
        package: item for item in cache.get("unresolved", []) for package in item["packages"]
    }
    for record in cache["records"]:
        for package in record["packages"]:
            cached_records.setdefault(package, []).append(record)

    exceptions: list[str] = []
    sections: list[str] = []
    inventory: list[str] = []
    source_files = 0
    recovered_files = 0
    font_assets = 0
    seen_cached: set[str] = set()
    third_party = sorted(
        (p for p in lock["package"] if p.get("source")),
        key=lambda p: (p["name"].lower(), p["version"], p["source"]),
    )

    for locked in third_party:
        name = locked["name"]
        version = locked["version"]
        identity = f"{name}@{version}"
        package = packages.get((name, version, locked["source"]))
        if package is None:
            exceptions.append(f"{identity}: locked package absent from Cargo metadata")
            continue
        source_root = Path(package["manifest_path"]).parent
        vcs_path = source_root / ".cargo_vcs_info.json"
        vcs = json.loads(read_text(vcs_path)) if vcs_path.is_file() else {}
        local = {
            f for f in source_root.rglob("*")
            if f.is_file() and NOTICE_NAME.search(f.name) and f.suffix.lower() not in SOURCE_SUFFIXES
        }
        if package.get("license_file"):
            explicit = contained_file(source_root, package["license_file"])
            if explicit.is_file():
                local.add(explicit)
            else:
                exceptions.append(f"{identity}: manifest license_file is missing: {package['license_file']}")
        blocks: list[str] = []
        for path in sorted(local, key=lambda p: p.relative_to(source_root).as_posix()):
            relative = path.relative_to(source_root).as_posix()
            raw = path.read_bytes()
            try:
                text = raw.decode("utf-8-sig")
            except UnicodeDecodeError:
                exceptions.append(f"{identity}: notice is not UTF-8 and requires explicit review: {relative}")
                continue
            blocks.append(
                f"--- Published crate file: {relative}\n"
                f"SHA-256: {digest(raw)}\n\n{text}"
            )
            source_files += 1

        crate_recovered = 0
        for record in cached_records.get(identity, []):
            if vcs.get("git", {}).get("sha1") != record["revision"]:
                exceptions.append(f"{identity}: recovered notice revision does not match published crate provenance")
                continue
            path = contained_file(cache_root, record["file"])
            if not path.is_file():
                exceptions.append(f"{identity}: recovered notice is missing: {record['file']}")
                continue
            raw = path.read_bytes()
            if digest(raw) != record["sha256"]:
                exceptions.append(f"{identity}: recovered notice hash mismatch: {record['file']}")
                continue
            seen_cached.add(record["file"])
            if record.get("asset"):
                asset = contained_file(source_root, record["asset"])
                if not asset.is_file() or digest(asset.read_bytes()) != record["asset_sha256"]:
                    exceptions.append(f"{identity}: bundled asset does not match recorded notice: {record['asset']}")
                    continue
                font_assets += 1
            else:
                crate_recovered += 1
            blocks.append(
                f"--- Recovered upstream notice: {record['upstream_path']}\n"
                f"Source: {record['url']}\n"
                f"Revision: {record['revision']}\n"
                f"SHA-256: {record['sha256']}\n"
                + (f"Bundled asset: {record['asset']}\nAsset SHA-256: {record['asset_sha256']}\n" if record.get("asset") else "")
                + f"\n{raw.decode('utf-8-sig')}"
            )
            recovered_files += 1
        if not local and not crate_recovered:
            detail = reviewed_exceptions.get(identity)
            exceptions.append(
                f"{identity}: no package license/notice text found; declared license: {package.get('license') or 'unspecified'}"
                + (f". Reviewed {detail['repository']} at {detail['revision']}: {detail['reason']}" if detail else "")
            )
            declaration = source_root / "Cargo.toml.orig"
            if declaration.is_file():
                raw = declaration.read_bytes()
                blocks.append(
                    "--- License declaration evidence only; no upstream license text was supplied: Cargo.toml.orig\n"
                    f"SHA-256: {digest(raw)}\n\n{raw.decode('utf-8-sig')}"
                )
        if name == "bevy_text" and not any(r.get("asset") for r in cached_records.get(identity, [])):
            exceptions.append(f"{identity}: embedded default font requires its own recorded license notice")

        inventory.append(f"{identity} | {package.get('license') or 'unspecified'} | {len(blocks)} text(s)")
        sections.append(
            "=" * 78 + f"\n{name} {version}\n" + "=" * 78 + "\n"
            f"Declared package license: {package.get('license') or 'unspecified'}\n"
            f"Package source: {locked['source']}\n"
            f"Package checksum: {locked.get('checksum', 'not supplied')}\n"
            f"Upstream repository: {package.get('repository') or 'not supplied'}\n"
            f"Authors: {', '.join(package.get('authors', [])) or 'not supplied'}\n\n"
            + "\n\n".join(blocks)
        )

    header = (
        "DIREVANITY — THIRD-PARTY LICENSES AND NOTICES\n"
        "Generated by scripts/collect_notices.py from exact Cargo.lock and materialized sources.\n"
        f"Cargo.lock SHA-256: {digest(lock_bytes)}\n"
        f"Locked third-party packages: {len(third_party)}\n"
        f"Collected package entries: {len(inventory)}\n"
        f"Published package notice files: {source_files}\n"
        f"Recovered upstream notice occurrences: {recovered_files}\n"
        f"Distinct recovered files: {len(seen_cached)}\n"
        f"Embedded font notice occurrences: {font_assets}\n\n"
        "SCOPE\n"
        "This bundle includes every third-party package in Cargo.lock, including build tools,\n"
        "optional dependencies and dependencies for other platforms. Inclusion does not assert\n"
        "that every package or every upstream asset is linked into a particular executable.\n"
        "Complete license/notice files are reproduced; package SPDX expressions are metadata,\n"
        "not replacements for those texts. The project's own MIT/Apache licenses accompany\n"
        "the distribution separately. System libraries and the Rust compiler toolchain are\n"
        "not Cargo packages and are outside this registry-dependency inventory.\n\n"
        "EXCEPTIONS\n"
        + ("\n".join(f"- {item}" for item in exceptions) if exceptions else "None: all locked packages have license/notice texts and required cached notices passed SHA-256 checks.")
        + "\n\nPACKAGE INDEX\n" + "\n".join(inventory) + "\n\n"
    )
    output = Path(args.output)
    if not output.is_absolute():
        output = root / output
    output.parent.mkdir(parents=True, exist_ok=True)
    data = (header + "\n\n".join(sections) + "\n").encode("utf-8")
    temporary = output.with_name(output.name + ".tmp")
    temporary.write_bytes(data)
    temporary.replace(output)
    print(json.dumps({
        "output": str(output), "sha256": digest(data), "bytes": len(data),
        "locked_packages": len(third_party), "collected_packages": len(inventory),
        "package_notice_files": source_files, "recovered_notice_occurrences": recovered_files,
        "distinct_recovered_files": len(seen_cached), "embedded_font_notices": font_assets,
        "exceptions": exceptions,
    }, indent=2))
    return 1 if exceptions and not args.allow_incomplete else 0


if __name__ == "__main__":
    sys.exit(main())
