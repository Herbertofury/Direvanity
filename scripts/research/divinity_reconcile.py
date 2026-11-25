#!/usr/bin/env python3
"""Validate the factual atlas or compare local DOS2 Forge exports with it.

Python standard library only. The comparison writes names, IDs, hashes and
classification questions; it never copies exported descriptions or raw stats.
Matching a SkillData label does not prove that a player can acquire that skill.
"""

from __future__ import annotations

import argparse
from collections import Counter, defaultdict
import hashlib
import json
from pathlib import Path
import re
import sys
import unicodedata


DEFAULT_CATALOG = Path(__file__).resolve().parents[2] / "data/research/divinity-catalog.json"
DOS2_EDITIONS = ("original-sin-ii-classic", "original-sin-ii-definitive")
EXPORT_FILES = (
    "skills.json", "statuses.json", "weapons.json", "armor.json", "potions.json",
    "objects.json", "templates.json", "instances.json", "recipes.json",
)
NON_ACTION_KINDS = {
    "talent", "quest_talent", "racial_talent", "gift_bag_talent", "ability_investment",
}
GRANT_FIELD_NAMES = {"skills", "skill", "addskill", "addskills", "skillset"}


def read_json(path: Path):
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except (OSError, ValueError) as exc:
        raise ValueError(f"Cannot read valid JSON from {path}: {exc}") from exc


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def normalize(name: str) -> str:
    """Fold punctuation/case only; explicit catalog aliases handle renamed skills."""
    return "".join(c for c in unicodedata.normalize("NFKD", name).casefold() if c.isalnum())


def validate(catalog: dict) -> dict:
    if not isinstance(catalog, dict):
        raise ValueError("Catalog root must be an object")
    errors: list[str] = []

    def require(condition, message):
        if not condition:
            errors.append(message)

    require(catalog.get("schema_version") == 1, "Unsupported catalog schema")
    games = catalog.get("games", [])
    abilities = catalog.get("abilities", [])
    sources = catalog.get("sources", {})
    game_ids = [g["id"] for g in games]
    ability_ids = [a["id"] for a in abilities]
    require(len(game_ids) == len(set(game_ids)), "Duplicate game IDs")
    require(len(ability_ids) == len(set(ability_ids)), "Duplicate ability IDs")
    require(catalog.get("total_catalog_entries") == len(abilities), "Total count differs from inventory")
    tree_owners = {}
    linked_abilities = Counter()
    entries = {a["id"]: a for a in abilities}

    def check_sources(ids, label):
        require(isinstance(ids, list) and bool(ids), f"Missing sources: {label}")
        for source_id in ids or []:
            require(source_id in sources, f"Unknown source {source_id}: {label}")

    for source_id, source in sources.items():
        require(source.get("url", "").startswith(("https://", "http://")), f"Missing source URL: {source_id}")
        if "commit" in source:
            require(bool(re.fullmatch(r"[0-9a-f]{40}", source["commit"])), f"Invalid commit pin: {source_id}")
    for game in games:
        for tree in game["trees"]:
            tree_id = tree["id"]
            require(tree_id not in tree_owners, f"Duplicate tree: {tree_id}")
            tree_owners[tree_id] = game["id"]
            check_sources(tree.get("source_ids"), tree_id)
            for ability_id in tree["ability_ids"]:
                linked_abilities[ability_id] += 1
                require(ability_id in entries, f"Tree references missing entry: {ability_id}")
                if ability_id in entries:
                    require(entries[ability_id]["tree_id"] == tree_id, f"Tree membership mismatch: {ability_id}")
    for entry in abilities:
        entry_id = entry["id"]
        require(bool(entry.get("name")), f"Missing name: {entry_id}")
        require(entry["game_id"] in game_ids, f"Unknown game: {entry_id}")
        require(tree_owners.get(entry["tree_id"]) == entry["game_id"], f"Wrong tree owner: {entry_id}")
        require(linked_abilities[entry_id] == 1, f"Entry must have exactly one tree binding: {entry_id}")
        check_sources(entry.get("source_ids"), entry_id)
        require("description" not in entry and "tooltip" not in entry, f"Tooltip prose must stay outside factual inventory: {entry_id}")
        if "combination" in entry:
            require(entry["combination"].get("source_id") in sources, f"Unknown hybrid source: {entry_id}")
        if entry["entry_kind"].startswith("gift_bag"):
            require(entry["game_id"] == DOS2_EDITIONS[1], f"Gift bag leaked into another edition: {entry_id}")
            require(bool(entry.get("requires_content") or entry.get("content_gate")), f"Missing optional-content gate: {entry_id}")

    counts = Counter(a["game_id"] for a in abilities)
    ledgers = catalog.get("reconciliation", [])
    require(Counter(r["game_id"] for r in ledgers) == Counter(game_ids), "Reconciliation must cover each game once")
    for row in ledgers:
        game_id = row["game_id"]
        actual_kinds = dict(Counter(a["entry_kind"] for a in abilities if a["game_id"] == game_id))
        require(row.get("included_count") == counts[game_id], f"Included count mismatch: {game_id}")
        require(row.get("discovered_count", -1) >= counts[game_id], f"Discovered count below included: {game_id}")
        require(row.get("entry_kind_counts") == actual_kinds, f"Kind counts mismatch: {game_id}")
        expected = row.get("source_scope_expected_count")
        if expected is not None:
            require(expected - counts[game_id] == row.get("missing_within_enumerated_scope"), f"Source-scope denominator mismatch: {game_id}")
        if row.get("whole_game_expected_count") is None:
            require(row.get("whole_game_complete") is False, f"Unknown denominator cannot be complete: {game_id}")
            require(row.get("unresolved_whole_game_count") is None, f"Unknown denominator must keep unknown remaining count: {game_id}")

    divine = [a for a in abilities if a["game_id"] == "divine-divinity-2002"]
    stages = Counter(a.get("progression_stage") for a in divine if a["entry_kind"] == "tree_skill")
    require(stages == {"pre_ascension": 88, "post_ascension": 8}, "Divine Divinity must have 88 regular + 8 Divine tree skills")
    require(sum(a["entry_kind"] == "class_special" for a in divine) == 3, "Divine Divinity must have three independent class specials")
    for edition in DOS2_EDITIONS:
        hybrids = [a["combination"] for a in abilities if a["game_id"] == edition and "combination" in a]
        pairs = defaultdict(Counter)
        for hybrid in hybrids:
            pairs[tuple(sorted(hybrid["schools"]))][hybrid["variant"]] += 1
        require(len(hybrids) == 48 and len(pairs) == 24, f"Expected 24 ordinary/Source hybrid pairs: {edition}")
        require(all(v == {"ordinary": 1, "Source": 1} for v in pairs.values()), f"Unpaired hybrid variants: {edition}")
    for project in catalog.get("projects", []):
        require(project.get("source_id") in sources, f"Project lacks source: {project.get('id')}")
    return {"valid": not errors, "errors": errors, "catalog_entries": len(abilities), "games": dict(counts), "source_count": len(sources)}


