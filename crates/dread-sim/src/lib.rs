//! A complete deterministic skirmish loop shared by native and browser clients.
//! The renderer owns interpolation/input/audio only; orders, combat, navigation,
//! recruitment, capture, saves and victory are authoritative here.

pub use dread_core::{CELL_SIZE, NavGrid, Point};
use dread_core::{SpatialIndex, stable_hash};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, VecDeque};
use std::fmt;

pub const TICK_HZ: u32 = 30;
pub const CAPTURE_TICKS: u32 = 150;
pub const CAPTURE_RADIUS: i32 = 2_200;
pub const ABILITY_RADIUS: i32 = 4_000;
pub const ABILITY_RANGE: i32 = 12_000;
pub const ABILITY_COOLDOWN: u32 = 240;
pub const TOTAL_WAVES: u32 = 5;
const SAVE_VERSION: u32 = 1;
const WAVE_INTERVAL: u32 = 900;

pub type UnitId = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Team {
    Player,
    Enemy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnitKind {
    Commander,
    Vanguard,
    Ranger,
    Reaver,
    Brute,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnitStats {
    pub hp: i32,
    pub damage: i32,
    pub range: i32,
    pub speed_per_tick: i32,
    pub attack_interval: u32,
    pub recruitment_cost: Option<u32>,
}

impl UnitKind {
    pub const fn stats(self) -> UnitStats {
        match self {
            Self::Commander => UnitStats {
                hp: 1_600,
                damage: 90,
                range: 1_650,
                speed_per_tick: 125,
                attack_interval: 15,
                recruitment_cost: None,
            },
            Self::Vanguard => UnitStats {
                hp: 450,
                damage: 44,
                range: 1_400,
                speed_per_tick: 110,
                attack_interval: 20,
                recruitment_cost: Some(25),
            },
            Self::Ranger => UnitStats {
                hp: 260,
                damage: 40,
                range: 6_500,
                speed_per_tick: 115,
                attack_interval: 24,
                recruitment_cost: Some(35),
            },
            Self::Reaver => UnitStats {
                hp: 170,
                damage: 22,
                range: 1_250,
                speed_per_tick: 90,
                attack_interval: 26,
                recruitment_cost: None,
            },
            Self::Brute => UnitStats {
                hp: 720,
                damage: 65,
                range: 1_600,
                speed_per_tick: 65,
                attack_interval: 42,
                recruitment_cost: None,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Order {
    Idle,
    Move,
    AttackMove,
    Hold,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Unit {
    pub id: UnitId,
    pub kind: UnitKind,
    pub team: Team,
    pub position: Point,
    pub hp: i32,
    pub max_hp: i32,
    pub target: Option<Point>,
    order: Order,
    path: VecDeque<Point>,
    path_goal: Option<Point>,
    attack_cooldown: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Objective {
    pub id: u32,
    pub position: Point,
    pub progress: u32,
    pub captured: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Economy {
    pub essence: u32,
    pub kills: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MatchStatus {
    Playing,
    Victory,
    Defeat,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Command {
    Move {
        units: Vec<UnitId>,
        target: Point,
        attack_move: bool,
    },
    Hold {
        units: Vec<UnitId>,
    },
    Cast {
        target: Point,
    },
    Recruit {
        kind: UnitKind,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SimEvent {
    Hit {
        source: UnitId,
        target: UnitId,
        damage: i32,
        position: Point,
    },
    Died {
        unit: UnitId,
        position: Point,
        team: Team,
    },
    Ability {
        position: Point,
        radius: i32,
    },
    Captured {
        objective: u32,
        position: Point,
    },
    Wave {
        number: u32,
    },
    Recruited {
        unit: UnitId,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandError {
    MatchFinished,
    EmptySelection,
    UnknownUnit(UnitId),
    NotPlayerUnit(UnitId),
    UnreachableDestination,
    AbilityCoolingDown,
    OutOfRange,
    InvalidRecruit,
    InsufficientEssence,
    IdExhausted,
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MatchFinished => write!(f, "The battle has ended. Start a new expedition."),
            Self::EmptySelection => write!(f, "Select one or more allied units first."),
            Self::UnknownUnit(id) => write!(f, "Unit {id} is no longer available."),
            Self::NotPlayerUnit(id) => write!(f, "Unit {id} is not yours to command."),
            Self::UnreachableDestination => write!(f, "Your squad cannot reach that destination."),
            Self::AbilityCoolingDown => write!(f, "Rift pulse is still recharging."),
            Self::OutOfRange => write!(f, "Target is outside the commander's 12-unit cast range."),
            Self::InvalidRecruit => write!(f, "Only vanguards and rangers can be recruited."),
            Self::InsufficientEssence => {
                write!(f, "Not enough essence. Capture relics or defeat enemies.")
            }
            Self::IdExhausted => write!(f, "The unit identity space is exhausted."),
        }
    }
}
impl std::error::Error for CommandError {}

#[derive(Debug)]
pub struct SaveError(pub String);
impl fmt::Display for SaveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
impl std::error::Error for SaveError {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct World {
    pub version: u32,
    pub tick: u64,
    pub map: NavGrid,
    pub units: Vec<Unit>,
    pub objectives: Vec<Objective>,
    pub economy: Economy,
    pub status: MatchStatus,
    pub wave: u32,
    pub wave_ticks_remaining: u32,
    pub commander_cooldown: u32,
    pub events: Vec<SimEvent>,
    rng: u64,
    next_id: UnitId,
    #[serde(skip)]
    spatial: SpatialIndex,
}

impl World {
    pub fn new(seed: u64) -> Self {
        let mut map = NavGrid::new(48, 36);
        for x in 0..48 {
            map.block(x, 0);
            map.block(x, 35);
        }
        for z in 0..36 {
            map.block(0, z);
            map.block(47, z);
        }
        for (x0, z0, x1, z1) in [
            (13, 6, 17, 12),
            (21, 23, 25, 28),
            (31, 9, 35, 13),
            (36, 22, 40, 24),
        ] {
            for z in z0..z1 {
                for x in x0..x1 {
                    map.block(x, z);
                }
            }
        }
        for (x, z) in [
            (11, 25),
            (12, 26),
            (19, 16),
            (20, 16),
            (30, 26),
            (30, 27),
            (39, 5),
            (40, 5),
        ] {
            map.block(x, z);
        }
        let rng = (seed ^ 0x9e3779b97f4a7c15).max(1);
        let mut world = Self {
            version: SAVE_VERSION,
            tick: 0,
            map,
            units: Vec::new(),
            objectives: vec![
                Objective {
                    id: 1,
                    position: Point::new(20_500, 6_500),
                    progress: 0,
                    captured: false,
                },
                Objective {
                    id: 2,
                    position: Point::new(25_500, 18_500),
                    progress: 0,
                    captured: false,
                },
                Objective {
                    id: 3,
                    position: Point::new(38_500, 28_500),
                    progress: 0,
                    captured: false,
                },
            ],
            economy: Economy {
                essence: 90,
                kills: 0,
            },
            status: MatchStatus::Playing,
            wave: 0,
            wave_ticks_remaining: 600,
            commander_cooldown: 0,
            events: Vec::new(),
            rng,
            next_id: 1,
            spatial: SpatialIndex::default(),
        };
        world
            .spawn(UnitKind::Commander, Team::Player, Point::new(7_500, 18_500))
            .expect("initial IDs");
        for (x, z) in [
            (6_000, 16_000),
            (7_000, 16_000),
            (8_000, 16_000),
            (6_000, 21_000),
            (7_000, 21_000),
            (8_000, 21_000),
        ] {
            world
                .spawn(UnitKind::Vanguard, Team::Player, Point::new(x, z))
                .expect("initial IDs");
        }
        for z in [15_000, 17_000, 19_000, 21_000] {
            world
                .spawn(UnitKind::Ranger, Team::Player, Point::new(4_500, z))
                .expect("initial IDs");
        }
        // Guardians establish an immediate first objective; timed reinforcements
        // are separate, leaving the opening advance under the player's control.
        for (x, z) in [
            (24_500, 16_500),
            (25_500, 17_500),
            (27_500, 19_500),
            (29_500, 18_500),
            (27_500, 21_500),
        ] {
            world
                .spawn(UnitKind::Reaver, Team::Enemy, Point::new(x, z))
                .expect("initial IDs");
        }
        world
    }

    pub fn commander(&self) -> Option<&Unit> {
        self.units
            .iter()
            .find(|u| u.team == Team::Player && u.kind == UnitKind::Commander && u.hp > 0)
    }

    pub fn player_ids(&self) -> Vec<UnitId> {
        self.units
            .iter()
            .filter(|u| u.team == Team::Player && u.hp > 0)
            .map(|u| u.id)
            .collect()
    }

    pub fn apply(&mut self, command: Command) -> Result<(), CommandError> {
        if self.status != MatchStatus::Playing {
            return Err(CommandError::MatchFinished);
        }
        match command {
            Command::Move {
                units,
                target,
                attack_move,
            } => {
                let ids = self.validate_selection(units)?;
                let destination = self
                    .map
                    .nearest_walkable(target)
                    .ok_or(CommandError::UnreachableDestination)?;
                let columns = (ids.len() as u64).isqrt() as i32
                    + i32::from(!(ids.len() as u64).isqrt().pow(2).eq(&(ids.len() as u64)));
                let rows = (ids.len() as i32 + columns - 1) / columns;
                let mut routes = Vec::with_capacity(ids.len());
                for (ordinal, id) in ids.into_iter().enumerate() {
                    let index = self
                        .units
                        .iter()
                        .position(|u| u.id == id)
                        .expect("validated unit");
                    let offset = Point::new(
                        destination.x + ((ordinal as i32 % columns) * 2 - (columns - 1)) * 450,
                        destination.z + ((ordinal as i32 / columns) * 2 - (rows - 1)) * 450,
                    );
                    let goal = self
                        .map
                        .nearest_walkable(offset)
                        .ok_or(CommandError::UnreachableDestination)?;
                    let path = self.map.path(self.units[index].position, goal);
                    if path.is_empty() {
                        return Err(CommandError::UnreachableDestination);
                    }
                    routes.push((index, goal, path));
                }
                // Validate and compute every route first, so a rejected squad order
                // cannot partially mutate an earlier member of the selection.
                for (index, goal, path) in routes {
                    let unit = &mut self.units[index];
                    unit.target = Some(goal);
                    unit.path_goal = Some(goal);
                    unit.path = path.into();
                    unit.order = if attack_move {
                        Order::AttackMove
                    } else {
                        Order::Move
                    };
                }
            }
            Command::Hold { units } => {
                let ids = self.validate_selection(units)?;
                for unit in &mut self.units {
                    if ids.binary_search(&unit.id).is_ok() {
                        unit.order = Order::Hold;
                        unit.target = None;
                        unit.path_goal = None;
                        unit.path.clear();
                    }
                }
            }
            Command::Cast { target } => {
                if self.commander_cooldown > 0 {
                    return Err(CommandError::AbilityCoolingDown);
                }
                let commander = self.commander().ok_or(CommandError::MatchFinished)?;
                let commander_id = commander.id;
                if target.x < 0
                    || target.z < 0
                    || target.x >= i32::from(self.map.width) * CELL_SIZE
                    || target.z >= i32::from(self.map.height) * CELL_SIZE
                    || commander.position.distance_squared(target) > (ABILITY_RANGE as u64).pow(2)
                {
                    return Err(CommandError::OutOfRange);
                }
                self.commander_cooldown = ABILITY_COOLDOWN;
                self.events.push(SimEvent::Ability {
                    position: target,
                    radius: ABILITY_RADIUS,
                });
                for unit in &mut self.units {
                    if unit.position.distance_squared(target) <= (ABILITY_RADIUS as u64).pow(2) {
                        if unit.team == Team::Enemy {
                            let damage = 260.min(unit.hp.max(0));
                            unit.hp -= damage;
                            self.events.push(SimEvent::Hit {
                                source: commander_id,
                                target: unit.id,
                                damage,
                                position: unit.position,
                            });
                        } else {
                            unit.hp = (unit.hp + 70).min(unit.max_hp);
                        }
                    }
                }
                self.resolve_deaths();
                self.update_outcome();
            }
            Command::Recruit { kind } => {
                let price = kind
                    .stats()
                    .recruitment_cost
                    .ok_or(CommandError::InvalidRecruit)?;
                if self.economy.essence < price {
                    return Err(CommandError::InsufficientEssence);
                }
                let commander_position = self
                    .commander()
                    .ok_or(CommandError::MatchFinished)?
                    .position;
                let ordinal = self.next_id;
                let offset = Point::new(
                    commander_position.x - 1_200 + (ordinal % 4) as i32 * 800,
                    commander_position.z + 1_500 + (ordinal % 3) as i32 * 300,
                );
                let position = self
                    .map
                    .nearest_walkable(offset)
                    .ok_or(CommandError::UnreachableDestination)?;
                let id = self.spawn(kind, Team::Player, position)?;
                self.economy.essence -= price;
                self.events.push(SimEvent::Recruited { unit: id });
            }
        }
        Ok(())
    }

    /// Advance exactly one authoritative tick, independent of render frame rate.
    pub fn step(&mut self) {
        self.events.clear();
        if self.status != MatchStatus::Playing {
            return;
        }
        self.tick = self.tick.saturating_add(1);
        self.commander_cooldown = self.commander_cooldown.saturating_sub(1);
        for unit in &mut self.units {
            unit.attack_cooldown = unit.attack_cooldown.saturating_sub(1);
        }
        if self.wave < TOTAL_WAVES {
            self.wave_ticks_remaining = self.wave_ticks_remaining.saturating_sub(1);
            if self.wave_ticks_remaining == 0 {
                self.spawn_wave();
            }
        }
        self.spatial.rebuild(
            self.units
                .iter()
                .enumerate()
                .filter(|(_, u)| u.hp > 0)
                .map(|(i, u)| (i, u.position)),
        );
        let commander_position = self.commander().map(|u| u.position);
        let mut intents = Vec::with_capacity(self.units.len());
        for (index, unit) in self.units.iter().enumerate() {
            if unit.hp <= 0 {
                continue;
            }
            let stats = unit.kind.stats();
            let moving_peacefully = unit.order == Order::Move && !unit.path.is_empty();
            let aggro = if unit.kind == UnitKind::Ranger {
                10_000
            } else {
                8_000
            };
            let opponent = (!moving_peacefully)
                .then(|| self.nearest_enemy(index, aggro))
                .flatten();
            let mut attack = None;
            let mut goal = None;
            if let Some(target_index) = opponent {
                let target = &self.units[target_index];
                if unit.position.distance_squared(target.position) <= (stats.range as u64).pow(2)
                    && self.map.line_of_sight(unit.position, target.position)
                {
                    if unit.attack_cooldown == 0 {
                        attack = Some(target_index);
                    }
                } else if unit.order != Order::Hold {
                    goal = Some(target.position);
                }
            } else if matches!(unit.order, Order::Move | Order::AttackMove) {
                goal = unit.target;
            } else if unit.team == Team::Enemy {
                goal = commander_position;
            }
            intents.push((index, goal, attack));
        }
        let mut hits = Vec::new();
        for (index, goal, attack) in intents {
            if let Some(target_index) = attack {
                let unit = &mut self.units[index];
                let stats = unit.kind.stats();
                unit.attack_cooldown = stats.attack_interval;
                hits.push((index, target_index, stats.damage));
            }
            let Some(goal) = goal else {
                continue;
            };
            let unit = &mut self.units[index];
            if unit.position.distance_squared(goal) <= 100u64.pow(2) {
                unit.path.clear();
                if unit
                    .target
                    .is_some_and(|t| t.distance_squared(unit.position) <= 100u64.pow(2))
                {
                    unit.target = None;
                    unit.order = Order::Idle;
                }
                continue;
            }
            let goal_changed = unit.path_goal.is_none_or(|old| {
                old.x.div_euclid(CELL_SIZE) != goal.x.div_euclid(CELL_SIZE)
                    || old.z.div_euclid(CELL_SIZE) != goal.z.div_euclid(CELL_SIZE)
            });
            if unit.path.is_empty() || (goal_changed && self.tick % 12 == u64::from(unit.id % 12)) {
                unit.path = self.map.path(unit.position, goal).into();
                unit.path_goal = Some(goal);
            }
            if let Some(next) = unit.path.front().copied() {
                let candidate = unit
                    .position
                    .step_towards(next, unit.kind.stats().speed_per_tick);
                if self.map.contains_point(candidate) {
                    unit.position = candidate;
                }
                if unit.position == next {
                    unit.path.pop_front();
                }
            }
        }
        // All attackers alive at the start of the tick resolve simultaneously.
        // A Vec-index reference stays valid until the later death-removal phase.
        for (source, target, amount) in hits {
            let source_id = self.units[source].id;
            let target = &mut self.units[target];
            let damage = amount.min(target.hp.max(0));
            target.hp -= damage;
            if damage > 0 {
                self.events.push(SimEvent::Hit {
                    source: source_id,
                    target: target.id,
                    damage,
                    position: target.position,
                });
            }
        }
        self.resolve_deaths();
        self.update_objectives();
        self.update_outcome();
    }

    pub fn save_json(&self) -> Result<String, SaveError> {
        serde_json::to_string(self).map_err(|e| SaveError(e.to_string()))
    }

    pub fn load_json(json: &str) -> Result<Self, SaveError> {
        let world: Self =
            serde_json::from_str(json).map_err(|e| SaveError(format!("Invalid save JSON: {e}")))?;
        world.validate()?;
        Ok(world)
    }

    pub fn checksum(&self) -> u64 {
        // Stable integer state + stable Vec order, independent of HashMap or GPU.
        // Diagnostic only: published artifact hashes use SHA-256 separately.
        stable_hash(
            self.save_json()
                .expect("integer simulation serializes")
                .as_bytes(),
        )
    }

    pub fn validate(&self) -> Result<(), SaveError> {
        let fail = |message: &str| SaveError(message.to_owned());
        if self.version != SAVE_VERSION {
            return Err(fail("Unsupported save version."));
        }
        if !self.map.validate() {
            return Err(fail(
                "Save navigation grid dimensions do not match its cells.",
            ));
        }
        if self.wave > TOTAL_WAVES
            || self.commander_cooldown > ABILITY_COOLDOWN
            || self.wave_ticks_remaining > WAVE_INTERVAL
        {
            return Err(fail("Save has invalid wave or cooldown counters."));
        }
        let mut ids = BTreeSet::new();
        for unit in &self.units {
            if unit.id == 0 || !ids.insert(unit.id) || unit.id >= self.next_id {
                return Err(fail("Save has duplicate or invalid unit identities."));
            }
            if unit.hp <= 0
                || unit.max_hp != unit.kind.stats().hp
                || unit.hp > unit.max_hp
                || unit.attack_cooldown > unit.kind.stats().attack_interval
            {
                return Err(fail("Save has invalid unit health or attack state."));
            }
            if !self.map.contains_point(unit.position)
                || unit.target.is_some_and(|p| !self.map.contains_point(p))
                || unit.path_goal.is_some_and(|p| !self.map.contains_point(p))
            {
                return Err(fail(
                    "Save has a unit or target outside traversable terrain.",
                ));
            }
            let mut previous = unit.position;
            for point in &unit.path {
                let (x, z) = (
                    previous.x.div_euclid(CELL_SIZE),
                    previous.z.div_euclid(CELL_SIZE),
                );
                let (nx, nz) = (point.x.div_euclid(CELL_SIZE), point.z.div_euclid(CELL_SIZE));
                if !self.map.contains_point(*point)
                    || (x - nx).abs() > 1
                    || (z - nz).abs() > 1
                    || (x != nx
                        && z != nz
                        && (!self.map.is_walkable(x, nz) || !self.map.is_walkable(nx, z)))
                {
                    return Err(fail("Save contains an invalid navigation path."));
                }
                previous = *point;
            }
        }
        if self
            .units
            .iter()
            .filter(|u| u.team == Team::Player && u.kind == UnitKind::Commander)
            .count()
            != usize::from(self.status != MatchStatus::Defeat)
        {
            return Err(fail("Save has an inconsistent commander/outcome state."));
        }
        let mut objective_ids = BTreeSet::new();
        for objective in &self.objectives {
            if !objective_ids.insert(objective.id)
                || !self.map.contains_point(objective.position)
                || objective.progress > CAPTURE_TICKS
                || objective.captured != (objective.progress == CAPTURE_TICKS)
            {
                return Err(fail("Save has an invalid relic capture state."));
            }
        }
        if self.objectives.is_empty() {
            return Err(fail("Save has no victory objectives."));
        }
        if self.status == MatchStatus::Victory
            && (self.wave < TOTAL_WAVES
                || self.units.iter().any(|u| u.team == Team::Enemy)
                || !self.objectives.iter().all(|o| o.captured))
        {
            return Err(fail("Save victory is inconsistent with the battle state."));
        }
        if self.rng == 0 {
            return Err(fail("Save has an invalid random generator state."));
        }
        Ok(())
    }

    fn validate_selection(&self, mut ids: Vec<UnitId>) -> Result<Vec<UnitId>, CommandError> {
        ids.sort_unstable();
        ids.dedup();
        if ids.is_empty() {
            return Err(CommandError::EmptySelection);
        }
        for id in &ids {
            let unit = self
                .units
                .iter()
                .find(|u| u.id == *id && u.hp > 0)
                .ok_or(CommandError::UnknownUnit(*id))?;
            if unit.team != Team::Player {
                return Err(CommandError::NotPlayerUnit(*id));
            }
        }
        Ok(ids)
    }

    fn spawn(
        &mut self,
        kind: UnitKind,
        team: Team,
        position: Point,
    ) -> Result<UnitId, CommandError> {
        let id = self.next_id;
        self.next_id = self
            .next_id
            .checked_add(1)
            .ok_or(CommandError::IdExhausted)?;
        let stats = kind.stats();
        self.units.push(Unit {
            id,
            kind,
            team,
            position,
            hp: stats.hp,
            max_hp: stats.hp,
            target: None,
            order: Order::Idle,
            path: VecDeque::new(),
            path_goal: None,
            attack_cooldown: 0,
        });
        Ok(id)
    }

    fn random(&mut self) -> u32 {
        self.rng ^= self.rng << 13;
        self.rng ^= self.rng >> 7;
        self.rng ^= self.rng << 17;
        (self.rng >> 16) as u32
    }

    fn spawn_wave(&mut self) {
        self.wave += 1;
        self.wave_ticks_remaining = if self.wave < TOTAL_WAVES {
            WAVE_INTERVAL
        } else {
            0
        };
        let count = 6 + self.wave * 3;
        for index in 0..count {
            let base_z = [6_500, 18_500, 28_500][(self.wave as usize + index as usize) % 3];
            let jitter = (self.random() % 2_400) as i32 - 1_200;
            let position = self.map.nearest_walkable(Point::new(
                43_500 + (index % 3) as i32 * 600,
                base_z + jitter,
            ));
            if let Some(position) = position {
                let kind = if self.wave >= 2 && index % 5 == 0 {
                    UnitKind::Brute
                } else {
                    UnitKind::Reaver
                };
                // Exhausting u32 IDs is a real identity-space limit, not an RTS
                // population cap. The default scenario cannot approach it.
                if self.spawn(kind, Team::Enemy, position).is_err() {
                    break;
                }
            }
        }
        self.events.push(SimEvent::Wave { number: self.wave });
    }

    fn nearest_enemy(&self, index: usize, radius: i32) -> Option<usize> {
        let unit = &self.units[index];
        let mut best: Option<(u64, UnitId, usize)> = None;
        self.spatial
            .for_each_candidate(unit.position, radius, |candidate| {
                let other = &self.units[candidate];
                if other.team == unit.team || other.hp <= 0 {
                    return;
                }
                let distance = unit.position.distance_squared(other.position);
                if distance > (radius as u64).pow(2) {
                    return;
                }
                let key = (distance, other.id, candidate);
                if best.is_none_or(|previous| key < previous) {
                    best = Some(key);
                }
            });
        best.map(|(_, _, index)| index)
    }

    fn resolve_deaths(&mut self) {
        for unit in self.units.iter().filter(|u| u.hp <= 0) {
            if unit.team == Team::Enemy {
                self.economy.kills = self.economy.kills.saturating_add(1);
                self.economy.essence = self
                    .economy
                    .essence
                    .saturating_add(if unit.kind == UnitKind::Brute { 12 } else { 5 });
            }
            self.events.push(SimEvent::Died {
                unit: unit.id,
                position: unit.position,
                team: unit.team,
            });
        }
        self.units.retain(|u| u.hp > 0);
    }

    fn update_objectives(&mut self) {
        for objective in &mut self.objectives {
            if objective.captured {
                continue;
            }
            let mut allied = false;
            let mut enemy = false;
            for unit in &self.units {
                if unit.position.distance_squared(objective.position)
                    <= (CAPTURE_RADIUS as u64).pow(2)
                {
                    if unit.team == Team::Player {
                        allied = true;
                    } else {
                        enemy = true;
                    }
                }
            }
            if allied && !enemy {
                objective.progress += 1;
            } else if !allied {
                objective.progress = objective.progress.saturating_sub(2);
            }
            if objective.progress == CAPTURE_TICKS {
                objective.captured = true;
                self.economy.essence = self.economy.essence.saturating_add(80);
                self.events.push(SimEvent::Captured {
                    objective: objective.id,
                    position: objective.position,
                });
            }
        }
        if self.tick.is_multiple_of(u64::from(TICK_HZ)) {
            let income = self.objectives.iter().filter(|o| o.captured).count() as u32 * 2;
            self.economy.essence = self.economy.essence.saturating_add(income);
        }
    }

    fn update_outcome(&mut self) {
        if self.commander().is_none() {
            self.status = MatchStatus::Defeat;
        } else if self.wave == TOTAL_WAVES
            && self.objectives.iter().all(|o| o.captured)
            && !self.units.iter().any(|u| u.team == Team::Enemy)
        {
            self.status = MatchStatus::Victory;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(world: &mut World, ticks: u32) {
        for _ in 0..ticks {
            world.step();
        }
    }

    #[test]
    fn same_seed_and_command_stream_produce_identical_replay() {
        let mut a = World::new(73);
        let mut b = World::new(73);
        for tick in 0..1_500 {
            if tick % 300 == 0 {
                let command = Command::Move {
                    units: a.player_ids(),
                    target: Point::new(24_500, 18_500),
                    attack_move: true,
                };
                assert_eq!(a.apply(command.clone()), b.apply(command));
            }
            a.step();
            b.step();
            if tick % 75 == 0 {
                assert_eq!(a.checksum(), b.checksum());
            }
        }
        assert!(a.economy.kills > 0);
    }

    #[test]
    fn save_resume_has_identical_future_state() {
        let mut original = World::new(901);
        original
            .apply(Command::Move {
                units: original.player_ids(),
                target: Point::new(20_500, 6_500),
                attack_move: true,
            })
            .unwrap();
        run(&mut original, 370);
        let mut restored = World::load_json(&original.save_json().unwrap()).unwrap();
        assert_eq!(original.checksum(), restored.checksum());
        run(&mut original, 900);
        run(&mut restored, 900);
        assert_eq!(original.checksum(), restored.checksum());
    }

    #[test]
    fn orders_are_transactional_and_enemy_control_is_rejected() {
        let mut world = World::new(1);
        let before = world.checksum();
        let enemy = world
            .units
            .iter()
            .find(|u| u.team == Team::Enemy)
            .unwrap()
            .id;
        assert_eq!(
            world.apply(Command::Move {
                units: vec![1, enemy],
                target: Point::new(10_500, 18_500),
                attack_move: false
            }),
            Err(CommandError::NotPlayerUnit(enemy))
        );
        assert_eq!(world.checksum(), before);
        assert_eq!(
            world.apply(Command::Hold { units: Vec::new() }),
            Err(CommandError::EmptySelection)
        );
    }

    #[test]
    fn command_json_contract_is_stable() {
        let command: Command = serde_json::from_str(
            r#"{"type":"move","units":[1],"target":{"x":18000,"z":18000},"attack_move":true}"#,
        )
        .unwrap();
        assert!(matches!(
            command,
            Command::Move {
                attack_move: true,
                ..
            }
        ));
        assert_eq!(
            serde_json::to_string(&UnitKind::Vanguard).unwrap(),
            "\"vanguard\""
        );
    }

    #[test]
    fn relics_capture_only_when_uncontested_and_pay_income() {
        let mut world = World::new(1);
        let target = world.objectives[0].position;
        world.units[0].position = target;
        let enemy_id = world.spawn(UnitKind::Brute, Team::Enemy, target).unwrap();
        world.update_objectives();
        assert_eq!(world.objectives[0].progress, 0);
        world.units.retain(|u| u.id != enemy_id);
        for _ in 0..CAPTURE_TICKS {
            world.tick += 1;
            world.update_objectives();
        }
        assert!(world.objectives[0].captured);
        assert!(world.economy.essence >= 170);
    }

    #[test]
    fn recruitment_spends_exact_cost_and_rejects_invalid_kind() {
        let mut world = World::new(2);
        let count = world.units.len();
        world
            .apply(Command::Recruit {
                kind: UnitKind::Ranger,
            })
            .unwrap();
        assert_eq!(world.units.len(), count + 1);
        assert_eq!(world.economy.essence, 55);
        assert_eq!(
            world.apply(Command::Recruit {
                kind: UnitKind::Commander
            }),
            Err(CommandError::InvalidRecruit)
        );
        world.economy.essence = 0;
        assert_eq!(
            world.apply(Command::Recruit {
                kind: UnitKind::Vanguard
            }),
            Err(CommandError::InsufficientEssence)
        );
    }

    #[test]
    fn pulse_damage_and_cooldown_are_authoritative() {
        let mut world = World::new(3);
        let position = world.commander().unwrap().position;
        let enemy = world
            .spawn(UnitKind::Reaver, Team::Enemy, position)
            .unwrap();
        world.apply(Command::Cast { target: position }).unwrap();
        assert!(!world.units.iter().any(|u| u.id == enemy));
        assert_eq!(world.economy.kills, 1);
        assert_eq!(world.commander_cooldown, ABILITY_COOLDOWN);
        assert_eq!(
            world.apply(Command::Cast { target: position }),
            Err(CommandError::AbilityCoolingDown)
        );
        world.step();
        assert_eq!(world.commander_cooldown, ABILITY_COOLDOWN - 1);
    }

    #[test]
    fn victory_and_defeat_stop_time_and_reject_orders() {
        let mut world = World::new(4);
        world.units.retain(|u| u.team == Team::Player);
        for objective in &mut world.objectives {
            objective.captured = true;
            objective.progress = CAPTURE_TICKS;
        }
        world.wave = TOTAL_WAVES;
        world.wave_ticks_remaining = 0;
        world.step();
        assert_eq!(world.status, MatchStatus::Victory);
        let tick = world.tick;
        world.step();
        assert_eq!(world.tick, tick);
        assert_eq!(
            world.apply(Command::Recruit {
                kind: UnitKind::Vanguard
            }),
            Err(CommandError::MatchFinished)
        );
        world.validate().unwrap();
        let mut lost = World::new(5);
        lost.units[0].hp = 0;
        lost.resolve_deaths();
        lost.update_outcome();
        assert_eq!(lost.status, MatchStatus::Defeat);
        lost.validate().unwrap();
    }

    #[test]
    fn malformed_saves_reject_bad_grid_and_duplicate_id_without_panicking() {
        let world = World::new(6);
        let mut value = serde_json::to_value(&world).unwrap();
        value["map"]["blocked"] = serde_json::json!([false]);
        assert!(World::load_json(&value.to_string()).is_err());
        let mut value = serde_json::to_value(&world).unwrap();
        value["units"][1]["id"] = value["units"][0]["id"].clone();
        assert!(World::load_json(&value.to_string()).is_err());
        let mut value = serde_json::to_value(&world).unwrap();
        value["version"] = serde_json::json!(9);
        assert!(World::load_json(&value.to_string()).is_err());
    }

    #[test]
    fn all_initial_units_and_navigation_stay_valid() {
        let mut world = World::new(7);
        world.validate().unwrap();
        world
            .apply(Command::Move {
                units: world.player_ids(),
                target: Point::new(38_500, 28_500),
                attack_move: true,
            })
            .unwrap();
        for tick in 0..2_500 {
            world.step();
            if tick % 100 == 0 {
                world.validate().unwrap();
            }
        }
        assert!(world.wave >= 2 || world.status == MatchStatus::Defeat);
    }
}
