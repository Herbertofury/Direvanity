//! Character-led campaign rules. Presentation never owns progression or save state.
use super::*;
use std::collections::BTreeMap;
use std::sync::OnceLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillDefinition {
    pub id: String,
    pub name: String,
    pub branch: String,
    pub tier: u32,
    pub minimum_level: u32,
    pub requires: Option<String>,
    pub kind: String,
    pub effect: String,
    pub power: i32,
    pub range: i32,
    pub radius: i32,
    pub cooldown_ticks: u32,
    pub focus_cost: u32,
    pub description: String,
    pub adaptation_status: String,
    pub source_scope: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassDefinition {
    pub id: String,
    pub name: String,
    pub aliases: Vec<String>,
    pub description: String,
    pub color: String,
    pub combat_style: String,
    pub branches: Vec<String>,
    pub skills: Vec<SkillDefinition>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Catalogue {
    pub schema: u32,
    pub design: String,
    pub classes: Vec<ClassDefinition>,
}
pub fn catalogue() -> &'static Catalogue {
    static DATA: OnceLock<Catalogue> = OnceLock::new();
    DATA.get_or_init(|| {
        serde_json::from_str(include_str!("../../../data/gameplay/classes.json"))
            .expect("the checked-in class catalogue is valid")
    })
}
pub fn class(id: &str) -> Option<&'static ClassDefinition> {
    catalogue().classes.iter().find(|c| c.id == id)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CharacterCreation {
    pub name: String,
    pub class_id: String,
    pub body: String,
    pub skin: String,
    pub hair: String,
    pub hair_style: u32,
    pub eyes: String,
    pub accent: String,
    pub stature: u32,
}
impl Default for CharacterCreation {
    fn default() -> Self {
        Self {
            name: "Wanderer".into(),
            class_id: "paladin".into(),
            body: "athletic".into(),
            skin: "#c69b7b".into(),
            hair: "#2d2521".into(),
            hair_style: 0,
            eyes: "#81b7b0".into(),
            accent: "#d9b66c".into(),
            stature: 100,
        }
    }
}
impl CharacterCreation {
    pub fn validate(&self) -> Result<(), String> {
        let name = self.name.trim();
        if name.is_empty() || name.chars().count() > 32 || name.chars().any(char::is_control) {
            return Err("Choose a name of 1 to 32 printable characters.".into());
        }
        if class(&self.class_id).is_none() {
            return Err("Choose a known class.".into());
        }
        if !["athletic", "slender", "stocky"].contains(&self.body.as_str()) {
            return Err("Choose an available body silhouette.".into());
        }
        if self.hair_style > 5 || !(85..=115).contains(&self.stature) {
            return Err("Appearance values are outside the creator's supported range.".into());
        }
        for color in [&self.skin, &self.hair, &self.eyes, &self.accent] {
            if color.len() != 7
                || !color.starts_with('#')
                || !color[1..].bytes().all(|b| b.is_ascii_hexdigit())
            {
                return Err("Appearance colors must be six-digit hexadecimal colors.".into());
            }
        }
        Ok(())
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: u32,
    pub name: String,
    pub slot: String,
    pub power: i32,
    pub rarity: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroundLoot {
    pub item: Item,
    pub position: Point,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusEffect {
    pub target: UnitId,
    pub remaining: u32,
    pub periodic_damage: i32,
    pub slow: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionMemory {
    pub enemies: Vec<Unit>,
    pub loot: Vec<GroundLoot>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignState {
    pub character: CharacterCreation,
    pub region: String,
    pub level: u32,
    pub experience: u32,
    pub skill_points: u32,
    pub learned: BTreeSet<String>,
    pub cooldowns: BTreeMap<String, u32>,
    pub focus: u32,
    pub ward_ticks: u32,
    pub effects: Vec<StatusEffect>,
    pub inventory: Vec<Item>,
    pub equipped: BTreeMap<String, u32>,
    pub ground_loot: Vec<GroundLoot>,
    pub next_item_id: u32,
    pub regions: BTreeMap<String, RegionMemory>,
    pub visited: BTreeSet<String>,
    pub flags: BTreeSet<String>,
    pub main_quest: u32,
    pub dialogue: String,
}
#[derive(Debug, Clone, Serialize)]
pub struct PointOfInterest {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub position: Point,
    pub destination: Option<String>,
}
pub fn region_name(id: &str) -> &'static str {
    match id {
        "hearthmere" => "Hearthmere",
        "hollow_moor" => "The Hollow Moor",
        "forgotten_sanctum" => "The Forgotten Sanctum",
        _ => "Unknown",
    }
}
pub fn points_of_interest(region: &str) -> Vec<PointOfInterest> {
    let rows = match region {
        "hearthmere" => vec![
            ("elder", "Archivist Maelin", "person", 11_500, 18_500, None),
            (
                "merchant",
                "Quartermaster Iven",
                "person",
                12_500,
                23_500,
                None,
            ),
            ("chest", "Wayfarer's Cache", "chest", 7_500, 25_500, None),
            (
                "moor_gate",
                "Road to the Hollow Moor",
                "gate",
                43_500,
                18_500,
                Some("hollow_moor"),
            ),
        ],
        "hollow_moor" => vec![
            (
                "home_gate",
                "Road to Hearthmere",
                "gate",
                4_500,
                18_500,
                Some("hearthmere"),
            ),
            (
                "sanctum_gate",
                "Stair of the Forgotten",
                "gate",
                43_500,
                18_500,
                Some("forgotten_sanctum"),
            ),
            (
                "satchel",
                "Iven's Lost Satchel",
                "chest",
                18_500,
                29_500,
                None,
            ),
            ("shrine", "Wayside Shrine", "shrine", 25_500, 7_500, None),
        ],
        "forgotten_sanctum" => vec![
            (
                "moor_gate",
                "Return to the Moor",
                "gate",
                4_500,
                18_500,
                Some("hollow_moor"),
            ),
            ("relic", "The Ashen Concord", "relic", 42_500, 18_500, None),
            ("chest", "Reliquary", "chest", 26_500, 6_500, None),
        ],
        _ => vec![],
    };
    rows.into_iter()
        .map(|(id, name, kind, x, z, destination)| PointOfInterest {
            id: id.into(),
            name: name.into(),
            kind: kind.into(),
            position: Point::new(x, z),
            destination: destination.map(str::to_owned),
        })
        .collect()
}
fn err(message: impl Into<String>) -> CommandError {
    CommandError::Roleplay(message.into())
}
impl CampaignState {
    pub fn class(&self) -> &'static ClassDefinition {
        class(&self.character.class_id).expect("validated class")
    }
    pub fn next_level_experience(&self) -> u32 {
        100u32.saturating_add(self.level.saturating_sub(1).saturating_mul(40))
    }
    pub fn quest_text(&self) -> &'static str {
        match self.main_quest {
            0 => "Speak with Archivist Maelin in Hearthmere.",
            1 => {
                "Cross the Hollow Moor. Enter the Forgotten Sanctum and recover the Ashen Concord."
            }
            2 => "Return the Ashen Concord to Archivist Maelin in Hearthmere.",
            _ => "Prologue complete. Explore, recover lost caches and develop your character.",
        }
    }
    fn gain_experience(&mut self, amount: u32) {
        self.experience = self.experience.saturating_add(amount);
        while self.experience >= self.next_level_experience() && self.level < u32::MAX {
            self.experience -= self.next_level_experience();
            self.level += 1;
            self.skill_points = self.skill_points.saturating_add(1);
        }
    }
    fn new_item(
        &mut self,
        name: &str,
        slot: &str,
        power: i32,
        rarity: &str,
    ) -> Result<Item, CommandError> {
        let id = self.next_item_id;
        self.next_item_id = self
            .next_item_id
            .checked_add(1)
            .ok_or_else(|| err("Item identity space exhausted."))?;
        Ok(Item {
            id,
            name: name.into(),
            slot: slot.into(),
            power,
            rarity: rarity.into(),
        })
    }
}
impl World {
    pub fn new_campaign(seed: u64, mut creation: CharacterCreation) -> Result<Self, SaveError> {
        creation.validate().map_err(SaveError)?;
        creation.name = creation.name.trim().into();
        let starter = class(&creation.class_id).expect("validated class").skills[0]
            .id
            .clone();
        let mut world = World::new(seed);
        world.version = 2;
        world.units.retain(|u| u.kind == UnitKind::Commander);
        world.objectives.clear();
        world.wave = 0;
        world.wave_ticks_remaining = 0;
        world.economy = Economy {
            essence: 40,
            kills: 0,
        };
        world.campaign = Some(CampaignState {
            character: creation,
            region: "hearthmere".into(),
            level: 1,
            experience: 0,
            skill_points: 2,
            learned: BTreeSet::from([starter]),
            cooldowns: BTreeMap::new(),
            focus: 100,
            ward_ticks: 0,
            effects: vec![],
            inventory: vec![],
            equipped: BTreeMap::new(),
            ground_loot: vec![],
            next_item_id: 1,
            regions: BTreeMap::new(),
            visited: BTreeSet::from(["hearthmere".into()]),
            flags: BTreeSet::new(),
            main_quest: 0,
            dialogue: "The bells have fallen silent. Find Archivist Maelin beside the old road."
                .into(),
        });
        world.set_campaign_map("hearthmere");
        world.validate()?;
        Ok(world)
    }
    fn set_campaign_map(&mut self, region: &str) {
        self.map = Self::campaign_map(region);
    }
    fn campaign_map(region: &str) -> NavGrid {
        let mut map = NavGrid::new(48, 36);
        for x in 0..48 {
            map.block(x, 0);
            map.block(x, 35);
        }
        for z in 0..36 {
            map.block(0, z);
            map.block(47, z);
        }
        let rectangles = match region {
            "hearthmere" => vec![
                (9, 7, 15, 12),
                (21, 6, 28, 12),
                (28, 24, 35, 30),
                (13, 27, 17, 32),
            ],
            "hollow_moor" => vec![
                (14, 8, 17, 15),
                (23, 22, 28, 26),
                (33, 6, 36, 13),
                (34, 27, 40, 30),
            ],
            _ => vec![
                (12, 3, 15, 14),
                (12, 23, 15, 33),
                (27, 9, 30, 15),
                (27, 23, 30, 31),
                (37, 4, 40, 11),
            ],
        };
        for (x0, z0, x1, z1) in rectangles {
            for z in z0..z1 {
                for x in x0..x1 {
                    map.block(x, z);
                }
            }
        }
        map
    }
    pub fn stats_for(&self, unit: &Unit) -> UnitStats {
        let mut stats = unit.kind.stats();
        if unit.kind == UnitKind::Commander
            && unit.team == Team::Player
            && let Some(c) = &self.campaign
        {
            let profile = c.class();
            stats.range = if profile.combat_style == "melee" {
                1_800
            } else {
                6_500
            };
            stats.damage = stats.damage.saturating_add(
                i32::try_from(c.level.saturating_sub(1))
                    .unwrap_or(i32::MAX)
                    .saturating_mul(8),
            );
            for skill in &profile.skills {
                if skill.kind == "passive" && c.learned.contains(&skill.id) {
                    stats.damage = stats.damage.saturating_add(skill.power);
                }
            }
            for id in c.equipped.values() {
                if let Some(item) = c.inventory.iter().find(|i| i.id == *id) {
                    stats.damage = stats.damage.saturating_add(item.power);
                }
            }
        }
        stats
    }
    pub(super) fn movement_speed(&self, unit: &Unit) -> i32 {
        let speed = unit.kind.stats().speed_per_tick;
        if self.campaign.as_ref().is_some_and(|c| {
            c.effects
                .iter()
                .any(|e| e.target == unit.id && e.slow && e.remaining > 0)
        }) {
            speed / 2
        } else {
            speed
        }
    }
    pub fn learn_skill(&mut self, id: &str) -> Result<(), CommandError> {
        let c = self
            .campaign
            .as_mut()
            .ok_or_else(|| err("Skills belong to campaign characters."))?;
        let skill = c
            .class()
            .skills
            .iter()
            .find(|s| s.id == id)
            .ok_or_else(|| err("This skill belongs to another class."))?;
        if c.learned.contains(id) {
            return Err(err("That skill is already learned."));
        }
        if c.skill_points == 0 {
            return Err(err("Earn a skill point by gaining a level."));
        }
        if c.level < skill.minimum_level {
            return Err(err(format!("Requires level {}.", skill.minimum_level)));
        }
        if skill
            .requires
            .as_ref()
            .is_some_and(|p| !c.learned.contains(p))
        {
            return Err(err("Learn the preceding skill in this branch first."));
        }
        c.learned.insert(id.into());
        c.skill_points -= 1;
        Ok(())
    }
    pub fn respec(&mut self) -> Result<(), CommandError> {
        let c = self
            .campaign
            .as_mut()
            .ok_or_else(|| err("No campaign character is active."))?;
        if c.region != "hearthmere" {
            return Err(err("Return to Hearthmere to retrain freely."));
        }
        c.learned = BTreeSet::from([c.class().skills[0].id.clone()]);
        c.skill_points = c.level.saturating_add(1);
        c.cooldowns.clear();
        c.dialogue = "Your training has been restored. Choose your disciplines anew.".into();
        Ok(())
    }
    pub fn cast_skill(&mut self, id: &str, target: Point) -> Result<(), CommandError> {
        let c = self
            .campaign
            .as_ref()
            .ok_or_else(|| err("No campaign character is active."))?;
        let skill = c
            .class()
            .skills
            .iter()
            .find(|s| s.id == id)
            .ok_or_else(|| err("Unknown class skill."))?
            .clone();
        if !c.learned.contains(id) || skill.kind != "active" {
            return Err(err("Learn an active skill before using it."));
        }
        if c.cooldowns.get(id).copied().unwrap_or(0) > 0 {
            return Err(err("This skill is still recovering."));
        }
        if c.focus < skill.focus_cost {
            return Err(err("Not enough focus. Let it recover between attacks."));
        }
        let hero = self.commander().ok_or(CommandError::MatchFinished)?;
        let hero_id = hero.id;
        let origin = hero.position;
        let stats = self.stats_for(hero);
        if !self.map.contains_point(target)
            || origin.distance_squared(target) > (skill.range as u64).pow(2)
        {
            return Err(err("Move closer to the target before using this skill."));
        }
        if skill.effect == "dash"
            && self.map.path(origin, target).is_empty()
            && origin.distance_squared(target) > 100u64.pow(2)
        {
            return Err(CommandError::UnreachableDestination);
        }
        // Validate all conditions before committing resources or modifying the world.
        if skill.effect == "summon" && self.next_id == u32::MAX {
            return Err(CommandError::IdExhausted);
        }
        let c = self.campaign.as_mut().expect("active campaign");
        c.focus -= skill.focus_cost;
        c.cooldowns.insert(id.into(), skill.cooldown_ticks);
        self.events.push(SimEvent::Ability {
            position: target,
            radius: skill.radius,
        });
        let power = skill
            .power
            .saturating_add((stats.damage - UnitKind::Commander.stats().damage).max(0));
        if skill.effect == "guard" {
            c.ward_ticks = 240 + skill.tier * 60;
        }
        if skill.effect == "summon" {
            let kind = if c.class().combat_style == "caster" {
                UnitKind::Ranger
            } else {
                UnitKind::Vanguard
            };
            let p = self
                .map
                .nearest_walkable(target)
                .ok_or(CommandError::UnreachableDestination)?;
            self.spawn(kind, Team::Player, p)?;
        } else {
            let mut drained: i32 = 0;
            for unit in &mut self.units {
                if skill.effect == "dash" && unit.id == hero_id {
                    unit.position = target;
                    unit.target = None;
                    unit.path.clear();
                    unit.path_goal = None;
                    unit.order = Order::Idle;
                }
                if unit.position.distance_squared(target) > (skill.radius as u64).pow(2) {
                    continue;
                }
                if skill.effect == "heal" && unit.team == Team::Player {
                    unit.hp = unit.hp.saturating_add(power).min(unit.max_hp);
                } else if unit.team == Team::Enemy
                    && !["heal", "guard"].contains(&skill.effect.as_str())
                {
                    let damage = power.min(unit.hp.max(0));
                    unit.hp -= damage;
                    drained = drained.saturating_add(damage / 3);
                    self.events.push(SimEvent::Hit {
                        source: hero_id,
                        target: unit.id,
                        damage,
                        position: unit.position,
                    });
                    if ["frost", "poison"].contains(&skill.effect.as_str()) {
                        c.effects.push(StatusEffect {
                            target: unit.id,
                            remaining: 180,
                            periodic_damage: if skill.effect == "poison" { 35 } else { 0 },
                            slow: skill.effect == "frost",
                        });
                    }
                }
            }
            if skill.effect == "drain"
                && let Some(hero) = self.units.iter_mut().find(|u| u.id == hero_id)
            {
                hero.hp = hero.hp.saturating_add(drained).min(hero.max_hp);
            }
        }
        self.resolve_deaths();
        self.update_outcome();
        Ok(())
    }
    pub(super) fn step_campaign_effects(&mut self) {
        let Some(c) = &mut self.campaign else {
            return;
        };
        for value in c.cooldowns.values_mut() {
            *value = value.saturating_sub(1);
        }
        c.cooldowns.retain(|_, value| *value > 0);
        c.ward_ticks = c.ward_ticks.saturating_sub(1);
        if self.tick.is_multiple_of(6) {
            c.focus = (c.focus + 1).min(100);
        }
        for effect in &mut c.effects {
            effect.remaining = effect.remaining.saturating_sub(1);
            if self.tick.is_multiple_of(u64::from(TICK_HZ))
                && effect.periodic_damage > 0
                && let Some(unit) = self
                    .units
                    .iter_mut()
                    .find(|u| u.id == effect.target && u.hp > 0)
            {
                let damage = effect.periodic_damage.min(unit.hp);
                unit.hp -= damage;
                self.events.push(SimEvent::Hit {
                    source: 1,
                    target: unit.id,
                    damage,
                    position: unit.position,
                });
            }
        }
        c.effects
            .retain(|e| e.remaining > 0 && self.units.iter().any(|u| u.id == e.target && u.hp > 0));
    }
    pub(super) fn reward_campaign_deaths(&mut self) {
        let Some(c) = &mut self.campaign else {
            return;
        };
        for unit in self
            .units
            .iter()
            .filter(|u| u.hp <= 0 && u.team == Team::Enemy)
        {
            c.gain_experience(if unit.kind == UnitKind::Brute {
                100
            } else {
                45
            });
            let power = 5 + (unit.id % 7) as i32;
            if let Ok(item) = c.new_item(
                if unit.kind == UnitKind::Brute {
                    "Ashkeeper's Signet"
                } else {
                    "Weathered Armament"
                },
                if unit.kind == UnitKind::Brute {
                    "charm"
                } else {
                    "weapon"
                },
                power,
                if unit.kind == UnitKind::Brute {
                    "rare"
                } else {
                    "common"
                },
            ) {
                c.ground_loot.push(GroundLoot {
                    item,
                    position: unit.position,
                });
            }
        }
    }
    pub fn equip(&mut self, id: u32) -> Result<(), CommandError> {
        let c = self
            .campaign
            .as_mut()
            .ok_or_else(|| err("No campaign character is active."))?;
        let item = c
            .inventory
            .iter()
            .find(|i| i.id == id)
            .ok_or_else(|| err("Pick up the item before equipping it."))?;
        c.equipped.insert(item.slot.clone(), id);
        Ok(())
    }
    pub fn interact(&mut self, target: &str) -> Result<(), CommandError> {
        let c = self
            .campaign
            .as_ref()
            .ok_or_else(|| err("No campaign character is active."))?;
        let position = self
            .commander()
            .ok_or(CommandError::MatchFinished)?
            .position;
        if let Some(id) = target.strip_prefix("loot:") {
            let id: u32 = id.parse().map_err(|_| err("Invalid loot identity."))?;
            let c = self.campaign.as_mut().expect("campaign");
            let index = c
                .ground_loot
                .iter()
                .position(|l| l.item.id == id)
                .ok_or_else(|| err("That item is no longer on the ground."))?;
            if c.ground_loot[index].position.distance_squared(position) > 3_000u64.pow(2) {
                return Err(err("Move closer to collect the item."));
            }
            let loot = c.ground_loot.remove(index);
            c.dialogue = format!("Collected {}.", loot.item.name);
            c.inventory.push(loot.item);
            return Ok(());
        }
        let poi = points_of_interest(&c.region)
            .into_iter()
            .find(|p| p.id == target)
            .ok_or_else(|| err("There is nothing to interact with here."))?;
        if position.distance_squared(poi.position) > 3_500u64.pow(2) {
            return Err(err("Move closer to interact."));
        }
        if let Some(region) = poi.destination {
            return self.travel(&region);
        }
        let enemies_alive = self.units.iter().any(|u| u.team == Team::Enemy);
        let key = format!("{}:{target}", c.region);
        let creates_item = match target {
            "elder" => c.main_quest == 2,
            "merchant" => {
                c.flags.contains("satchel_found") && !c.flags.contains("satchel_returned")
            }
            "chest" => !c.flags.contains(&key),
            _ => false,
        };
        if creates_item && c.next_item_id == u32::MAX {
            return Err(err("Item identity space exhausted."));
        }
        let c = self.campaign.as_mut().expect("campaign");
        match target {
            "elder" => {
                if c.main_quest == 0 {
                    c.main_quest = 1;
                    c.gain_experience(40);
                    c.dialogue = "Maelin: The Ashen Concord once kept these roads safe. Its keeper has fallen silent. Cross the moor, descend into the sanctum, and bring the relic home. The old stairs lie beyond the eastern road.".into();
                } else if c.main_quest == 2 {
                    c.main_quest = 3;
                    c.gain_experience(200);
                    self.economy.essence = self.economy.essence.saturating_add(120);
                    let item = c.new_item("Concord Keeper's Seal", "charm", 30, "legendary")?;
                    c.inventory.push(item);
                    c.dialogue = "Maelin: You brought back more than a relic. You brought back a reason to keep the lamps burning. Beyond these roads, the other concords are breaking. Your journey has only begun. Prologue complete.".into();
                } else {
                    c.dialogue = if c.main_quest == 1 { "Maelin: Follow the road east. The sanctum's keeper carries the seal. Return to me when the Concord is yours." } else { "Maelin: The Concord is safe. Hearthmere will remember your name." }.into();
                }
            }
            "merchant" => {
                if c.flags.contains("satchel_found") && !c.flags.contains("satchel_returned") {
                    c.flags.insert("satchel_returned".into());
                    c.gain_experience(90);
                    self.economy.essence = self.economy.essence.saturating_add(45);
                    let item = c.new_item("Iven's Wayfinder", "charm", 18, "rare")?;
                    c.inventory.push(item);
                    c.dialogue = "Iven: My satchel! Take this wayfinder. It has seen more roads than either of us. Side quest complete.".into();
                } else {
                    c.flags.insert("satchel_quest".into());
                    c.dialogue = "Iven: I lost a leather satchel beside the southern moor trail. Bring it back and I will see you rewarded. Rest here whenever the road grows cruel.".into();
                }
                if let Some(hero) = self
                    .units
                    .iter_mut()
                    .find(|u| u.kind == UnitKind::Commander)
                {
                    hero.hp = hero.max_hp;
                }
                c.focus = 100;
            }
            "relic" => {
                if c.main_quest == 0 {
                    return Err(err(
                        "Speak to Archivist Maelin before disturbing the Concord.",
                    ));
                }
                if enemies_alive {
                    return Err(err(
                        "Defeat the sanctum's guardians before claiming the Concord.",
                    ));
                }
                if c.main_quest == 1 {
                    c.main_quest = 2;
                    c.gain_experience(160);
                    c.dialogue =
                        "The Ashen Concord warms in your hands. Return it to Maelin in Hearthmere."
                            .into();
                } else {
                    c.dialogue =
                        "The reliquary is empty. You have already recovered the Concord.".into();
                }
            }
            "satchel" => {
                if !c.flags.insert(key) {
                    return Err(err("You have already recovered this satchel."));
                }
                c.flags.insert("satchel_found".into());
                c.dialogue =
                    "A battered satchel bears Iven's mark. Return it to the quartermaster.".into();
            }
            "chest" => {
                if !c.flags.insert(key) {
                    return Err(err("This cache has already been opened."));
                }
                let item = c.new_item("Wayfarer's Tempered Blade", "weapon", 16, "magic")?;
                c.inventory.push(item);
                self.economy.essence = self.economy.essence.saturating_add(20);
                c.dialogue = "You recover a tempered weapon and a handful of old coin. Open Inventory to equip your find.".into();
            }
            "shrine" => {
                if let Some(hero) = self
                    .units
                    .iter_mut()
                    .find(|u| u.kind == UnitKind::Commander)
                {
                    hero.hp = hero.max_hp;
                }
                c.focus = 100;
                c.dialogue =
                    "The shrine steadies your breathing. Health and focus restored.".into();
            }
            _ => return Err(err("This interaction is not available.")),
        }
        Ok(())
    }
    pub fn travel(&mut self, region: &str) -> Result<(), CommandError> {
        let c = self
            .campaign
            .as_ref()
            .ok_or_else(|| err("No campaign is active."))?;
        let origin = self
            .commander()
            .ok_or(CommandError::MatchFinished)?
            .position;
        let gate = points_of_interest(&c.region)
            .into_iter()
            .find(|p| p.destination.as_deref() == Some(region))
            .ok_or_else(|| err("There is no connecting road from this region."))?;
        if origin.distance_squared(gate.position) > 3_500u64.pow(2) {
            return Err(err("Reach the region's marked exit before travelling."));
        }
        // Validate the entire transition before changing region, identity counters or entities.
        let encounter: &[(i32, i32, bool)] = if !c.visited.contains(region) {
            match region {
                "hollow_moor" => &[
                    (19_500, 17_500, false),
                    (21_500, 19_500, false),
                    (27_500, 17_500, false),
                    (30_500, 19_500, false),
                    (37_500, 18_500, false),
                    (40_500, 20_500, true),
                ],
                "forgotten_sanctum" => &[
                    (19_500, 18_500, false),
                    (24_500, 17_500, false),
                    (32_500, 20_500, false),
                    (38_500, 18_500, true),
                ],
                _ => &[],
            }
        } else {
            &[]
        };
        self.next_id
            .checked_add(encounter.len() as u32)
            .ok_or(CommandError::IdExhausted)?;
        let previous = c.region.clone();
        let enemies = self
            .units
            .iter()
            .filter(|u| u.team == Team::Enemy)
            .cloned()
            .collect();
        let c = self.campaign.as_mut().expect("campaign");
        c.regions.insert(
            previous.clone(),
            RegionMemory {
                enemies,
                loot: std::mem::take(&mut c.ground_loot),
            },
        );
        let memory = c.regions.remove(region);
        c.visited.insert(region.into());
        c.region = region.into();
        c.effects.clear();
        c.dialogue = format!("Entered {}.", region_name(region));
        self.units.retain(|u| u.team == Team::Player);
        self.set_campaign_map(region);
        let arrival_x = if (previous == "hollow_moor" && region == "hearthmere")
            || previous == "forgotten_sanctum"
        {
            41_000
        } else {
            6_500
        };
        for (index, unit) in self.units.iter_mut().enumerate() {
            let p = Point::new(arrival_x, 17_000 + (index % 5) as i32 * 600);
            unit.position = self
                .map
                .nearest_walkable(p)
                .ok_or(CommandError::UnreachableDestination)?;
            unit.target = None;
            unit.path.clear();
            unit.path_goal = None;
            unit.order = Order::Idle;
        }
        if let Some(memory) = memory {
            self.units.extend(memory.enemies);
            self.campaign.as_mut().expect("campaign").ground_loot = memory.loot;
        } else {
            for &(x, z, boss) in encounter {
                self.spawn(
                    if boss {
                        UnitKind::Brute
                    } else {
                        UnitKind::Reaver
                    },
                    Team::Enemy,
                    Point::new(x, z),
                )?;
            }
        }
        self.events.clear();
        Ok(())
    }
    pub(super) fn validate_campaign(&self) -> Result<(), SaveError> {
        let Some(c) = &self.campaign else {
            return Ok(());
        };
        let fail = |s: &str| SaveError(s.into());
        c.character.validate().map_err(SaveError)?;
        if region_name(&c.region) == "Unknown"
            || !c.visited.contains(&c.region)
            || c.level == 0
            || c.focus > 100
            || c.ward_ticks > 360
            || c.main_quest > 3
            || c.experience >= c.next_level_experience()
        {
            return Err(fail("Invalid character progression or region state."));
        }
        if self.map != Self::campaign_map(&c.region)
            || !self.objectives.is_empty()
            || self.wave != 0
            || self.wave_ticks_remaining != 0
            || self.status == MatchStatus::Victory
            || c.visited
                .iter()
                .any(|region| region != &c.region && !c.regions.contains_key(region))
            || c.next_item_id == 0
            || c.visited
                .iter()
                .any(|region| region_name(region) == "Unknown")
        {
            return Err(fail(
                "Campaign map, outcome or persistent identity state is invalid.",
            ));
        }
        let class = c.class();
        if !c.learned.contains(&class.skills[0].id)
            || u64::from(c.skill_points) + (c.learned.len() as u64 - 1) != u64::from(c.level) + 1
        {
            return Err(fail(
                "Skill-point accounting does not match character level.",
            ));
        }
        for id in &c.learned {
            let Some(skill) = class.skills.iter().find(|s| &s.id == id) else {
                return Err(fail("Character learned a foreign skill."));
            };
            if c.level < skill.minimum_level
                || skill
                    .requires
                    .as_ref()
                    .is_some_and(|p| !c.learned.contains(p))
            {
                return Err(fail("Skill prerequisites are invalid."));
            }
        }
        for (id, ticks) in &c.cooldowns {
            if !class.skills.iter().any(|s| {
                &s.id == id
                    && s.kind == "active"
                    && c.learned.contains(id)
                    && *ticks <= s.cooldown_ticks
            }) {
                return Err(fail("Invalid skill cooldown."));
            }
        }
        let mut items = BTreeSet::new();
        let mut verify_item = |i: &Item| -> Result<(), SaveError> {
            if i.id == 0
                || i.id >= c.next_item_id
                || !items.insert(i.id)
                || !["weapon", "charm"].contains(&i.slot.as_str())
                || i.power < 0
            {
                return Err(fail("Invalid or duplicated item identity."));
            }
            Ok(())
        };
        for item in &c.inventory {
            verify_item(item)?;
        }
        for loot in &c.ground_loot {
            verify_item(&loot.item)?;
            if !self.map.contains_point(loot.position) {
                return Err(fail("Loot is outside the region."));
            }
        }
        let mut ids: BTreeSet<_> = self.units.iter().map(|u| u.id).collect();
        for (region, memory) in &c.regions {
            if region == &c.region
                || region_name(region) == "Unknown"
                || !c.visited.contains(region)
            {
                return Err(fail("Invalid stored region."));
            }
            let region_map = Self::campaign_map(region);
            for u in &memory.enemies {
                if u.id == 0
                    || u.team != Team::Enemy
                    || u.hp <= 0
                    || u.hp > u.max_hp
                    || u.max_hp != u.kind.stats().hp
                    || u.id >= self.next_id
                    || !ids.insert(u.id)
                    || u.attack_cooldown > u.kind.stats().attack_interval
                    || !region_map.contains_point(u.position)
                    || u.target.is_some_and(|p| !region_map.contains_point(p))
                    || u.path_goal.is_some_and(|p| !region_map.contains_point(p))
                {
                    return Err(fail(
                        "Stored enemies have invalid identity, health or navigation state.",
                    ));
                }
                let mut previous = u.position;
                for point in &u.path {
                    let (x, z) = (
                        previous.x.div_euclid(CELL_SIZE),
                        previous.z.div_euclid(CELL_SIZE),
                    );
                    let (nx, nz) = (point.x.div_euclid(CELL_SIZE), point.z.div_euclid(CELL_SIZE));
                    if !region_map.contains_point(*point)
                        || (x - nx).abs() > 1
                        || (z - nz).abs() > 1
                        || (x != nx
                            && z != nz
                            && (!region_map.is_walkable(x, nz) || !region_map.is_walkable(nx, z)))
                    {
                        return Err(fail("Stored enemy has an invalid navigation path."));
                    }
                    previous = *point;
                }
            }
            for loot in &memory.loot {
                verify_item(&loot.item)?;
                if !region_map.contains_point(loot.position) {
                    return Err(fail("Stored loot is outside traversable terrain."));
                }
            }
        }
        for (slot, id) in &c.equipped {
            if !c.inventory.iter().any(|i| i.id == *id && i.slot == *slot) {
                return Err(fail("Equipped item is missing from inventory."));
            }
        }
        for e in &c.effects {
            if e.remaining == 0
                || e.remaining > 180
                || e.periodic_damage < 0
                || e.periodic_damage > 35
                || !self
                    .units
                    .iter()
                    .any(|u| u.id == e.target && u.team == Team::Enemy)
            {
                return Err(fail("Invalid timed skill effect."));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn allocation_exhaustion_does_not_consume_rewards_or_partially_travel() {
        for (target, position) in [
            ("chest", Point::new(7_500, 25_500)),
            ("elder", Point::new(11_500, 18_500)),
            ("merchant", Point::new(12_500, 23_500)),
        ] {
            let mut world = make("paladin");
            move_to(&mut world, position);
            let c = world.campaign.as_mut().unwrap();
            c.main_quest = 2;
            c.flags.insert("satchel_found".into());
            c.next_item_id = u32::MAX;
            world.validate().unwrap();
            let before = world.save_json().unwrap();
            assert!(world.interact(target).is_err());
            assert_eq!(world.save_json().unwrap(), before);
            world.campaign.as_mut().unwrap().next_item_id = 1;
            world.economy.essence = u32::MAX;
            world.interact(target).unwrap();
            assert_eq!(world.economy.essence, u32::MAX);
        }
        for next in [u32::MAX, u32::MAX - 2] {
            let mut world = make("paladin");
            move_to(&mut world, Point::new(43_500, 18_500));
            world.next_id = next;
            world.validate().unwrap();
            let before = world.save_json().unwrap();
            assert!(world.travel("hollow_moor").is_err());
            assert_eq!(world.save_json().unwrap(), before);
        }
    }
    #[test]
    fn stored_regions_cannot_smuggle_invalid_navigation_into_a_save() {
        let mut world = make("paladin");
        move_to(&mut world, Point::new(43_500, 18_500));
        world.travel("hollow_moor").unwrap();
        world.travel("hearthmere").unwrap();
        world.validate().unwrap();
        let original = world.save_json().unwrap();
        let enemy = &mut world
            .campaign
            .as_mut()
            .unwrap()
            .regions
            .get_mut("hollow_moor")
            .unwrap()
            .enemies[0];
        enemy.position.x = -1;
        assert!(World::load_json(&world.save_json().unwrap()).is_err());
        world = World::load_json(&original).unwrap();
        let enemy = &mut world
            .campaign
            .as_mut()
            .unwrap()
            .regions
            .get_mut("hollow_moor")
            .unwrap()
            .enemies[0];
        enemy.path.push_back(Point::new(45_500, 30_500));
        assert!(World::load_json(&world.save_json().unwrap()).is_err());
        world = World::load_json(&original).unwrap();
        world.map.blocked[200] = !world.map.blocked[200];
        assert!(World::load_json(&world.save_json().unwrap()).is_err());
    }
    fn make(class_id: &str) -> World {
        World::new_campaign(
            9,
            CharacterCreation {
                class_id: class_id.into(),
                ..Default::default()
            },
        )
        .unwrap()
    }
    fn move_to(world: &mut World, p: Point) {
        world
            .apply(Command::Move {
                units: vec![1],
                target: p,
                attack_move: true,
            })
            .unwrap();
        for _ in 0..1200 {
            world.step();
            if world
                .commander()
                .is_some_and(|h| h.position.distance_squared(p) < 700u64.pow(2))
            {
                break;
            }
        }
    }
    #[test]
    fn every_class_can_be_created_learn_and_roundtrip() {
        assert_eq!(catalogue().classes.len(), 19);
        for profile in &catalogue().classes {
            let mut world = make(&profile.id);
            assert_eq!(world.units.len(), 1);
            let other = &profile.skills[3];
            world
                .apply(Command::LearnSkill {
                    skill: other.id.clone(),
                })
                .unwrap();
            let saved = world.save_json().unwrap();
            assert_eq!(
                World::load_json(&saved).unwrap().save_json().unwrap(),
                saved
            );
            assert!(
                world
                    .apply(Command::LearnSkill {
                        skill: profile.skills[2].id.clone()
                    })
                    .is_err()
            );
        }
    }
    #[test]
    fn creator_and_foreign_skill_errors_are_atomic() {
        let c = CharacterCreation {
            skin: "<script>".into(),
            ..Default::default()
        };
        assert!(World::new_campaign(1, c).is_err());
        let mut w = make("paladin");
        let before = w.checksum();
        assert!(
            w.apply(Command::LearnSkill {
                skill: "necromancer.0.0".into()
            })
            .is_err()
        );
        assert_eq!(before, w.checksum());
        let mut value: serde_json::Value = serde_json::from_str(&w.save_json().unwrap()).unwrap();
        value["campaign"]["skill_points"] = 999.into();
        assert!(World::load_json(&value.to_string()).is_err());
    }
    #[test]
    fn story_rewards_chests_and_region_state_persist() {
        let mut w = make("sorcerer");
        move_to(&mut w, Point::new(11_500, 18_500));
        w.apply(Command::Interact {
            target: "elder".into(),
        })
        .unwrap();
        let xp = w.campaign.as_ref().unwrap().experience;
        w.apply(Command::Interact {
            target: "elder".into(),
        })
        .unwrap();
        assert_eq!(xp, w.campaign.as_ref().unwrap().experience);
        move_to(&mut w, Point::new(7_500, 25_500));
        w.apply(Command::Interact {
            target: "chest".into(),
        })
        .unwrap();
        assert!(
            w.apply(Command::Interact {
                target: "chest".into()
            })
            .is_err()
        );
        w.apply(Command::Equip { item: 1 }).unwrap();
        assert!(w.stats_for(w.commander().unwrap()).damage > 90);
        move_to(&mut w, Point::new(43_500, 18_500));
        w.apply(Command::Travel {
            region: "hollow_moor".into(),
        })
        .unwrap();
        assert_eq!(w.units.iter().filter(|u| u.team == Team::Enemy).count(), 6);
        move_to(&mut w, Point::new(4_500, 18_500));
        w.apply(Command::Travel {
            region: "hearthmere".into(),
        })
        .unwrap();
        let saved = w.save_json().unwrap();
        assert_eq!(World::load_json(&saved).unwrap().checksum(), w.checksum());
    }
    #[test]
    fn skills_apply_distinct_effects_and_cooldown_and_cost() {
        for cid in [
            "sorcerer",
            "necromancer",
            "witch_doctor",
            "warlock",
            "blood_knight",
            "tempest",
        ] {
            let mut w = make(cid);
            let p = w.commander().unwrap().position;
            w.spawn(UnitKind::Brute, Team::Enemy, Point::new(p.x + 1000, p.z))
                .unwrap();
            let id = class(cid).unwrap().skills[0].id.clone();
            let before = w.campaign.as_ref().unwrap().focus;
            w.apply(Command::CastSkill {
                skill: id.clone(),
                target: Point::new(p.x + 1000, p.z),
            })
            .unwrap();
            assert!(w.campaign.as_ref().unwrap().focus < before);
            let checksum = w.checksum();
            assert!(
                w.apply(Command::CastSkill {
                    skill: id,
                    target: p
                })
                .is_err()
            );
            assert_eq!(checksum, w.checksum());
            let save = w.save_json().unwrap();
            World::load_json(&save).unwrap();
        }
    }
}