def read_records(path: Path) -> list[dict]:
    records = read_json(path)
    if not isinstance(records, list) or not all(isinstance(row, dict) for row in records):
        raise ValueError(f"{path.name} must be an array of object records")
    return records


def grant_tokens(value):
    """Extract only explicit Skill/Skills/SkillSet fields, never arbitrary prose."""
    if isinstance(value, dict):
        for key, child in value.items():
            if str(key).casefold() in GRANT_FIELD_NAMES:
                values = child if isinstance(child, list) else [child]
                for raw in values:
                    if isinstance(raw, str):
                        for token in re.split(r"[;,]", raw):
                            if token.strip():
                                yield str(key), token.strip()
            elif isinstance(child, (dict, list)):
                yield from grant_tokens(child)
    elif isinstance(value, list):
        for child in value:
            yield from grant_tokens(child)


def reconcile(catalog: dict, game_id: str, export_dir: Path, build: str | None, enabled_content: list[str]) -> dict:
    inputs = {}
    datasets = {}
    for filename in EXPORT_FILES:
        path = export_dir / filename
        if path.is_file():
            records = read_records(path)
            datasets[filename] = records
            inputs[filename] = {"sha256": digest(path), "records": len(records)}
    skills = datasets.get("skills.json")
    if not skills:
        raise ValueError("A nonempty skills.json from dos2forge export skills/all is required")
    seen_stat_ids = set()
    for row in skills:
        if not isinstance(row.get("name"), str) or not row["name"].strip() or row.get("type") != "SkillData":
            raise ValueError("Every skills.json row must have a nonempty internal name and type SkillData")
        if row["name"] in seen_stat_ids:
            raise ValueError(f"Duplicate SkillData internal ID: {row['name']}")
        seen_stat_ids.add(row["name"])
        if row.get("display_name") is not None and not isinstance(row["display_name"], str):
            raise ValueError(f"Invalid display_name on {row['name']}")

    actions = [a for a in catalog["abilities"] if a["game_id"] == game_id and a["entry_kind"] not in NON_ACTION_KINDS]
    name_index = defaultdict(set)
    for entry in actions:
        for name in [entry["name"], *entry.get("aliases", [])]:
            name_index[normalize(name)].add(entry["id"])
    matched_ids = set()
    matches = []
    ambiguous = []
    unknown = []
    unlocalized = []
    for row in skills:
        label = (row.get("display_name") or "").strip()
        identity = {"stat_id": row["name"], "display_name": label}
        if not label or re.fullmatch(r"h[0-9a-fA-Fg-]{30,}", label):
            unlocalized.append(identity)
            continue
        candidates = sorted(name_index.get(normalize(label), ()))
        if len(candidates) == 1:
            matched_ids.add(candidates[0])
            matches.append({**identity, "catalog_id": candidates[0]})
        elif candidates:
            ambiguous.append({**identity, "candidate_catalog_ids": candidates})
        else:
            unknown.append(identity)

    explicit_grants = []
    for filename, records in datasets.items():
        if filename == "skills.json":
            continue
        for index, record in enumerate(records):
            for field, token in sorted(set(grant_tokens(record))):
                explicit_grants.append({
                    "dataset": filename,
                    "record_id": record.get("name") or record.get("map_key") or f"row:{index}",
                    "field": field,
                    "token": token,
                    "resolves_to_exported_skill_id": token in seen_stat_ids,
                })
    absent = [{"id": a["id"], "name": a["name"], "entry_kind": a["entry_kind"],
               **({"requires_content": a["requires_content"]} if a.get("requires_content") else {})}
              for a in actions if a["id"] not in matched_ids]
    return {
        "schema_version": 1,
        "game_id": game_id,
        "declared_build": build,
        "declared_enabled_content": enabled_content,
        "input_files": inputs,
        "missing_input_files": [f for f in EXPORT_FILES if f not in inputs],
        "counts": {
            "exported_skilldata_records": len(skills), "catalog_action_records": len(actions),
            "matched_catalog_action_records": len(matched_ids), "matching_stat_records": len(matches),
            "ambiguous_stat_records": len(ambiguous), "unknown_stat_records": len(unknown),
            "unlocalized_stat_records": len(unlocalized), "catalog_actions_without_unique_label_match": len(absent),
            "explicit_grant_tokens": len(explicit_grants),
        },
        "matches": matches,
        "ambiguous_matches": ambiguous,
        "unknown_stat_names": unknown,
        "unlocalized_stat_names": unlocalized,
        "catalog_actions_without_unique_label_match": absent,
        "explicit_grant_tokens": explicit_grants,
        "whole_game_complete": False,
        "limitations": [
            "A name match links a catalog fact to SkillData; it does not establish player acquisition or gameplay behavior.",
            "Unknown SkillData can be NPC, unused, template, modded or renamed content; do not count every unknown row as a missing player ability.",
            "Missing labels can reflect localization, aliases, optional-content enablement or edition differences.",
            "Talents, civil/combat investments and traits are not compared with the SkillData dataset.",
            "Explicit grant tokens are leads only. Template inheritance, skill sets, status scripts, recipes and quest grants need further version-specific inspection.",
            "The report retains no description/tooltip prose, raw stats or commercial assets.",
            "Build and enabled-content labels are caller declarations, not an independently verified installation manifest.",
        ],
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--catalog", type=Path, default=DEFAULT_CATALOG)
    sub = parser.add_subparsers(dest="command", required=True)
    sub.add_parser("validate", help="check IDs, sources, tree links and count denominators")
    sub.add_parser("requirements", help="show exact known installed-data requirements and gaps")
    command = sub.add_parser("reconcile", help="compare a local DOS2 Forge export without copying prose")
    command.add_argument("--game-id", choices=DOS2_EDITIONS, required=True)
    command.add_argument("--export-dir", type=Path, required=True)
    command.add_argument("--output", type=Path, required=True)
    command.add_argument("--build", help="declared installed game version/build")
    command.add_argument("--enabled-content", action="append", default=[], help="declared optional content/mod; repeatable")
    args = parser.parse_args()
    try:
        catalog = read_json(args.catalog)
        validation = validate(catalog)
        if not validation["valid"] or args.command == "validate":
            print(json.dumps(validation, indent=2))
            return 0 if validation["valid"] else 1
        if args.command == "requirements":
            print(json.dumps(catalog["original_data_requirements"], indent=2, ensure_ascii=False))
            return 0
        report = reconcile(catalog, args.game_id, args.export_dir, args.build, args.enabled_content)
        report["catalog_sha256"] = digest(args.catalog)
        target = args.output.resolve()
        protected = {args.catalog.resolve(), *(p.resolve() for p in args.export_dir.glob("*.json"))}
        if target in protected:
            raise ValueError("The report must not overwrite its catalog or input exports")
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(json.dumps(report, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
        print(json.dumps({"output": str(target), **report["counts"], "whole_game_complete": False}, indent=2))
        return 0
    except (OSError, ValueError, KeyError, TypeError) as exc:
        print(f"error: {exc}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
