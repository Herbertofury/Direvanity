#!/usr/bin/env python3
"""Validate the authored production plan without presenting its estimates as playtime."""
from __future__ import annotations

import argparse
import hashlib
import json
import math
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def validate(plan: dict) -> dict:
    if plan.get("schema_version") != 1 or plan.get("kind") != "production_plan":
        raise ValueError("Expected the versioned production-plan schema")
    if plan.get("status") != "planned_not_shipped":
        raise ValueError("Production estimates must stay explicitly separate from shipped content")
    if not plan.get("duration_evidence") or not plan.get("quality_gates"):
        raise ValueError("Duration uncertainty and quality gates are required")
    targets = plan["targets"]
    for flag in ("repetition_counts_as_authored_content", "mandatory_grind", "daily_attendance"):
        if targets[flag] is not False:
            raise ValueError(f"The campaign contract forbids {flag}")
    if targets["offline_solo"] is not True or targets["maximum_hours"] is not None:
        raise ValueError("Preserve offline solo play and the open-ended production ambition")

    all_ids: set[str] = set()
    result = {"game": plan["game"], "kind": "production_plan_validation",
              "measured_playtime_hours": None, "shipped_duration_claim": False}
    specs = (("acts", "estimated_active_hours", "campaign_min_hours"),
             ("side_arcs", "estimated_active_hours", "side_content_min_hours"),
             ("endgame_disciplines", "estimated_first_mastery_hours", "endgame_first_mastery_min_hours"))
    for group, budget_key, target_key in specs:
        floor = targets[target_key]
        if (not isinstance(floor, (int, float)) or isinstance(floor, bool)
                or not math.isfinite(floor) or floor < 100):
            raise ValueError(f"Invalid accepted 100+ hour planning floor: {target_key}")
        items = plan[group]
        if not items:
            raise ValueError(f"Empty content group: {group}")
        ids = {item["id"] for item in items}
        if len(ids) != len(items) or ids & all_ids:
            raise ValueError(f"Duplicate identity in {group}")
        all_ids |= ids
        dependencies = {}
        low = high = 0.0
        for item in items:
            if item["state"] not in {"planned", "partial"}:
                raise ValueError("This planning catalogue cannot certify production-complete content")
            for field in ("title", "premise"):
                if not isinstance(item[field], str) or not item[field].strip():
                    raise ValueError(f"Missing authored {field}: {item['id']}")
            budget = item[budget_key]
            values = (budget["low"], budget["high"])
            if not all(isinstance(v, (int, float)) and not isinstance(v, bool)
                       and math.isfinite(v) and v > 0 for v in values):
                raise ValueError(f"Invalid active-play estimate: {item['id']}")
            if values[0] > values[1]:
                raise ValueError(f"Reversed active-play estimate: {item['id']}")
            low += values[0]
            high += values[1]
            requires = item.get("requires", [])
            if len(set(requires)) != len(requires) or not set(requires) <= ids:
                raise ValueError(f"Invalid prerequisite: {item['id']}")
            dependencies[item["id"]] = requires
            if group == "acts":
                if not item.get("central_choice") or not item.get("signature_boss_design"):
                    raise ValueError("Each act needs a decision and a distinct boss design")
                for region in item["regions"]:
                    if region["id"] in all_ids:
                        raise ValueError(f"Duplicate region: {region['id']}")
                    all_ids.add(region["id"])
            if group == "side_arcs" and item.get("replay_budget_included") is not False:
                raise ValueError("Do not count mutually exclusive replays as first-playthrough content")
        visited: set[str] = set()
        active: set[str] = set()

        def visit(key: str):
            if key in active:
                raise ValueError(f"Content dependency cycle at {key}")
            if key in visited:
                return
            active.add(key)
            for dependency in dependencies[key]:
                visit(dependency)
            active.remove(key)
            visited.add(key)

        for key in dependencies:
            visit(key)
        if low < targets[target_key]:
            raise ValueError(f"{group} planning floor misses the accepted 100+ hour goal")
        result[group] = {"count": len(items), "estimated_hours_low": low,
                         "estimated_hours_high": high, "measured_hours": None}
    if [a["number"] for a in plan["acts"]] != list(range(1, len(plan["acts"]) + 1)):
        raise ValueError("Acts must be uniquely numbered in order")
    result["region_groups"] = sum(len(a["regions"]) for a in plan["acts"])
    result["side_factions"] = len({a["faction"] for a in plan["side_arcs"]})
    result["result"] = "valid planning data; playtime and production quality remain unproven"
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("path", type=Path, nargs="?", default=ROOT / "data/production/long_campaign.json")
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    data = args.path.read_bytes()
    result = validate(json.loads(data))
    result["source_sha256"] = hashlib.sha256(data).hexdigest()
    rendered = json.dumps(result, indent=2) + "\n"
    if args.output:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(rendered, encoding="utf-8")
    print(rendered, end="")


if __name__ == "__main__":
    main()
