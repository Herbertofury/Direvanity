"""Planning thresholds cannot bypass the user's accepted scope through NaN or type coercion."""
from copy import deepcopy
import importlib.util
import json
from pathlib import Path
import unittest

ROOT = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location("content_plan", ROOT / "scripts/validate_content_plan.py")
validator = importlib.util.module_from_spec(spec)
spec.loader.exec_module(validator)
PLAN = json.loads((ROOT / "data/production/long_campaign.json").read_text(encoding="utf-8"))


class ProductionThresholdTests(unittest.TestCase):
    def test_current_plan_is_explicitly_unmeasured(self):
        result = validator.validate(PLAN)
        self.assertIsNone(result["measured_playtime_hours"])
        self.assertFalse(result["shipped_duration_claim"])

    def test_invalid_floors_cannot_bypass_accepted_scope(self):
        for key in ("campaign_min_hours", "side_content_min_hours", "endgame_first_mastery_min_hours"):
            for value in (float("nan"), float("inf"), True, False, -1, 0, 99, "100"):
                with self.subTest(key=key, value=value):
                    plan = deepcopy(PLAN)
                    plan["targets"][key] = value
                    with self.assertRaises(ValueError):
                        validator.validate(plan)

    def test_small_campaign_cannot_hide_behind_nan(self):
        plan = deepcopy(PLAN)
        plan["targets"]["campaign_min_hours"] = float("nan")
        for act in plan["acts"]:
            act["estimated_active_hours"] = {"low": 0.01, "high": 0.02}
        with self.assertRaises(ValueError):
            validator.validate(plan)


if __name__ == "__main__":
    unittest.main()
