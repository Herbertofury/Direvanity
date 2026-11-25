#!/usr/bin/env python3
"""Extract factual ARPG reference inventories from pinned public data sources.

This is research data, not an import of third-party gameplay code or assets.
No tooltip prose, artwork, sound, or numerical damage implementation is copied.
The complete input tables are examined. Coverage and excluded record classes
are counted; a complete extraction is not a claim of complete live-game parity.
"""
from __future__ import annotations

import argparse
import collections
import concurrent.futures
import hashlib
import html
import json
import re
import urllib.request
from pathlib import Path

from arpg_design import design_metadata

ROOT = Path(__file__).resolve().parents[2]
LOA_REV = "2f89591450db347b2cf802fc0f7828768e769720"
POB1_REV = "5606cc64fa9f15694e69001a52b90aa968d01a86"
POB2_REV = "601768bd105869618c3d6581b883e5d7aeec91b2"
TREE1_REV = "8bd138b32ea2631455cac5935bfab089f826094f"
TREE2_REV = "bd87e6512c92b868542eddfb1ba4ea8b6dc2da36"
PREVIEW1_REV = "550fb34ae7fca9ec4d760a2ac845bb2ba1032ec3"
FILES = {
    name: f"https://raw.githubusercontent.com/snoww/loa-logs/{LOA_REV}/src-tauri/meter-data/{name}"
    for name in ["PCData.json", "SkillWindows.json", "Skill.json", "SkillFeature.json", "ArkPassive.json", "ArkGrid.json"]
}
FILES.update({
    "poe1_gems.lua": f"https://raw.githubusercontent.com/PathOfBuildingCommunity/PathOfBuilding/{POB1_REV}/src/Data/Gems.lua",
    "poe2_gems.lua": f"https://raw.githubusercontent.com/PathOfBuildingCommunity/PathOfBuilding-PoE2/{POB2_REV}/src/Data/Gems.lua",
    "poe1_tree.json": f"https://raw.githubusercontent.com/grindinggear/skilltree-export/{TREE1_REV}/data.json",
    "poe2_tree.json": f"https://raw.githubusercontent.com/grindinggear/poe2-skilltree-export/{TREE2_REV}/data.json",
    "poe1_tree_preview.json": f"https://raw.githubusercontent.com/grindinggear/skilltree-export/{PREVIEW1_REV}/data.json",
})
BASE_CLASSES = {0, 101, 111, 201, 301, 311, 401, 501, 511, 601}
CLASS_RECONCILIATION = {604: "Wildsoul", 612: "Dimensionalist", 702: "Guardianknight"}


def clean_label(value: str | None) -> str:
    return html.unescape(re.sub(r"<[^>]*>", "", value or "")).replace("{0}", "").strip()


def factual_structure(value):
    """Preserve nested identifiers/relationships without tooltip prose."""
    if isinstance(value, dict):
        return {key: factual_structure(child) for key, child in value.items()
                if key not in {"description", "desc", "stats", "flavourText", "reminderText"}}
    if isinstance(value, list):
        return [factual_structure(child) for child in value]
    return value


def extract_gems(text: str) -> list[dict]:
    # Generated PoB tables use one-tab top-level keys and two-tab properties.
    # Anchor at this indentation so nested tags cannot become false records.
    starts = list(re.finditer(r'^\t\["([^"\n]+)"\] = \{\s*$', text, re.M))
    records = []
    for index, start in enumerate(starts):
        block = text[start.end():starts[index + 1].start() if index + 1 < len(starts) else len(text)]
        values = {m.group(1): json.loads('"' + m.group(2) + '"')
                  for m in re.finditer(r'^\t\t(\w+) = "((?:[^"\\]|\\.)*)",?$', block, re.M)}
        if not values.get("name"):
            raise ValueError(f"Top-level gem {start.group(1)} has no parsed name")
        tag_block = re.search(r'^\t\ttags = \{(.*?)^\t\t\}', block, re.M | re.S)
        tags = re.findall(r'^\t\t\t(\w+) = true,?$', tag_block.group(1), re.M) if tag_block else []
        record = {"id": start.group(1), "source_ordinal": index + 1, "name": values["name"], "tags": tags}
        for key in ["baseTypeName", "gameId", "variantId", "grantedEffectId", "secondaryGrantedEffectId", "gemType", "weaponRequirements"]:
            if key in values:
                record[key] = values[key]
        record["category"] = "support" if "support" in tags or re.search(r'^\t\tsupport = true', block, re.M) else "active_or_granted"
        if re.search(r'^\t\tvaalGem = true', block, re.M):
            record["vaal_variant"] = True
        records.append(record)
    assert len(records) == len(starts)
    occurrences = collections.Counter(r["id"] for r in records)
    final_ordinals = {r["id"]: r["source_ordinal"] for r in records}
    for record in records:
        if occurrences[record["id"]] > 1:
            record["duplicate_source_key"] = True
            record["effective_in_lua_table"] = record["source_ordinal"] == final_ordinals[record["id"]]
    return records


