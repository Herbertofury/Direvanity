//! Original authored chapter and deterministic, persistent Concord expeditions.
//! Duration is a production/playtest property, never manufactured by these rules.
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChronicleState {
    pub version: u32,
    pub chapter: u32,
    pub choice: String,
    pub side_quests: BTreeSet<String>,
    pub expedition: ExpeditionState,
}
impl Default for ChronicleState {
    fn default() -> Self {
        Self {
            version: 1,
            chapter: 0,
            choice: String::new(),
            side_quests: BTreeSet::new(),
            expedition: ExpeditionState::default(),
        }
    }
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExpeditionState {
    pub attempts: u32,
    pub completed: u32,
    pub best_depth: u32,
    pub active: Option<ExpeditionRun>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpeditionRun {
    pub depth: u32,
    pub seed: u64,
    pub objective: String,
    pub modifier: String,
    pub kills: u32,
    pub enemies_total: u32,
    pub warden_id: u32,
    pub warden_defeated: bool,
    pub seals: BTreeSet<String>,
    pub reward_claimed: bool,
}
impl ExpeditionRun {
    pub fn objective_complete(&self) -> bool {
        match self.objective.as_str() {
            "hunt" => self.warden_defeated,
            "seals" => self.seals.len() == 3,
            "purge" => self.kills == self.enemies_total,
            _ => false,
        }
    }
    pub fn objective_text(&self) -> String {
        if self.reward_claimed {
            return "Expedition complete. Return through the western rift; your next expedition will be deeper.".into();
        }
        if self.objective_complete() {
            return "The objective is complete. Claim the Concord cache in the southeast; it also recovers every uncollected drop.".into();
        }
        match self.objective.as_str() {
            "hunt" => "Defeat the Concord Warden. Other enemies are optional; claim the southeastern cache afterward.".into(),
            "seals" => format!("Close the three fracture seals ({}/3). Clear nearby enemies before touching each seal.", self.seals.len()),
            _ => format!("Purge the fracture: {}/{} enemies defeated. Claim the southeastern cache when the region is clear.", self.kills, self.enemies_total),
        }
    }
}
impl ChronicleState {
    pub fn quest_text(&self) -> &'static str {
        match self.chapter {
            0 => {
                "Choose the Concord's fate at the oath stones beside Maelin, then take Hearthmere's northern road to the Ember Coast."
            }
            1 => {
                "Clear the Ember Coast and rekindle its beacon. Seek the ferryman beside the southern shore."
            }
            2 => {
                "Cross into the Glasswood. Defeat its guardians and awaken the orchard heart in the southeast."
            }
            3 => "Enter the Drowned Archive. Defeat the keeper and recover the Crown of Names.",
            4 => {
                "Return the Crown of Names to Maelin in Hearthmere. Your decision will shape the Concord."
            }
            _ => {
                "The chapter is complete. Finish remaining side quests, refine your build, or enter the Concord rift east of Hearthmere for expeditions."
            }
        }
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct JournalEntry {
    pub id: &'static str,
    pub title: &'static str,
    pub kind: &'static str,
    pub status: &'static str,
    pub objective: String,
}
pub fn journal(c: &CampaignState) -> Vec<JournalEntry> {
    let mut entries = vec![
        JournalEntry {
            id: "ashen_concord",
            title: "The Ashen Concord",
            kind: "campaign",
            status: if c.main_quest == 3 {
                "complete"
            } else {
                "active"
            },
            objective: match c.main_quest {
                0 => "Speak with Maelin in Hearthmere.",
                1 => "Recover the Ashen Concord from the Forgotten Sanctum.",
                2 => "Return the Concord to Maelin.",
                _ => "Hearthmere's lamps burn again.",
            }
            .into(),
        },
        JournalEntry {
            id: "crown_of_names",
            title: "A Crown for the Nameless",
            kind: "campaign",
            status: if c.chronicle.chapter == 5 {
                "complete"
            } else if c.main_quest < 3 {
                "locked"
            } else {
                "active"
            },
            objective: c.chronicle.quest_text().into(),
        },
        JournalEntry {
            id: "satchel",
            title: "A Quartermaster's Burden",
            kind: "side",
            status: if c.flags.contains("satchel_returned") {
                "complete"
            } else {
                "active"
            },
            objective: if c.flags.contains("satchel_returned") {
                "Iven's satchel has been returned."
            } else if c.flags.contains("satchel_found") {
                "Return the satchel to Iven in Hearthmere."
            } else {
                "Find Iven's satchel on the southern Hollow Moor trail."
            }
            .into(),
        },
    ];
    for (id, title, found, returned, missing, carry, complete) in [
        (
            "bell",
            "The Last Ferry",
            "bell_found",
            "bell_returned",
            "Meet Ferryman Oren on the Ember Coast. Recover the bell beside the eastern shore.",
            "Return the ferry bell to Oren on the Ember Coast.",
            "Oren can guide the stranded home. The ferry bell rings again.",
        ),
        (
            "seed",
            "A Garden of Voices",
            "seed_found",
            "seed_returned",
            "Meet Cartographer Vey in the Glasswood. Recover the living seed in the southeast.",
            "Bring the living seed back to Vey in the Glasswood.",
            "Vey replanted the orchard. Its voices remember your kindness.",
        ),
        (
            "ledger",
            "Names Beneath the Water",
            "ledger_found",
            "ledger_returned",
            "Find Scholar Edda in the Drowned Archive. Recover the ledger from the northeast alcove.",
            "Return the ledger to Edda in the Drowned Archive.",
            "Edda has restored the drowned names. No one disappears from the record.",
        ),
    ] {
        let status = if c.chronicle.side_quests.contains(returned) {
            "complete"
        } else if c.chronicle.side_quests.contains(found) {
            "return"
        } else {
            "available"
        };
        entries.push(JournalEntry {
            id,
            title,
            kind: "side",
            status,
            objective: (if status == "complete" {
                complete
            } else if status == "return" {
                carry
            } else {
                missing
            })
            .into(),
        });
    }
    entries.push(JournalEntry { id: "expeditions", title: "The Shattered Concord", kind: "endgame", status: if c.chronicle.chapter < 5 { "locked" } else { "active" }, objective: c.chronicle.expedition.active.as_ref().map_or_else(|| "Complete A Crown for the Nameless, then enter Hearthmere's eastern rift. Each completed expedition opens a deeper fracture.".into(), ExpeditionRun::objective_text) });
    entries
}
#[derive(Debug, Clone, Serialize)]
pub struct RegionDefinition {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub connections: &'static [&'static str],
}
pub fn atlas() -> Vec<RegionDefinition> {
    [
        ("hearthmere", "Town, training, the Concord oath and the expedition rift.", &["hollow_moor", "ember_coast", "expedition"][..]),
        ("hollow_moor", "A wild crossing, lost satchel, shrine and forgotten stair.", &["hearthmere", "forgotten_sanctum"][..]),
        ("forgotten_sanctum", "The fallen keeper and the Ashen Concord.", &["hollow_moor"][..]),
        ("ember_coast", "A darkened beacon, a stranded ferryman and the burning shore.", &["hearthmere", "glasswood"][..]),
        ("glasswood", "An orchard of remembered voices and its imprisoned heart.", &["ember_coast", "drowned_archive"][..]),
        ("drowned_archive", "The flooded records, a surviving scholar and the Crown of Names.", &["glasswood"][..]),
        ("expedition", "Seeded fractures with hunt, seal and purge objectives; rising depth and recoverable loot.", &["hearthmere"][..]),
    ].into_iter().map(|(id, description, connections)| RegionDefinition { id, name: region_name(id), description, connections }).collect()
}
type PoiRow = (
    &'static str,
    &'static str,
    &'static str,
    i32,
    i32,
    Option<&'static str>,
);
pub(super) fn extra_points(region: &str) -> Vec<PoiRow> {
    match region {
        "hearthmere" => vec![
            (
                "vow_ward",
                "Oath: shelter the living",
                "choice",
                10_500,
                15_500,
                None,
            ),
            (
                "vow_release",
                "Oath: free the bound souls",
                "choice",
                14_500,
                15_500,
                None,
            ),
            (
                "coast_gate",
                "Northern Road: Ember Coast",
                "gate",
                43_500,
                7_500,
                Some("ember_coast"),
            ),
            ("rift", "The Concord Rift", "rift", 33_500, 18_500, None),
        ],
        "ember_coast" => vec![
            (
                "home_gate",
                "Road to Hearthmere",
                "gate",
                4_500,
                18_500,
                Some("hearthmere"),
            ),
            (
                "glasswood_gate",
                "Path to the Glasswood",
                "gate",
                43_500,
                18_500,
                Some("glasswood"),
            ),
            ("beacon", "The Ashen Beacon", "relic", 41_500, 7_500, None),
            ("ferryman", "Ferryman Oren", "person", 10_500, 27_500, None),
            ("bell", "The Last Ferry Bell", "quest", 33_500, 28_500, None),
            ("shrine", "Ember Sanctuary", "shrine", 7_500, 7_500, None),
            ("chest", "Coastguard's Cache", "chest", 23_500, 7_500, None),
        ],
        "glasswood" => vec![
            (
                "coast_gate",
                "Return to the Coast",
                "gate",
                4_500,
                18_500,
                Some("ember_coast"),
            ),
            (
                "archive_gate",
                "Stair to the Drowned Archive",
                "gate",
                43_500,
                18_500,
                Some("drowned_archive"),
            ),
            ("heart", "The Orchard Heart", "relic", 38_500, 28_500, None),
            (
                "cartographer",
                "Cartographer Vey",
                "person",
                8_500,
                7_500,
                None,
            ),
            (
                "seed",
                "The Last Living Seed",
                "quest",
                30_500,
                28_500,
                None,
            ),
            (
                "shrine",
                "The Listening Tree",
                "shrine",
                7_500,
                27_500,
                None,
            ),
            (
                "chest",
                "Orchard Keeper's Cache",
                "chest",
                22_500,
                6_500,
                None,
            ),
        ],
        "drowned_archive" => vec![
            (
                "glasswood_gate",
                "Return to the Glasswood",
                "gate",
                4_500,
                18_500,
                Some("glasswood"),
            ),
            ("crown", "The Crown of Names", "relic", 42_500, 18_500, None),
            ("scholar", "Scholar Edda", "person", 8_500, 27_500, None),
            ("ledger", "The Drowned Ledger", "quest", 33_500, 6_500, None),
            ("shrine", "The Dry Alcove", "shrine", 7_500, 7_500, None),
            (
                "chest",
                "Archivist's Strongbox",
                "chest",
                36_500,
                28_500,
                None,
            ),
        ],
        "expedition" => vec![
            (
                "exit",
                "Return to Hearthmere (progress preserved)",
                "gate",
                4_500,
                18_500,
                Some("hearthmere"),
            ),
            (
                "seal_north",
                "Northern Fracture Seal",
                "seal",
                24_500,
                5_500,
                None,
            ),
            (
                "seal_south",
                "Southern Fracture Seal",
                "seal",
                24_500,
                30_500,
                None,
            ),
            (
                "seal_east",
                "Eastern Fracture Seal",
                "seal",
                40_500,
                18_500,
                None,
            ),
            (
                "claim",
                "Concord Reward Cache",
                "chest",
                42_500,
                28_500,
                None,
            ),
        ],
        _ => vec![],
    }
}
pub fn campaign_points_of_interest(c: &CampaignState) -> Vec<PointOfInterest> {
    points_of_interest(&c.region)
        .into_iter()
        .filter(|p| {
            if p.id.starts_with("vow_") {
                return c.main_quest == 3 && c.chronicle.chapter == 0;
            }
            if p.id.starts_with("seal_") {
                return c
                    .chronicle
                    .expedition
                    .active
                    .as_ref()
                    .is_some_and(|run| run.objective == "seals");
            }
            true
        })
        .collect()
}
pub(super) fn is_chronicle_interaction(id: &str) -> bool {
    matches!(
        id,
        "vow_ward"
            | "vow_release"
            | "beacon"
            | "heart"
            | "crown"
            | "ferryman"
            | "bell"
            | "cartographer"
            | "seed"
            | "scholar"
            | "ledger"
            | "rift"
            | "seal_north"
            | "seal_south"
            | "seal_east"
            | "claim"
    )
}
pub(super) fn expedition_map(seed: u64) -> NavGrid {
    let mut map = NavGrid::new(48, 36);
    for x in 0..48 {
        map.block(x, 0);
        map.block(x, 35);
    }
    for z in 0..36 {
        map.block(0, z);
        map.block(47, z);
    }
    let walls = match seed % 3 {
        0 => [(14, 8, 18, 13), (28, 23, 32, 28)],
        1 => [(14, 23, 18, 28), (28, 8, 32, 13)],
        _ => [(20, 9, 24, 14), (20, 23, 24, 28)],
    };
    for (x0, z0, x1, z1) in walls {
        for z in z0..z1 {
            for x in x0..x1 {
                map.block(x, z);
            }
        }
    }
    map
}
pub(super) fn encounter(region: &str, chronicle: &ChronicleState) -> Vec<(i32, i32, bool)> {
    match region {
        "ember_coast" => vec![
            (17_500, 18_500, false),
            (22_500, 17_500, false),
            (27_500, 19_500, false),
            (32_500, 18_500, false),
            (36_500, 19_500, false),
            (38_500, 9_500, true),
        ],
        "glasswood" => vec![
            (18_500, 18_500, false),
            (23_500, 17_500, false),
            (28_500, 18_500, false),
            (33_500, 19_500, false),
            (36_500, 25_500, false),
            (40_500, 28_500, true),
        ],
        "drowned_archive" => vec![
            (20_500, 18_500, false),
            (24_500, 17_500, false),
            (31_500, 19_500, false),
            (34_500, 17_500, false),
            (37_500, 20_500, false),
            (39_500, 18_500, true),
        ],
        "expedition" => {
            let Some(run) = &chronicle.expedition.active else {
                return vec![];
            };
            let mut rows = vec![
                (16_500, 18_500, false),
                (20_500, 19_500, false),
                (25_500, 18_500, false),
                (29_500, 20_500, false),
                (34_500, 18_500, false),
                (38_500, 17_500, false),
                (33_500, 30_500, false),
                (24_500, 6_500, false),
            ];
            if run.seed & 1 != 0 {
                rows.push((12_500, 29_500, false));
                rows.push((38_500, 6_500, false));
            }
            rows.push((41_500, 19_500, true));
            rows
        }
        _ => vec![],
    }
}
pub(super) fn enemy_stats(kind: UnitKind, region: &str, chronicle: &ChronicleState) -> UnitStats {
    let mut stats = kind.stats();
    let depth = if region == "expedition" {
        chronicle.expedition.active.as_ref().map_or(0, |r| r.depth)
    } else {
        0
    };
    let scale = match region {
        "ember_coast" => 115,
        "glasswood" => 130,
        "drowned_archive" => 150,
        "expedition" => 150u64.saturating_add(u64::from(depth) * 15),
        _ => 100,
    };
    let scaled = |value: i32, percent: u64| -> i32 {
        ((value as u64).saturating_mul(percent) / 100).min(i32::MAX as u64) as i32
    };
    stats.hp = scaled(stats.hp, scale);
    stats.damage = scaled(
        stats.damage,
        if region == "expedition" {
            110 + u64::from(depth) * 5
        } else {
            100
        },
    );
    if region == "expedition"
        && let Some(run) = &chronicle.expedition.active
    {
        if run.modifier == "wrath" {
            stats.damage = scaled(stats.damage, 125);
        }
        if run.modifier == "haste" {
            stats.speed_per_tick = scaled(stats.speed_per_tick, 130);
        }
    }
    stats
}
impl World {
    pub(super) fn validate_chronicle_travel(&self, region: &str) -> Result<(), CommandError> {
        let c = self.campaign.as_ref().expect("campaign");
        let required = match region {
            "ember_coast" => 1,
            "glasswood" => 2,
            "drowned_archive" => 3,
            _ => 0,
        };
        if c.chronicle.chapter < required {
            return Err(err(match region {
                "ember_coast" => {
                    "Return the Concord, then choose its fate at Maelin's oath stones."
                }
                "glasswood" => {
                    "Rekindle the Ember Coast beacon before crossing into the Glasswood."
                }
                _ => "Awaken the Orchard Heart before descending into the archive.",
            }));
        }
        Ok(())
    }
    pub(super) fn interact_chronicle(&mut self, target: &str) -> Result<(), CommandError> {
        if target == "rift" {
            return self.enter_expedition();
        }
        let enemies_alive = self.units.iter().any(|u| u.team == Team::Enemy && u.hp > 0);
        let nearby_enemies = if target.starts_with("seal_") {
            let p = points_of_interest("expedition")
                .into_iter()
                .find(|p| p.id == target)
                .expect("seal")
                .position;
            self.units.iter().any(|u| {
                u.team == Team::Enemy
                    && u.hp > 0
                    && u.position.distance_squared(p) < 6_000u64.pow(2)
            })
        } else {
            false
        };
        let c = self.campaign.as_mut().expect("campaign");
        match target {
            "vow_ward" | "vow_release" => {
                if c.main_quest != 3 || c.chronicle.chapter != 0 {
                    return Err(err(
                        "This oath can be made once, after returning the Ashen Concord.",
                    ));
                }
                c.chronicle.choice = if target == "vow_ward" {
                    "ward"
                } else {
                    "release"
                }
                .into();
                c.chronicle.chapter = 1;
                c.dialogue=if target=="vow_ward" {"You bind the Concord to shelter the living. Its ward will protect the opening moments of every hostile crossing. Oren's stranded families will find a safe shore. Take the northern road to the Ember Coast."} else {"You release the bound souls. Their gratitude will refill your class resource whenever you enter hostile country. The orchard's imprisoned voices will remember this freedom. Take the northern road to the Ember Coast."}.into();
            }
            "beacon" | "heart" | "crown" => {
                let chapter = match target {
                    "beacon" => 1,
                    "heart" => 2,
                    _ => 3,
                };
                if c.chronicle.chapter > chapter {
                    return Err(err("This chapter objective is already complete."));
                }
                if c.chronicle.chapter != chapter {
                    return Err(err(
                        "Follow the current chapter objective before claiming this relic.",
                    ));
                }
                if enemies_alive {
                    return Err(err(
                        "Defeat the region's guardians before completing this objective.",
                    ));
                }
                c.chronicle.chapter += 1;
                c.gain_experience(220 + chapter * 60);
                c.dialogue=match target {"beacon"=>"The beacon burns again. Oren's ferry can find the shore. The eastern road now leads into the Glasswood; its heart is still imprisoned.","heart"=>if c.chronicle.choice=="release" {"The orchard recognizes the freed souls and opens willingly. Beyond it, the drowned keep the names of those Hearthmere forgot. Descend into the archive."} else {"Your ward shelters the orchard's frightened voices. Its heart yields the road to the Drowned Archive. Seek the names the flood concealed."},_=>"The Crown of Names rises from the flood. Every lost name burns along its rim. Return to Maelin in Hearthmere, by the same roads you restored."}.into();
            }
            "bell" | "seed" | "ledger" => {
                let flag = format!("{target}_found");
                if !c.chronicle.side_quests.insert(flag) {
                    return Err(err("You already recovered this quest item."));
                }
                c.dialogue=match target {"bell"=>"You recover the ferry bell. Return it to Oren beside the coast's southern shore.","seed"=>"A living seed whispers beneath the glass. Carry it to Vey in the northwest orchard.",_=>"The ledger survived in an air pocket. Return it to Edda in the southwest archive."}.into();
            }
            "ferryman" | "cartographer" | "scholar" => {
                let (id, item, power, request, thanks) = match target {
                    "ferryman" => (
                        "bell",
                        "Oren's Shoreward Token",
                        28,
                        "Oren: Without my bell the ferry loses its way. It fell beside the eastern shore. Bring it back and the stranded can cross.",
                        "Oren: The bell! I will bring every last family home. Keep this token; the shore will remember you.",
                    ),
                    "cartographer" => (
                        "seed",
                        "Vey's Verdant Lens",
                        32,
                        "Vey: One living seed remains beneath the southeastern glass. Bring it to me; these trees deserve another spring.",
                        "Vey: A garden can begin with one act of kindness. Take this lens. It sees the paths that grief conceals.",
                    ),
                    _ => (
                        "ledger",
                        "Edda's Remembered Seal",
                        38,
                        "Edda: The crown keeps names, but the ledger keeps their lives. Find it in the northeast alcove before the water erases it.",
                        "Edda: They were people, not entries. Now their stories survive. Carry this seal in their memory.",
                    ),
                };
                let returned = format!("{id}_returned");
                if c.chronicle.side_quests.contains(&returned) {
                    c.dialogue = thanks.into();
                } else if c.chronicle.side_quests.contains(&format!("{id}_found")) {
                    let reward = c.new_item(item, "charm", power, "rare")?;
                    c.inventory.push(reward);
                    c.chronicle.side_quests.insert(returned);
                    c.gain_experience(180);
                    self.economy.essence = self.economy.essence.saturating_add(70);
                    c.dialogue = thanks.into();
                } else {
                    c.dialogue = request.into();
                }
            }
            "seal_north" | "seal_south" | "seal_east" => {
                let run = c
                    .chronicle
                    .expedition
                    .active
                    .as_mut()
                    .ok_or_else(|| err("No expedition is active."))?;
                if run.objective != "seals" || run.reward_claimed {
                    return Err(err("There is no active seal objective here."));
                }
                if nearby_enemies {
                    return Err(err(
                        "Clear the enemies guarding this seal before touching it.",
                    ));
                }
                if !run.seals.insert(target.into()) {
                    return Err(err("This fracture seal is already closed."));
                }
                c.dialogue = run.objective_text();
            }
            "claim" => {
                let run = c
                    .chronicle
                    .expedition
                    .active
                    .as_ref()
                    .ok_or_else(|| err("No expedition is active."))?;
                if run.reward_claimed {
                    return Err(err("This expedition's reward has already been claimed."));
                }
                if !run.objective_complete() {
                    return Err(err(run.objective_text()));
                }
                let depth = run.depth;
                let power = 30u64 + u64::from(depth) * 4;
                let reward = c.new_item(
                    &format!("Concord Relic · Depth {depth}"),
                    if depth.is_multiple_of(2) {
                        "charm"
                    } else {
                        "weapon"
                    },
                    power.min(i32::MAX as u64) as i32,
                    if depth.is_multiple_of(3) {
                        "legendary"
                    } else {
                        "rare"
                    },
                )?;
                c.inventory.push(reward);
                c.inventory.extend(c.ground_loot.drain(..).map(|l| l.item));
                c.gain_experience(160u32.saturating_add(depth.saturating_mul(35)));
                self.economy.essence = self
                    .economy
                    .essence
                    .saturating_add(60u32.saturating_add(depth.saturating_mul(10)));
                c.chronicle.expedition.completed =
                    c.chronicle.expedition.completed.saturating_add(1);
                c.chronicle.expedition.best_depth = c.chronicle.expedition.best_depth.max(depth);
                c.chronicle
                    .expedition
                    .active
                    .as_mut()
                    .expect("run")
                    .reward_claimed = true;
                c.dialogue = format!(
                    "Depth {depth} complete. Your relic and every uncollected drop are safely in your inventory. Return west to Hearthmere; the next fracture will be deeper."
                );
            }
            _ => return Err(err("Unknown chapter interaction.")),
        }
        Ok(())
    }
    fn enter_expedition(&mut self) -> Result<(), CommandError> {
        let c = self.campaign.as_ref().expect("campaign");
        if c.chronicle.chapter < 5 {
            return Err(err(
                "Bring the Crown of Names to Maelin to open the Concord rift.",
            ));
        }
        if c.chronicle
            .expedition
            .active
            .as_ref()
            .is_some_and(|r| !r.reward_claimed)
        {
            return self.transition_campaign_region("expedition");
        }
        let depth = c
            .chronicle
            .expedition
            .best_depth
            .checked_add(1)
            .ok_or_else(|| err("The expedition depth counter is exhausted."))?;
        let attempt = c
            .chronicle
            .expedition
            .attempts
            .checked_add(1)
            .ok_or_else(|| err("The expedition attempt counter is exhausted."))?;
        // Check allocations before changing saved encounter state or consuming deterministic RNG.
        self.next_id
            .checked_add(11)
            .ok_or(CommandError::IdExhausted)?;
        // Avalanche the layout seed; simple XOR/modulo produced identical modifiers
        // for all first three expeditions of the original acceptance seed.
        let mut seed = self.rng ^ u64::from(attempt).wrapping_mul(0x9e3779b97f4a7c15);
        seed = (seed ^ (seed >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        seed = (seed ^ (seed >> 27)).wrapping_mul(0x94d049bb133111eb);
        seed ^= seed >> 31;
        let ordinal = u64::from(attempt - 1);
        let objective = ["hunt", "seals", "purge"][(ordinal % 3) as usize];
        // Every group of three has all modifiers. Rotate later groups so objectives
        // are not permanently paired with one modifier; the world seed sets the phase.
        let modifier =
            ["wrath", "haste", "drought"][((ordinal + ordinal / 3 + self.rng % 3) % 3) as usize];
        let c = self.campaign.as_mut().expect("campaign");
        c.chronicle.expedition.attempts = attempt;
        c.chronicle.expedition.active = Some(ExpeditionRun {
            depth,
            seed,
            objective: objective.into(),
            modifier: modifier.into(),
            kills: 0,
            enemies_total: if seed & 1 != 0 { 11 } else { 9 },
            warden_id: 0,
            warden_defeated: false,
            seals: BTreeSet::new(),
            reward_claimed: false,
        });
        if let Some(previous) = c.regions.remove("expedition") {
            // Drops acquired after an early hunt reward must also survive a new run.
            c.inventory
                .extend(previous.loot.into_iter().map(|loot| loot.item));
        }
        c.visited.remove("expedition");
        self.transition_campaign_region("expedition")?;
        let c = self.campaign.as_mut().expect("campaign");
        c.dialogue = format!(
            "Concord depth {depth} · {modifier}. {} You can return to Hearthmere and resume this exact encounter.",
            c.chronicle
                .expedition
                .active
                .as_ref()
                .expect("run")
                .objective_text()
        );
        Ok(())
    }
    pub(super) fn validate_chronicle(&self) -> Result<(), SaveError> {
        let c = self.campaign.as_ref().expect("campaign");
        let ch = &c.chronicle;
        let e = &ch.expedition;
        let fail = || SaveError("Invalid chapter, oath or expedition progression.".into());
        if ch.version != 1
            || ch.chapter > 5
            || (ch.chapter == 0 && !ch.choice.is_empty())
            || (ch.chapter > 0
                && (!matches!(ch.choice.as_str(), "ward" | "release") || c.main_quest != 3))
        {
            return Err(fail());
        }
        if ch.side_quests.iter().any(|s| {
            ![
                "bell_found",
                "bell_returned",
                "seed_found",
                "seed_returned",
                "ledger_found",
                "ledger_returned",
            ]
            .contains(&s.as_str())
        }) {
            return Err(fail());
        }
        for id in ["bell", "seed", "ledger"] {
            if ch.side_quests.contains(&format!("{id}_returned"))
                && !ch.side_quests.contains(&format!("{id}_found"))
            {
                return Err(fail());
            }
        }
        if e.completed > e.attempts
            || e.best_depth != e.completed
            || (e.attempts == 0) != (e.active.is_none())
            || (e.attempts > 0 && ch.chapter != 5)
        {
            return Err(fail());
        }
        if c.visited.contains("expedition") != e.active.is_some() {
            return Err(fail());
        }
        for (region, chapter) in [("ember_coast", 1), ("glasswood", 2), ("drowned_archive", 3)] {
            if c.visited.contains(region) && ch.chapter < chapter {
                return Err(fail());
            }
        }
        if let Some(run) = &e.active {
            if run.depth == 0
                || !["hunt", "seals", "purge"].contains(&run.objective.as_str())
                || !["wrath", "haste", "drought"].contains(&run.modifier.as_str())
                || run.kills > run.enemies_total
                || run.enemies_total != encounter("expedition", ch).len() as u32
                || run.warden_id == 0
                || run.warden_id >= self.next_id
                || run
                    .seals
                    .iter()
                    .any(|s| !["seal_north", "seal_south", "seal_east"].contains(&s.as_str()))
                || (run.objective != "seals" && !run.seals.is_empty())
            {
                return Err(fail());
            }
            if (run.reward_claimed
                && (!run.objective_complete()
                    || run.depth != e.best_depth
                    || e.attempts != e.completed))
                || (!run.reward_claimed
                    && (run.depth != e.best_depth.saturating_add(1)
                        || e.attempts != e.completed.saturating_add(1)))
            {
                return Err(fail());
            }
            let enemies: Vec<_> = if c.region == "expedition" {
                self.units
                    .iter()
                    .filter(|u| u.team == Team::Enemy)
                    .collect()
            } else {
                c.regions
                    .get("expedition")
                    .ok_or_else(fail)?
                    .enemies
                    .iter()
                    .collect()
            };
            if enemies.len() as u32 + run.kills != run.enemies_total
                || enemies.iter().any(|u| u.id == run.warden_id) == run.warden_defeated
            {
                return Err(fail());
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn make(class_id: &str) -> World {
        World::new_campaign(
            20261125,
            CharacterCreation {
                class_id: class_id.into(),
                ..Default::default()
            },
        )
        .unwrap()
    }
    fn at(world: &mut World, poi: &str) {
        let c = world.campaign.as_ref().unwrap();
        let p = points_of_interest(&c.region)
            .into_iter()
            .find(|p| p.id == poi)
            .unwrap()
            .position;
        let h = world
            .units
            .iter_mut()
            .find(|u| u.kind == UnitKind::Commander)
            .unwrap();
        h.position = p;
        h.path.clear();
        h.path_goal = None;
        h.target = None;
        h.order = Order::Idle;
    }
    fn expedition_ready() -> World {
        let mut w = make("paladin");
        let c = w.campaign.as_mut().unwrap();
        c.main_quest = 3;
        c.chronicle.chapter = 5;
        c.chronicle.choice = "ward".into();
        at(&mut w, "rift");
        w
    }
    fn defeat_all(w: &mut World) {
        for u in &mut w.units {
            if u.team == Team::Enemy {
                u.hp = 0;
            }
        }
        w.resolve_deaths();
    }
    #[test]
    fn class_resource_identity_and_limits_are_bound_to_ids() {
        for (id, name) in [
            ("warrior", "Resolve"),
            ("barbarian", "Fury"),
            ("paladin", "Conviction"),
            ("crusader", "Wrath"),
            ("amazon", "Vigor"),
            ("rogue", "Energy"),
            ("assassin", "Momentum"),
            ("demon_hunter", "Hatred"),
            ("monk", "Spirit"),
            ("sorcerer", "Mana"),
            ("wizard", "Arcane Power"),
            ("necromancer", "Essence"),
            ("witch_doctor", "Mana"),
            ("druid", "Primal Spirit"),
            ("spiritborn", "Vigor"),
            ("blood_knight", "Blood"),
            ("tempest", "Flow"),
            ("warlock", "Pact"),
            ("bard", "Resonance"),
        ] {
            let w = make(id);
            let c = w.campaign.as_ref().unwrap();
            let r = &c.class().resource;
            assert_eq!(
                r.name, name,
                "Resource belongs to the class identity, never its array index"
            );
            assert_eq!(c.focus, r.maximum);
            assert!(r.maximum >= 100 && r.idle_regen_ticks > 0 && r.combat_regen_ticks > 0);
            w.validate().unwrap();
        }
    }
    #[test]
    fn resources_recover_differently_and_kills_return_authoritative_power() {
        let mut barbarian = make("barbarian");
        let mut wizard = make("wizard");
        barbarian.campaign.as_mut().unwrap().focus = 0;
        wizard.campaign.as_mut().unwrap().focus = 0;
        for _ in 0..30 {
            barbarian.step();
            wizard.step();
        }
        assert_eq!(barbarian.campaign.as_ref().unwrap().focus, 1);
        assert_eq!(wizard.campaign.as_ref().unwrap().focus, 6);
        let p = barbarian.commander().unwrap().position;
        barbarian
            .spawn(UnitKind::Reaver, Team::Enemy, Point::new(p.x + 1000, p.z))
            .unwrap();
        let before = barbarian.campaign.as_ref().unwrap().focus;
        defeat_all(&mut barbarian);
        assert_eq!(barbarian.campaign.as_ref().unwrap().focus, before + 12);
        barbarian.validate().unwrap();
    }
    #[test]
    fn legacy_v2_saves_migrate_additively_and_invalid_extensions_fail() {
        let w = make("sorcerer");
        let mut value: serde_json::Value = serde_json::from_str(&w.save_json().unwrap()).unwrap();
        value["campaign"]
            .as_object_mut()
            .unwrap()
            .remove("chronicle");
        value["campaign"]["focus"] = 100.into();
        let loaded = World::load_json(&value.to_string()).unwrap();
        assert_eq!(loaded.campaign.as_ref().unwrap().chronicle.chapter, 0);
        assert_eq!(loaded.campaign.as_ref().unwrap().focus, 100);
        let exact = loaded.save_json().unwrap();
        assert_eq!(
            World::load_json(&exact).unwrap().save_json().unwrap(),
            exact
        );
        value["campaign"]["chronicle"] = serde_json::to_value(ChronicleState {
            chapter: 5,
            ..Default::default()
        })
        .unwrap();
        assert!(World::load_json(&value.to_string()).is_err());
    }
    #[test]
    fn oath_is_irreversible_and_locked_roads_are_atomic() {
        let mut w = make("paladin");
        at(&mut w, "coast_gate");
        let before = w.save_json().unwrap();
        assert!(w.travel("ember_coast").is_err());
        assert_eq!(w.save_json().unwrap(), before);
        w.campaign.as_mut().unwrap().main_quest = 3;
        at(&mut w, "vow_release");
        w.interact("vow_release").unwrap();
        at(&mut w, "vow_ward");
        let before = w.save_json().unwrap();
        assert!(w.interact("vow_ward").is_err());
        assert_eq!(w.save_json().unwrap(), before);
        at(&mut w, "coast_gate");
        w.campaign.as_mut().unwrap().focus = 0;
        w.travel("ember_coast").unwrap();
        let c = w.campaign.as_ref().unwrap();
        assert_eq!(c.focus, c.class().resource.maximum);
        assert_eq!(c.chronicle.choice, "release");
        w.validate().unwrap();
    }
    #[test]
    fn every_authored_poi_is_reachable_and_scaled_saves_roundtrip() {
        let mut w = make("paladin");
        let c = w.campaign.as_mut().unwrap();
        c.main_quest = 3;
        c.chronicle.chapter = 5;
        c.chronicle.choice = "ward".into();
        for region in [
            "hearthmere",
            "hollow_moor",
            "forgotten_sanctum",
            "ember_coast",
            "glasswood",
            "drowned_archive",
        ] {
            w.transition_campaign_region(region).unwrap();
            for p in points_of_interest(region) {
                assert!(w.map.contains_point(p.position), "{region}:{}", p.id);
                let origin = w.commander().unwrap().position;
                assert!(
                    !w.map.path(origin, p.position).is_empty()
                        || origin.distance_squared(p.position) < 100u64.pow(2),
                    "Unreachable {region}:{}",
                    p.id
                );
            }
            let saved = w.save_json().unwrap();
            assert_eq!(
                World::load_json(&saved).unwrap().save_json().unwrap(),
                saved
            );
        }
    }
    #[test]
    fn expeditions_resume_scale_vary_and_reward_exactly_once() {
        let mut w = expedition_ready();
        let mut objectives = BTreeSet::new();
        let mut modifiers = BTreeSet::new();
        for depth in 1..=3 {
            at(&mut w, "rift");
            w.interact("rift").unwrap();
            w.validate().unwrap();
            let run = w
                .campaign
                .as_ref()
                .unwrap()
                .chronicle
                .expedition
                .active
                .as_ref()
                .unwrap();
            assert_eq!(run.depth, depth);
            objectives.insert(run.objective.clone());
            modifiers.insert(run.modifier.clone());
            assert!(
                w.units
                    .iter()
                    .any(|u| u.team == Team::Enemy && u.max_hp > u.kind.stats().hp)
            );
            let before = w.save_json().unwrap();
            at(&mut w, "claim");
            let unearned = w.save_json().unwrap();
            assert!(w.interact("claim").is_err());
            assert_eq!(w.save_json().unwrap(), unearned);
            w = World::load_json(&before).unwrap();
            let enemies = w
                .units
                .iter()
                .filter(|u| u.team == Team::Enemy)
                .cloned()
                .collect::<Vec<_>>();
            at(&mut w, "exit");
            w.interact("exit").unwrap();
            let save = w.save_json().unwrap();
            w = World::load_json(&save).unwrap();
            at(&mut w, "rift");
            w.interact("rift").unwrap();
            assert_eq!(
                serde_json::to_string(&enemies).unwrap(),
                serde_json::to_string(
                    &w.units
                        .iter()
                        .filter(|u| u.team == Team::Enemy)
                        .cloned()
                        .collect::<Vec<_>>()
                )
                .unwrap()
            );
            defeat_all(&mut w);
            if depth == 2 {
                for id in ["seal_north", "seal_south", "seal_east"] {
                    at(&mut w, id);
                    w.interact(id).unwrap();
                }
            }
            at(&mut w, "claim");
            let c = w.campaign.as_mut().unwrap();
            let next = c.next_item_id;
            c.next_item_id = u32::MAX;
            let exhausted = w.save_json().unwrap();
            assert!(w.interact("claim").is_err());
            assert_eq!(w.save_json().unwrap(), exhausted);
            w.campaign.as_mut().unwrap().next_item_id = next;
            w.interact("claim").unwrap();
            assert!(w.campaign.as_ref().unwrap().ground_loot.is_empty());
            let rewarded = w.save_json().unwrap();
            assert!(w.interact("claim").is_err());
            assert_eq!(w.save_json().unwrap(), rewarded);
            w.validate().unwrap();
            assert_eq!(
                World::load_json(&rewarded).unwrap().save_json().unwrap(),
                rewarded
            );
            at(&mut w, "exit");
            w.interact("exit").unwrap();
        }
        assert_eq!(objectives.len(), 3);
        assert_eq!(modifiers.len(), 3);
        assert_eq!(
            w.campaign.as_ref().unwrap().chronicle.expedition.completed,
            3
        );
    }
    #[test]
    fn failed_expedition_allocation_preserves_world_and_rng() {
        let mut w = expedition_ready();
        w.next_id = u32::MAX;
        let before = w.save_json().unwrap();
        assert!(w.interact("rift").is_err());
        assert_eq!(w.save_json().unwrap(), before);
    }
    #[test]
    fn early_hunt_reward_then_later_drops_survive_the_next_expedition() {
        let mut w = expedition_ready();
        w.interact("rift").unwrap();
        let warden = w
            .campaign
            .as_ref()
            .unwrap()
            .chronicle
            .expedition
            .active
            .as_ref()
            .unwrap()
            .warden_id;
        w.units.iter_mut().find(|u| u.id == warden).unwrap().hp = 0;
        w.resolve_deaths();
        assert!(w.units.iter().any(|u| u.team == Team::Enemy));
        at(&mut w, "claim");
        w.interact("claim").unwrap();
        w.validate().unwrap();
        w.units
            .iter_mut()
            .find(|u| u.team == Team::Enemy)
            .unwrap()
            .hp = 0;
        w.resolve_deaths();
        let drop = w.campaign.as_ref().unwrap().ground_loot[0].item.id;
        at(&mut w, "exit");
        w.interact("exit").unwrap();
        let save = w.save_json().unwrap();
        w = World::load_json(&save).unwrap();
        at(&mut w, "rift");
        w.interact("rift").unwrap();
        let c = w.campaign.as_ref().unwrap();
        assert!(c.inventory.iter().any(|i| i.id == drop));
        assert_eq!(c.chronicle.expedition.active.as_ref().unwrap().depth, 2);
        w.validate().unwrap();
    }
    #[test]
    fn oath_choices_have_distinct_crossing_effects() {
        for choice in ["ward", "release"] {
            let mut w = make("paladin");
            let c = w.campaign.as_mut().unwrap();
            c.main_quest = 3;
            c.chronicle.chapter = 1;
            c.chronicle.choice = choice.into();
            c.focus = 0;
            c.ward_ticks = 0;
            at(&mut w, "coast_gate");
            w.travel("ember_coast").unwrap();
            let c = w.campaign.as_ref().unwrap();
            assert_eq!(c.ward_ticks, if choice == "ward" { 180 } else { 0 });
            assert_eq!(
                c.focus,
                if choice == "release" {
                    c.class().resource.maximum
                } else {
                    0
                }
            );
        }
    }
    #[test]
    fn numeric_level_exhaustion_preserves_xp_saves_retraining_and_learning() {
        let mut w = make("paladin");
        let mastery = class("paladin").unwrap().skills[1].id.clone();
        let c = w.campaign.as_mut().unwrap();
        c.level = u32::MAX;
        c.learned.insert(mastery.clone());
        c.skill_points = u64::from(u32::MAX);
        c.experience = u32::MAX - 1;
        w.validate().unwrap();
        w.campaign.as_mut().unwrap().gain_experience(45);
        assert_eq!(w.campaign.as_ref().unwrap().experience, u32::MAX);
        let saved = w.save_json().unwrap();
        w = World::load_json(&saved).unwrap();
        assert_eq!(w.save_json().unwrap(), saved);
        w.respec().unwrap();
        assert_eq!(
            w.campaign.as_ref().unwrap().skill_points,
            u64::from(u32::MAX) + 1
        );
        w.validate().unwrap();
        w.learn_skill(&mastery).unwrap();
        w.validate().unwrap();
        w.campaign.as_mut().unwrap().skill_points = u64::MAX;
        let malicious = w.save_json().unwrap();
        assert!(World::load_json(&malicious).is_err());
    }
    #[test]
    fn modifiers_change_actual_damage_speed_and_resource_recovery() {
        let mut w = expedition_ready();
        w.interact("rift").unwrap();
        let mut results = BTreeMap::new();
        for modifier in ["wrath", "haste", "drought"] {
            let mut trial = w.clone();
            trial
                .campaign
                .as_mut()
                .unwrap()
                .chronicle
                .expedition
                .active
                .as_mut()
                .unwrap()
                .modifier = modifier.into();
            let enemy = trial.units.iter().find(|u| u.team == Team::Enemy).unwrap();
            let stats = trial.stats_for(enemy);
            defeat_all(&mut trial);
            trial.campaign.as_mut().unwrap().focus = 0;
            for _ in 0..60 {
                trial.step();
            }
            trial.validate().unwrap();
            results.insert(
                modifier,
                (
                    stats.damage,
                    stats.speed_per_tick,
                    trial.campaign.as_ref().unwrap().focus,
                ),
            );
        }
        assert!(results["wrath"].0 > results["haste"].0);
        assert!(results["haste"].1 > results["wrath"].1);
        assert_eq!(results["drought"].2 * 2, results["wrath"].2);
    }
}