def extract_tree(data: dict, version: str, source: str, status: str) -> dict:
    classes = [{"name": c["name"], "ascendancies": [
        {"id": a["id"], "name": a.get("name"), "named": bool(a.get("name")),
         "override_pairs": a.get("overridePairs", {})}
        for a in c.get("ascendancies", [])], "override_pairs": c.get("overridePairs", {})}
        for c in data["classes"]]
    alternatives = [{"id": a["id"], "name": a.get("name")}
                    for a in data.get("alternate_ascendancies", [])]
    nodes = []
    for node_id, row in data["nodes"].items():
        node = {"id": node_id, "name": row.get("name"), "out": [str(x) for x in row.get("out", [])]}
        for key in ["ascendancyName", "ascendancyId", "isKeystone", "isNotable", "isMastery", "isJewelSocket", "isAscendancyStart", "isBloodline", "classStartIndex", "isFree", "isGenericAttribute", "isMultipleChoice", "isMultipleChoiceOption", "multipleChoiceParent", "unlockConstraint", "hideConnection", "grantedSkill"]:
            if key in row:
                node[key] = factual_structure(row[key])
        if "masteryEffects" in row:
            node["mastery_effect_ids"] = [m.get("effect") for m in row["masteryEffects"]]
        nodes.append(node)
    override_nodes = [{"key": k, "id": r.get("id"), "skill": r.get("skill"), "name": r.get("name")}
                      for k, r in data.get("skillOverrides", {}).items()]
    node_ids = {n["id"] for n in nodes}
    outgoing = [target for n in nodes for target in n["out"]]
    dangling = sorted(set(outgoing) - node_ids)
    assert not dangling, f"Unresolved passive tree links: {dangling}"
    return {"source": source, "version": version, "status": status, "classes": classes,
            "alternate_ascendancies": alternatives, "skill_overrides": override_nodes,
            "nodes": nodes, "coverage": {"source_node_count": len(data["nodes"]),
            "included_node_count": len(nodes), "named_node_count": sum(bool(n["name"]) for n in nodes),
            "unnamed_structural_node_count": sum(not n["name"] for n in nodes),
            "named_ascendancy_count": sum(a["named"] for c in classes for a in c["ascendancies"]),
            "alternate_ascendancy_records": len(alternatives), "skill_override_records": len(override_nodes),
            "outgoing_edges": len(outgoing), "dangling_edges": len(dangling),
            "omitted_source_nodes": 0, "proof": "Every source node ID and outgoing edge retained; no pagination.",
            "limits": "Class slots and unnamed ascendancies in exported data are not proof of released playable classes. Tooltip prose and modifier values are not included in this research-name/graph index."}}


def extract_lostark(load) -> dict:
    names = {int(k): v for k, v in load("PCData.json").items()}
    names.update(CLASS_RECONCILIATION)
    skills = load("Skill.json")
    features = load("SkillFeature.json")
    passives = load("ArkPassive.json")
    grid = load("ArkGrid.json")
    named, unnamed, nonplayer = [], [], 0
    for key, row in skills.items():
        cid = row.get("classId", 0)
        if not cid:
            nonplayer += 1
            continue
        if not row.get("name"):
            unnamed.append({"id": int(key), "class_id": cid})
            continue
        item = {"id": int(key), "name": row["name"], "class_id": cid,
                "class": names.get(cid, f"unresolved:{cid}"), "type": row.get("type"),
                "grade": row.get("grade"), "identity_category": row.get("identityCategory"),
                "hyper_awakening": bool(row.get("isHyperAwakening")),
                "cooldown_ms": row.get("cooldown"), "source_level_count": len(row.get("levels", {}))}
        if key in features:
            item["tripods"] = [{"id": int(tid), "name": tr.get("name"),
                                "effect_types": sorted({e.get("type", "unknown") for e in tr.get("entries", [])}),
                                "levels": sorted({e.get("level", 0) for e in tr.get("entries", [])})}
                               for tid, tr in features[key].get("tripods", {}).items()]
        named.append(item)
    # Retain every tripod group independently, including groups without named
    # player-skill rows. Joining only visible skills would silently lose facts.
    tripods = [{"skill_id": row.get("skillId", int(key)), "skill_name": skills.get(key, {}).get("name"),
                "class_id": skills.get(key, {}).get("classId"),
                "choices": [{"id": int(tid), "name": tr.get("name"),
                              "effect_types": sorted({e.get("type", "unknown") for e in tr.get("entries", [])}),
                              "levels": sorted({e.get("level", 0) for e in tr.get("entries", [])})}
                             for tid, tr in row.get("tripods", {}).items()]}
               for key, row in features.items()]
    passive_rows = [{"id": int(key), "levels": [{"level": int(level), "name": value.get("name"),
                    "effect_types": sorted({a.get("type", "unknown") for a in value.get("addons", [])})}
                    for level, value in row.get("levels", {}).items()]}
                    for key, row in passives.items()]
    cores = [{"id": int(key), "class_id": row.get("pcClass"),
              "class": names.get(row.get("pcClass"), "universal" if not row.get("pcClass") else "unresolved"),
              "name": clean_label(row.get("title")), "category": clean_label(row.get("category")),
              "grade": row.get("grade"), "group_id": row.get("groupId"),
              "options": [{"id": o.get("optionId"), "required_points": o.get("requiredPoints")}
                          for o in row.get("options", [])]}
             for key, row in grid["cores"].items()]
    core_options = [{"id": int(key), "name": clean_label(row.get("name")),
                     "effect_types": sorted({a.get("type", "unknown") for a in row.get("addons", [])}),
                     "stat_types": sorted({a.get("statType", "unknown") for a in row.get("addons", [])}),
                     "include_effect_node_id": row.get("includeEffectNodeId")}
                    for key, row in grid["coreOptions"].items()]
    grid_gems = [{"id": int(key), "classify_type": row.get("classifyType"),
                  "classify_index": row.get("classifyIndex"), "gem_type": row.get("gemType"),
                  "option_groups": [{"id": o.get("optionId"), "order": o.get("order"),
                     "levels": [{"level": r.get("level"), "name": clean_label(r.get("name")),
                                 "effect_type": r.get("addonType"), "stat_type": r.get("statType")}
                                for r in o.get("levels", [])]}
                                    for o in row.get("optionGroups", [])]}
                 for key, row in grid["gems"].items()]
    actual_class_ids = sorted({r["class_id"] for r in named if r["class_id"] not in BASE_CLASSES})
    roster = [{"id": cid, "name": names[cid], "named_skill_rows": sum(r["class_id"] == cid for r in named)} for cid in actual_class_ids]
    assert len(named) + len(unnamed) + nonplayer == len(skills)
    assert len(tripods) == len(features) and len(passive_rows) == len(passives)
    return {"source": FILES["Skill.json"], "revision": LOA_REV,
            "provenance": "Community maintained game-data tables; cross-checked against official western class releases, not a Smilegate API export.",
            "classes": roster, "named_skill_records": named, "unnamed_class_record_ids": unnamed,
            "tripod_groups": tripods, "ark_passives": passive_rows, "ark_grid_cores": cores,
            "ark_grid_bases": list(grid["bases"].values()), "ark_grid_core_options": core_options,
            "ark_grid_gems": grid_gems,
            "class_name_reconciliation": [{"id": k, "name": v, "reason": "PCData label is missing or an internal codename; current public class release and skill identity resolve it."} for k, v in CLASS_RECONCILIATION.items()],
            "coverage": {"source_skill_rows": len(skills), "included_named_class_rows": len(named),
                         "included_unnamed_class_ids": len(unnamed), "excluded_class_zero_rows": nonplayer,
                         "advanced_classes": len(roster), "tripod_groups": len(tripods),
                         "tripod_choices": sum(len(r["choices"]) for r in tripods),
                         "ark_passive_records": len(passive_rows), "ark_passive_level_rows": sum(len(r["levels"]) for r in passive_rows),
                         "ark_grid_core_variants": len(cores), "omitted_tripod_groups": 0,
                         "ark_grid_bases": len(grid["bases"]), "ark_grid_core_options": len(core_options),
                         "ark_grid_gems": len(grid_gems),
                         "excluded_economy_tables": {k: len(grid[k]) for k in ["processes", "gemFusionRules", "gemMaterialFusionRules"]},
                         "unknown_live_total": True,
                         "limits": ["Auxiliary, legacy and transformation skill rows can share display names; IDs remain separate.",
                                    "Raw class-zero rows include NPC, item and generic effects; excluded from player-class catalogue and counted.",
                                    "This dataset does not expose complete player-facing Ark Passive prerequisite edges or reliable tier/class metadata; do not invent those relationships.",
                                    "All extracted source rows are reconciled. Complete extraction of a pinned dataset does not prove that every live regional skill or modifier is present."]}}


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--cache", type=Path, default=ROOT / "dist/research-cache/arpg")
    ap.add_argument("--fetch", action="store_true", help="Download missing pinned inputs only")
    ap.add_argument("--output", type=Path, default=ROOT / "data/research/arpg-comparison.json")
    args = ap.parse_args()
    args.cache.mkdir(parents=True, exist_ok=True)
    missing = [(name, url) for name, url in FILES.items() if not (args.cache / name).is_file()]
    if missing and not args.fetch:
        raise SystemExit("Missing pinned inputs; rerun with --fetch: " + ", ".join(n for n, _ in missing))
    def fetch(pair):
        name, url = pair
        with urllib.request.urlopen(url, timeout=60) as response:
            data = response.read()
        (args.cache / name).write_bytes(data)
    if missing:
        with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:
            list(pool.map(fetch, missing))
    def load(name):
        return json.loads((args.cache / name).read_text(encoding="utf-8"))
    receipt = [{"file": name, "url": url, "bytes": (args.cache / name).stat().st_size,
                "sha256": hashlib.sha256((args.cache / name).read_bytes()).hexdigest()} for name, url in FILES.items()]
    data = {"schema": 1, "researched_at": "2026-11-25", "purpose": "Factual reference names, identifiers, tags and tree structure for original Dreadmarch class design.",
            "implementation_status": "Reference catalogue; these upstream abilities are not implemented by their presence here.",
            "source_receipts": receipt, "lost_ark": extract_lostark(load),
            "design_synthesis": design_metadata()}
    for prefix, version, revision in [("poe1", "3.29.1", TREE1_REV), ("poe2", "0.5.5", TREE2_REV)]:
        gems = extract_gems((args.cache / (prefix + "_gems.lua")).read_text())
        data[prefix] = {"gem_source": FILES[prefix + "_gems.lua"], "gem_provenance": "Path of Building Community generated gem data; header attributes game data to Grinding Gear Games.",
                        "gems": gems, "gem_coverage": {"source_top_level_entries": len(gems), "included": len(gems), "omitted": 0,
                          "categories": dict(collections.Counter(r["category"] for r in gems)),
                          "unique_display_names": len({r["name"] for r in gems}),
                          "unique_source_keys": len({r["id"] for r in gems}),
                          "duplicate_source_key_extra_entries": len(gems) - len({r["id"] for r in gems}),
                          "limits": "Includes variants and possible legacy/unused records from the pinned planner; game availability is not inferred from a name alone."},
                        "passive_tree": extract_tree(load(prefix + "_tree.json"), version, FILES[prefix + "_tree.json"], "published_stable_export")}
    data["poe1_preview"] = extract_tree(load("poe1_tree_preview.json"), "3.30.0 preview", FILES["poe1_tree_preview.json"], "preview_not_live_baseline")
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(data, ensure_ascii=False, separators=(",", ":")) + "\n", encoding="utf-8")
    print(json.dumps({"output": str(args.output), "bytes": args.output.stat().st_size,
                      "lost_ark": data["lost_ark"]["coverage"],
                      "poe1_gems": data["poe1"]["gem_coverage"], "poe2_gems": data["poe2"]["gem_coverage"],
                      "poe1_nodes": len(data["poe1"]["passive_tree"]["nodes"]), "poe2_nodes": len(data["poe2"]["passive_tree"]["nodes"])}, indent=2))


if __name__ == "__main__":
    main()
