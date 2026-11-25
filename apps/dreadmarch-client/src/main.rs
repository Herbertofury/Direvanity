//! Dreadmarch's native presentation. The authoritative game state lives in dread-sim.
//! All geometry and visual effects below are original procedural game assets.
use bevy::{
    app::{AppExit, ScheduleRunnerPlugin},
    camera::{CameraUpdateSystems, RenderTarget, ScalingMode, ShadowLodOrigin},
    input::{ButtonState, keyboard::KeyboardInput, mouse::MouseWheel},
    picking::hover::Hovered,
    post_process::bloom::Bloom,
    prelude::*,
    render::{
        render_resource::{TextureFormat, TextureUsages},
        view::screenshot::{Screenshot, ScreenshotCaptured, save_to_disk},
    },
    transform::TransformSystems,
    ui::{IsDefaultUiCamera, Pressed, UiSystems},
    ui_widgets::{Activate, Button as UiButton},
    window::{ExitCondition, PrimaryWindow},
    winit::WinitPlugin,
};
use dread_sim::rpg::{self, CharacterCreation};
use dread_sim::{
    ABILITY_RADIUS, CAPTURE_RADIUS, CAPTURE_TICKS, Command as SimCommand, MatchStatus, Point,
    SimEvent, TICK_HZ, TOTAL_WAVES, Team, UnitKind, World as SimWorld,
};
use std::collections::{HashMap, HashSet};
use std::f32::consts::{FRAC_PI_2, PI};
use std::time::Duration;

const SEED: u64 = 0xD4EA_DA4C;
const GOLD: Color = Color::srgb(0.90, 0.69, 0.34);
const TEAL: Color = Color::srgb(0.28, 0.93, 0.81);
const UI_BG: Color = Color::srgba(0.025, 0.038, 0.046, 0.96);

#[derive(Resource)]
struct Battle {
    world: SimWorld,
    selected: HashSet<u32>,
    paused: bool,
    help: bool,
    drag_start: Option<Vec2>,
    cursor_world: Option<Point>,
    notice: String,
    notice_ticks: u32,
    cast_mode: bool,
    order_marker: Option<(Vec3, f32)>,
    spell_effect: Option<(Vec3, f32)>,
    impacts: Vec<Impact>,
    living_ids: HashSet<u32>,
    command_count: u32,
    command_failures: u32,
    creator: Option<CharacterCreation>,
    editing_name: bool,
    panel: CampaignPanel,
    skill_index: usize,
    item_index: usize,
    scene_generation: u64,
    ui_keyboard_consumed: bool,
}
struct Impact {
    position: Vec3,
    lifetime: f32,
    death: bool,
}
impl Default for Battle {
    fn default() -> Self {
        let world = SimWorld::new(SEED);
        let selected = world
            .units
            .iter()
            .filter(|u| u.team == Team::Player)
            .map(|u| u.id)
            .collect();
        Self {
            world,
            selected,
            paused: false,
            help: false,
            drag_start: None,
            cursor_world: None,
            notice: "Secure the three relics. Keep your commander alive.".into(),
            notice_ticks: 180,
            cast_mode: false,
            order_marker: None,
            spell_effect: None,
            impacts: Vec::new(),
            living_ids: HashSet::new(),
            command_count: 0,
            command_failures: 0,
            creator: None,
            editing_name: false,
            panel: CampaignPanel::Closed,
            skill_index: 0,
            item_index: 0,
            scene_generation: 1,
            ui_keyboard_consumed: false,
        }
    }
}
impl Battle {
    fn campaign(creation: CharacterCreation, show_creator: bool) -> Self {
        let world = SimWorld::new_campaign(SEED, creation.clone())
            .expect("initial campaign character must be valid");
        let mut battle = Self::default();
        battle.replace_world(world);
        if show_creator {
            battle.creator = Some(creation);
            battle.paused = true;
        }
        battle
    }
    fn replace_world(&mut self, world: SimWorld) {
        self.world = world;
        self.all_army();
        self.creator = None;
        self.editing_name = false;
        self.panel = CampaignPanel::Closed;
        self.skill_index = 0;
        self.item_index = 0;
        self.paused = false;
        self.cast_mode = false;
        self.drag_start = None;
        self.impacts.clear();
        self.spell_effect = None;
        self.order_marker = None;
        self.scene_generation = self.scene_generation.wrapping_add(1);
        self.notice = "Journey ready. Speak with Archivist Maelin.".into();
        self.notice_ticks = 180;
    }
    fn restart(&mut self) {
        let world = if let Some(c) = &self.world.campaign {
            SimWorld::new_campaign(SEED, c.character.clone()).expect("validated character")
        } else {
            SimWorld::new(SEED)
        };
        self.replace_world(world);
    }
    fn begin_journey(&mut self) {
        let Some(draft) = self.creator.clone() else {
            return;
        };
        match SimWorld::new_campaign(SEED, draft) {
            Ok(world) => self.replace_world(world),
            Err(error) => {
                self.notice = format!("{error:?}");
                self.notice_ticks = 180;
            }
        }
    }
    fn selected_skill(&self) -> Option<&rpg::SkillDefinition> {
        self.world
            .campaign
            .as_ref()
            .and_then(|c| c.class().skills.get(self.skill_index))
    }
    fn cast_selected(&mut self, target: Point) {
        if self.world.campaign.is_some() {
            if let Some(skill) = self.selected_skill() {
                let skill = skill.id.clone();
                self.command(SimCommand::CastSkill { skill, target });
            }
        } else {
            self.command(SimCommand::Cast { target });
        }
    }
    fn learn_selected(&mut self) {
        if let Some(skill) = self.selected_skill() {
            let skill = skill.id.clone();
            self.command(SimCommand::LearnSkill { skill });
        }
    }
    fn equip_selected(&mut self) {
        if let Some(item) = self
            .world
            .campaign
            .as_ref()
            .and_then(|c| c.inventory.get(self.item_index))
        {
            let item = item.id;
            self.command(SimCommand::Equip { item });
        }
    }
    fn nearest_interaction(&self) -> Option<(String, String, u64)> {
        let c = self.world.campaign.as_ref()?;
        let hero = self.world.commander()?;
        rpg::points_of_interest(&c.region)
            .into_iter()
            .map(|p| (p.id, p.name, hero.position.distance_squared(p.position)))
            .chain(c.ground_loot.iter().map(|l| {
                (
                    format!("loot:{}", l.item.id),
                    l.item.name.clone(),
                    hero.position.distance_squared(l.position),
                )
            }))
            .min_by_key(|(_, _, distance)| *distance)
    }
    fn interact_nearest(&mut self) {
        if let Some((target, _, _)) = self.nearest_interaction() {
            self.command(SimCommand::Interact { target });
        }
    }
    fn cycle_selection(&mut self, forward: bool) {
        if let Some(c) = &self.world.campaign {
            let (index, count) = if self.panel == CampaignPanel::Inventory {
                (&mut self.item_index, c.inventory.len())
            } else {
                (&mut self.skill_index, c.class().skills.len())
            };
            if count > 0 {
                *index = (*index + if forward { 1 } else { count - 1 }) % count;
            }
        }
    }
    fn command(&mut self, command: SimCommand) {
        self.command_count += 1;
        let previous_region = self.world.campaign.as_ref().map(|c| c.region.clone());
        let cast = match &command {
            SimCommand::Cast { target } | SimCommand::CastSkill { target, .. } => Some(*target),
            _ => None,
        };
        match self.world.apply(command) {
            Ok(()) => {
                if previous_region != self.world.campaign.as_ref().map(|c| c.region.clone()) {
                    self.scene_generation = self.scene_generation.wrapping_add(1);
                }
                if let Some(target) = cast {
                    self.spell_effect = Some((point3(target, 0.14), 0.85));
                    self.notice = if self.world.campaign.is_some() {
                        "Class skill unleashed."
                    } else {
                        "Soulfire unleashed."
                    }
                    .into();
                    self.notice_ticks = 75;
                }
            }
            Err(error) => {
                self.command_failures += 1;
                self.notice = format!("{error}");
                self.notice_ticks = 120;
            }
        }
    }
    fn selected_ids(&self) -> Vec<u32> {
        let mut ids: Vec<_> = self.selected.iter().copied().collect();
        ids.sort_unstable();
        ids
    }
    fn all_army(&mut self) {
        self.selected = self
            .world
            .units
            .iter()
            .filter(|u| u.team == Team::Player && u.hp > 0)
            .map(|u| u.id)
            .collect();
    }
    fn advance(&mut self) {
        self.world.step();
        self.living_ids.clear();
        self.living_ids.extend(
            self.world
                .units
                .iter()
                .filter(|u| u.team == Team::Player && u.hp > 0)
                .map(|u| u.id),
        );
        self.selected.retain(|id| self.living_ids.contains(id));
        for event in &self.world.events {
            match *event {
                SimEvent::Hit { position, .. } => self.impacts.push(Impact {
                    position: point3(position, 0.8),
                    lifetime: 0.18,
                    death: false,
                }),
                SimEvent::Died { position, .. } => self.impacts.push(Impact {
                    position: point3(position, 0.4),
                    lifetime: 0.6,
                    death: true,
                }),
                SimEvent::Captured { .. } => {
                    self.notice = "Relic secured. Essence income increased.".into();
                    self.notice_ticks = 120;
                }
                _ => {}
            }
        }
    }
}

#[derive(Resource, Default)]
struct Cli {
    smoke: bool,
    campaign_smoke: bool,
    campaign_stage: u8,
    save_path: String,
    screenshot: Option<String>,
    frames: u32,
    captured: bool,
    creator_captured: bool,
    hud_validated: bool,
}
#[derive(Resource)]
struct OffscreenTarget(Handle<Image>);
#[derive(Resource)]
struct Orbit {
    focus: Vec3,
    yaw: f32,
    height: f32,
}
impl Default for Orbit {
    fn default() -> Self {
        Self {
            focus: Vec3::new(23.0, 0.0, 18.0),
            yaw: -0.65,
            height: 35.5,
        }
    }
}
#[derive(Resource)]
struct VisualAssets {
    cube: Handle<Mesh>,
    sphere: Handle<Mesh>,
    cone: Handle<Mesh>,
    ring: Handle<Mesh>,
    steel: Handle<StandardMaterial>,
    dark: Handle<StandardMaterial>,
    gold: Handle<StandardMaterial>,
    teal: Handle<StandardMaterial>,
    red: Handle<StandardMaterial>,
    ember: Handle<StandardMaterial>,
    cloth: Handle<StandardMaterial>,
    bone: Handle<StandardMaterial>,
    stone: Handle<StandardMaterial>,
    basalt: Handle<StandardMaterial>,
    moss: Handle<StandardMaterial>,
    pale: Handle<StandardMaterial>,
}
#[derive(Component)]
struct MainCamera;
#[derive(Component)]
struct UnitVisual {
    id: u32,
    kind: UnitKind,
    team: Team,
    previous: Vec3,
    current: Vec3,
    tick: u64,
    appearance: Option<CharacterCreation>,
}
#[derive(Default)]
struct RenderScratch {
    indices: HashMap<u32, usize>,
    existing: HashSet<u32>,
}
#[derive(Component)]
struct RelicVisual(u32);
#[derive(Component)]
struct Hud;
#[derive(Component)]
struct StatusLine;
#[derive(Component)]
struct HelpText;
type HelpFilter = (With<HelpText>, Without<Hud>, Without<StatusLine>);
#[derive(Component)]
struct DragRect;
#[derive(Component)]
struct RegionGeometry;
#[derive(Component)]
struct PoiLabel(Point);
#[derive(Component)]
struct CampaignControls;
#[derive(Component)]
struct SkirmishControls;
#[derive(Component)]
struct CreatorPanel;
#[derive(Component)]
struct CreatorSummary;
type CampaignPanelFilter = Or<(
    With<CreatorPanel>,
    With<CampaignControls>,
    With<SkirmishControls>,
)>;
type CampaignPanelData<'a> = (
    &'a mut Node,
    Has<CreatorPanel>,
    Has<CampaignControls>,
    Has<SkirmishControls>,
);
#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum CampaignPanel {
    #[default]
    Closed,
    Skills,
    Inventory,
}
#[derive(Component, Clone, Copy, PartialEq, Eq)]
enum Action {
    Vanguard,
    Ranger,
    Spell,
    Army,
    Hold,
    Pause,
    Restart,
    Help,
    Skills,
    Inventory,
    Previous,
    Next,
    Learn,
    Equip,
    Interact,
    Respec,
    Creator,
    ChooseClass(usize),
    EditName,
    Body,
    Skin,
    Hair,
    HairStyle,
    Eyes,
    Accent,
    Stature,
    Begin,
    CloseCreator,
}

fn main() {
    let mut cli = Cli::default();
    let mut skirmish = false;
    let mut creation = CharacterCreation::default();
    let mut load_path = None;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--smoke-test" => {
                cli.smoke = true;
                skirmish = true;
            }
            "--campaign-smoke" => {
                cli.smoke = true;
                cli.campaign_smoke = true;
            }
            "--skirmish" => skirmish = true,
            "--campaign" => skirmish = false,
            "--load" | "--save" | "--class" | "--name" => {
                let value = args.next().unwrap_or_else(|| {
                    eprintln!("{arg} requires a value");
                    std::process::exit(2);
                });
                match arg.as_str() {
                    "--load" => load_path = Some(value),
                    "--save" => cli.save_path = value,
                    "--class" => creation.class_id = value,
                    _ => creation.name = value,
                }
            }
            "--screenshot" => {
                cli.screenshot = args.next();
                if cli.screenshot.is_none() {
                    eprintln!("--screenshot requires a path");
                    std::process::exit(2);
                }
            }
            "--help" | "-h" => {
                println!(
                    "Dreadmarch native campaign foundation\n  Default: character creator and single-player campaign\n  --class ID --name NAME  seed the editable creator\n  --load PATH            load a validated exported campaign or skirmish save\n  --save PATH            choose the F5/F9 save file\n  --campaign-smoke       exercise native creator, campaign, skills and saves\n  --skirmish             original tactical mode\n  --smoke-test           original tactical rendering regression\n  --screenshot PATH      capture PNG after scene warmup\nCampaign: right-click/WASD move; Space selected skill; 1-6 learned skills; J interact/loot/travel; K skills; I inventory; [/] select; L learn; U equip; T retrain in town; F2 creator; Q/E orbit; wheel zoom; P pause; F5/F9 save/load; F1 help."
                );
                return;
            }
            _ => {
                eprintln!("Unknown argument: {arg}; use --help");
                std::process::exit(2);
            }
        }
    }
    if cli.campaign_smoke {
        skirmish = false;
    }
    if let Err(error) = creation.validate() {
        eprintln!("Invalid character: {error}");
        std::process::exit(2);
    }
    let mut battle = if skirmish {
        Battle::default()
    } else {
        Battle::campaign(creation, true)
    };
    if let Some(path) = load_path {
        let world = std::fs::read_to_string(&path)
            .map_err(|e| format!("{e}"))
            .and_then(|text| SimWorld::load_json(&text).map_err(|e| format!("{e:?}")))
            .unwrap_or_else(|error| {
                eprintln!("Cannot load {path}: {error}");
                std::process::exit(2);
            });
        battle.replace_world(world);
    }
    if cli.campaign_smoke && battle.world.campaign.is_none() {
        eprintln!("--campaign-smoke requires a campaign world");
        std::process::exit(2);
    }
    if cli.campaign_smoke && battle.creator.is_none() {
        battle.creator = battle.world.campaign.as_ref().map(|c| c.character.clone());
        battle.paused = true;
    }
    if cli.save_path.is_empty() {
        cli.save_path = if battle.world.campaign.is_some() {
            "dreadmarch-campaign.json"
        } else {
            "dreadmarch-save.json"
        }
        .into();
    }
    let mut orbit = Orbit::default();
    if battle.world.campaign.is_some() {
        if let Some(hero) = battle.world.commander() {
            orbit.focus = point3(hero.position, 0.0);
        }
        orbit.height = if battle.creator.is_some() { 11.0 } else { 24.0 };
    }
    let smoke = cli.smoke;
    let mut app = App::new();
    app.insert_resource(cli)
        .insert_resource(battle)
        .insert_resource(orbit)
        .insert_resource(Time::<Fixed>::from_hz(f64::from(TICK_HZ)))
        .insert_resource(ClearColor(Color::srgb(0.025, 0.041, 0.05)))
        .insert_resource(GlobalAmbientLight {
            color: Color::srgb(0.45, 0.65, 0.75),
            brightness: 300.0,
            ..default()
        });
    let plugins = DefaultPlugins.set(WindowPlugin {
        primary_window: (!smoke).then(|| Window {
            title: "DREADMARCH | The Ashen Concord - single-player foundation".into(),
            resolution: (1440, 900).into(),
            present_mode: bevy::window::PresentMode::AutoNoVsync,
            ..default()
        }),
        exit_condition: if smoke {
            ExitCondition::DontExit
        } else {
            ExitCondition::OnAllClosed
        },
        ..default()
    });
    if smoke {
        app.add_plugins(plugins.disable::<WinitPlugin>())
            .add_plugins(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                1.0 / 60.0,
            )));
    } else {
        app.add_plugins(plugins);
    }
    app.add_systems(Startup, (setup_scene, setup_ui).chain())
        .add_systems(FixedUpdate, step_simulation)
        .add_observer(activate_button)
        .add_systems(
            Update,
            (
                smoke_driver,
                camera_input,
                keyboard_input.run_if(interactive),
                pointer_input.run_if(interactive),
                button_input.run_if(interactive),
                sync_region_geometry,
                update_units,
                update_hud,
                update_campaign_ui,
                draw_tactical,
            )
                .chain(),
        )
        .add_systems(
            PostUpdate,
            update_poi_labels
                .after(TransformSystems::Propagate)
                .after(CameraUpdateSystems)
                .before(UiSystems::Layout),
        )
        .run();
}
fn interactive(cli: Res<Cli>) -> bool {
    !cli.smoke
}

fn point3(point: Point, height: f32) -> Vec3 {
    Vec3::new(point.x as f32 / 1000.0, height, point.z as f32 / 1000.0)
}
fn point2(world: Vec3) -> Point {
    Point {
        x: (world.x * 1000.0).round() as i32,
        z: (world.z * 1000.0).round() as i32,
    }
}
fn material(
    materials: &mut Assets<StandardMaterial>,
    color: Color,
    metal: f32,
) -> Handle<StandardMaterial> {
    materials.add(StandardMaterial {
        base_color: color,
        metallic: metal,
        perceptual_roughness: 0.82 - metal * 0.4,
        ..default()
    })
}
fn piece(
    commands: &mut Commands,
    mesh: Handle<Mesh>,
    mat: Handle<StandardMaterial>,
    pos: Vec3,
    scale: Vec3,
) -> Entity {
    commands
        .spawn((
            Mesh3d(mesh),
            MeshMaterial3d(mat),
            Transform::from_translation(pos).with_scale(scale),
        ))
        .id()
}
fn noise(x: i32, z: i32) -> f32 {
    let mut n =
        (x as u32).wrapping_mul(0x9E37_79B9) ^ (z as u32).wrapping_mul(0x85EB_CA6B) ^ 0xDEAD_CAFE;
    n ^= n >> 16;
    n = n.wrapping_mul(0x7FEB_352D);
    n ^= n >> 15;
    (n & 65535) as f32 / 65535.0
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    orbit: Res<Orbit>,
    cli: Res<Cli>,
) {
    let cube = meshes.add(Cuboid::default());
    let sphere = meshes.add(Sphere::new(0.5).mesh().ico(2).expect("valid ico sphere"));
    let cone = meshes.add(Cone::new(0.5, 1.0));
    let ring = meshes.add(Torus::new(0.38, 0.46));
    let stone = material(&mut materials, Color::srgb(0.18, 0.21, 0.21), 0.0);
    let earth = material(&mut materials, Color::srgb(0.105, 0.14, 0.15), 0.0);
    let basalt = material(&mut materials, Color::srgb(0.075, 0.10, 0.115), 0.15);
    let moss = material(&mut materials, Color::srgb(0.14, 0.21, 0.17), 0.0);
    let pale = material(&mut materials, Color::srgb(0.29, 0.31, 0.28), 0.05);
    let steel = material(&mut materials, Color::srgb(0.48, 0.60, 0.62), 0.8);
    let dark = material(&mut materials, Color::srgb(0.055, 0.07, 0.09), 0.35);
    let gold = material(&mut materials, GOLD, 0.7);
    let teal = materials.add(StandardMaterial {
        base_color: TEAL,
        emissive: LinearRgba::rgb(0.2, 2.1, 1.4),
        ..default()
    });
    let ember = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.28, 0.07),
        emissive: LinearRgba::rgb(4.0, 0.38, 0.04),
        ..default()
    });
    let red = material(&mut materials, Color::srgb(0.48, 0.065, 0.06), 0.15);
    let cloth = material(&mut materials, Color::srgb(0.07, 0.30, 0.31), 0.0);
    let bone = material(&mut materials, Color::srgb(0.72, 0.67, 0.51), 0.15);
    piece(
        &mut commands,
        cube.clone(),
        basalt.clone(),
        Vec3::new(24.0, -0.65, 18.0),
        Vec3::new(50.0, 1.25, 38.0),
    );
    piece(
        &mut commands,
        cube.clone(),
        earth.clone(),
        Vec3::new(24.0, -0.03, 18.0),
        Vec3::new(48.0, 0.12, 36.0),
    );
    // Border monuments frame the playable area without adding invisible collision.
    for x in (2..48).step_by(5) {
        for z in [-1.0, 37.0] {
            let h = 2.0 + noise(x, z as i32) * 2.4;
            piece(
                &mut commands,
                cube.clone(),
                basalt.clone(),
                Vec3::new(x as f32, h / 2.0, z),
                Vec3::new(1.1, h, 1.1),
            );
            piece(
                &mut commands,
                cone.clone(),
                pale.clone(),
                Vec3::new(x as f32, h + 0.35, z),
                Vec3::new(1.25, 0.7, 1.25),
            );
            if x % 2 == 0 {
                piece(
                    &mut commands,
                    sphere.clone(),
                    ember.clone(),
                    Vec3::new(x as f32, h + 0.9, z),
                    Vec3::splat(0.3),
                );
            }
        }
    }
    // Sanctuary and invasion gate, both original symbolic architecture.
    for (x, mat) in [(4.0, teal.clone()), (44.0, ember.clone())] {
        for z in [16.0, 20.0] {
            piece(
                &mut commands,
                cube.clone(),
                dark.clone(),
                Vec3::new(x, 1.6, z),
                Vec3::new(0.8, 3.2, 0.8),
            );
            piece(
                &mut commands,
                cone.clone(),
                mat.clone(),
                Vec3::new(x, 3.65, z),
                Vec3::new(0.65, 1.0, 0.65),
            );
        }
        piece(
            &mut commands,
            cube.clone(),
            gold.clone(),
            Vec3::new(x, 3.0, 18.0),
            Vec3::new(0.5, 0.25, 4.8),
        );
        commands.spawn((
            PointLight {
                color: if x < 10.0 {
                    TEAL
                } else {
                    Color::srgb(1.0, 0.28, 0.08)
                },
                intensity: 650_000.0,
                range: 12.0,
                ..default()
            },
            Transform::from_xyz(x, 3.0, 18.0),
        ));
    }
    commands.spawn((
        DirectionalLight {
            illuminance: 10_000.0,
            shadow_maps_enabled: true,
            color: Color::srgb(0.88, 0.92, 1.0),
            ..default()
        },
        Transform::from_xyz(-15.0, 32.0, 10.0).looking_at(Vec3::new(20.0, 0.0, 18.0), Vec3::Y),
    ));
    let render_target = if cli.smoke {
        let mut image = Image::new_target_texture(1440, 900, TextureFormat::Rgba8UnormSrgb, None);
        image.texture_descriptor.usage |= TextureUsages::COPY_SRC;
        let handle = images.add(image);
        commands.insert_resource(OffscreenTarget(handle.clone()));
        RenderTarget::Image(handle.into())
    } else {
        RenderTarget::default()
    };
    commands.spawn((
        MainCamera,
        Camera3d::default(),
        IsDefaultUiCamera,
        ShadowLodOrigin,
        render_target,
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: orbit.height,
            },
            ..OrthographicProjection::default_3d()
        }),
        camera_transform(&orbit),
        Bloom {
            intensity: 0.17,
            ..Bloom::NATURAL
        },
        DistanceFog {
            color: Color::srgb(0.065, 0.11, 0.13),
            falloff: FogFalloff::Linear {
                start: 65.0,
                end: 115.0,
            },
            ..default()
        },
    ));
    commands.insert_resource(VisualAssets {
        cube,
        sphere,
        cone,
        ring,
        steel,
        dark,
        gold,
        teal,
        red,
        ember,
        cloth,
        bone,
        stone,
        basalt,
        moss,
        pale,
    });
}

fn label(text: &str, size: f32, color: Color) -> (Text, TextFont, TextColor) {
    (
        Text::new(text),
        TextFont {
            font_size: bevy::text::FontSize::Px(size),
            ..default()
        },
        TextColor(color),
    )
}

fn sync_region_geometry(
    mut commands: Commands,
    battle: Res<Battle>,
    assets: Res<VisualAssets>,
    previous: Query<Entity, With<RegionGeometry>>,
    mut generation: Local<u64>,
) {
    if *generation == battle.scene_generation {
        return;
    }
    *generation = battle.scene_generation;
    for entity in &previous {
        commands.entity(entity).despawn();
    }
    for z in 0..i32::from(battle.world.map.height) {
        for x in 0..i32::from(battle.world.map.width) {
            let n = noise(x, z);
            let center = Vec3::new(x as f32 + 0.5, 0.0, z as f32 + 0.5);
            let geometry = if !battle.world.map.is_walkable(x, z) {
                let height = 0.6 + n * 1.9;
                Some((
                    assets.cube.clone(),
                    if n > 0.65 {
                        assets.pale.clone()
                    } else {
                        assets.stone.clone()
                    },
                    center + Vec3::Y * height * 0.5,
                    Vec3::new(0.95, height, 0.95),
                ))
            } else if (z - 18).abs() < 2 || x % 12 == 0 {
                Some((
                    assets.cube.clone(),
                    if n > 0.5 {
                        assets.stone.clone()
                    } else {
                        assets.basalt.clone()
                    },
                    center + Vec3::Y * 0.045,
                    Vec3::new(0.92, 0.08, 0.92),
                ))
            } else if n > 0.9 {
                Some((
                    assets.sphere.clone(),
                    assets.moss.clone(),
                    center + Vec3::Y * 0.12,
                    Vec3::new(0.7, 0.25, 0.65),
                ))
            } else {
                None
            };
            if let Some((mesh, mat, pos, scale)) = geometry {
                let entity = piece(&mut commands, mesh, mat, pos, scale);
                commands.entity(entity).insert(RegionGeometry);
            }
            if !battle.world.map.is_walkable(x, z) && n > 0.92 {
                let cap = piece(
                    &mut commands,
                    assets.cube.clone(),
                    assets.pale.clone(),
                    center + Vec3::Y * (0.7 + n * 1.9),
                    Vec3::new(1.13, 0.2, 1.13),
                );
                commands.entity(cap).insert(RegionGeometry);
            }
        }
    }
    for objective in &battle.world.objectives {
        let pos = point3(objective.position, 0.1);
        let entity = piece(
            &mut commands,
            assets.cube.clone(),
            assets.dark.clone(),
            pos,
            Vec3::new(2.4, 0.2, 2.4),
        );
        commands.entity(entity).insert(RegionGeometry);
        commands.spawn((
            RegionGeometry,
            RelicVisual(objective.id),
            Mesh3d(assets.cone.clone()),
            MeshMaterial3d(assets.gold.clone()),
            Transform::from_translation(pos + Vec3::Y * 1.3).with_scale(Vec3::new(0.9, 2.1, 0.9)),
        ));
        let ring = piece(
            &mut commands,
            assets.ring.clone(),
            assets.gold.clone(),
            pos + Vec3::Y * 0.2,
            Vec3::splat(3.8),
        );
        commands.entity(ring).insert(RegionGeometry);
    }
    if let Some(c) = &battle.world.campaign {
        for poi in rpg::points_of_interest(&c.region) {
            let pos = point3(poi.position, 0.15);
            let (mesh, scale) = match poi.kind.as_str() {
                "person" => (assets.cone.clone(), Vec3::new(0.8, 1.6, 0.8)),
                "gate" => (assets.ring.clone(), Vec3::splat(2.4)),
                "chest" => (assets.cube.clone(), Vec3::new(1.1, 0.7, 0.7)),
                _ => (assets.cone.clone(), Vec3::new(0.8, 2.1, 0.8)),
            };
            let entity = piece(
                &mut commands,
                mesh,
                if poi.kind == "person" {
                    assets.teal.clone()
                } else {
                    assets.gold.clone()
                },
                pos + Vec3::Y * 0.7,
                scale,
            );
            commands.entity(entity).insert(RegionGeometry);
            if poi.kind == "person" {
                let head = piece(
                    &mut commands,
                    assets.sphere.clone(),
                    assets.bone.clone(),
                    pos + Vec3::Y * 1.8,
                    Vec3::splat(0.5),
                );
                commands.entity(head).insert(RegionGeometry);
            }
            commands.spawn((
                RegionGeometry,
                PoiLabel(poi.position),
                GlobalZIndex(-1),
                label(&poi.name, 12.0, GOLD),
                Node {
                    position_type: PositionType::Absolute,
                    padding: UiRect::all(px(4)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.02, 0.03, 0.04, 0.8)),
            ));
        }
    }
}

fn update_poi_labels(
    camera: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
    battle: Res<Battle>,
    mut labels: Query<(&PoiLabel, &mut Node)>,
) {
    let (camera, transform) = *camera;
    for (poi, mut node) in &mut labels {
        if battle.creator.is_some() {
            node.display = Display::None;
        } else if let Ok(p) = camera.world_to_viewport(transform, point3(poi.0, 2.4)) {
            node.left = px(p.x - 50.0);
            node.top = px(p.y);
            let size = camera
                .logical_viewport_size()
                .unwrap_or(Vec2::new(1440.0, 900.0));
            node.display = if p.x < 0.0
                || p.x > size.x
                || p.y < 95.0
                || p.y > size.y - 220.0
                || ((battle.help || battle.panel != CampaignPanel::Closed)
                    && p.x < 600.0
                    && p.y < 690.0)
            {
                Display::None
            } else {
                Display::Flex
            };
        } else {
            node.display = Display::None;
        }
    }
}
fn setup_ui(mut commands: Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: px(0),
                left: px(0),
                right: px(0),
                height: px(86),
                padding: UiRect::axes(px(24), px(15)),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(UI_BG),
        ))
        .with_children(|parent| {
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: px(3),
                    ..default()
                })
                .with_children(|p| {
                    p.spawn(label("D R E A D M A R C H", 26.0, GOLD));
                    p.spawn(label(
                        "THE ASHEN CONCORD    /    PLAYABLE FOUNDATION",
                        11.0,
                        Color::srgb(0.55, 0.68, 0.69),
                    ));
                });
            parent.spawn((Hud, label("", 16.0, Color::srgb(0.82, 0.88, 0.85))));
        });
    commands.spawn((Node { position_type:PositionType::Absolute, bottom:px(0), left:px(0), right:px(0), min_height:px(138), padding:UiRect::all(px(18)), flex_direction:FlexDirection::Column, row_gap:px(12), ..default() }, BackgroundColor(UI_BG))).with_children(|parent| {
        parent.spawn((StatusLine, label("", 15.0, TEAL)));
        parent.spawn((SkirmishControls, Node { column_gap:px(8), flex_wrap:FlexWrap::Wrap, row_gap:px(6), ..default() })).with_children(|p| {
            for (text, action) in [(format!("1  VANGUARD  {}", UnitKind::Vanguard.stats().recruitment_cost.unwrap_or(0)),Action::Vanguard),(format!("2  RANGER  {}", UnitKind::Ranger.stats().recruitment_cost.unwrap_or(0)),Action::Ranger),("SPACE  SOULFIRE".into(),Action::Spell),("TAB  ARMY".into(),Action::Army),("H  HOLD".into(),Action::Hold),("P  PAUSE".into(),Action::Pause),("R  RESTART".into(),Action::Restart),("F1  HELP".into(),Action::Help)] {
                p.spawn((UiButton, Hovered::default(), action, Node { padding:UiRect::axes(px(14),px(11)), border:UiRect::all(px(1)), ..default() }, BackgroundColor(Color::srgb(0.075, 0.105, 0.12)), BorderColor::all(Color::srgb(0.24, 0.31, 0.31)))).with_child(label(&text,13.0,GOLD));
            }
        });
        parent.spawn((CampaignControls, Node { column_gap:px(6), flex_wrap:FlexWrap::Wrap, row_gap:px(6), ..default() })).with_children(|p| {
            for (text, action) in [("SPACE  CAST", Action::Spell), ("J  INTERACT", Action::Interact), ("K  SKILLS", Action::Skills), ("[  PREV", Action::Previous), ("]  NEXT", Action::Next), ("L  LEARN", Action::Learn), ("I  INVENTORY", Action::Inventory), ("U  EQUIP", Action::Equip), ("T  RETRAIN", Action::Respec), ("P  PAUSE", Action::Pause), ("F2  CREATOR", Action::Creator), ("F1  HELP", Action::Help)] {
                p.spawn((UiButton, Hovered::default(), action, Node { padding:UiRect::axes(px(10),px(9)), border:UiRect::all(px(1)), ..default() }, BackgroundColor(Color::srgb(0.075, 0.105, 0.12)), BorderColor::all(Color::srgb(0.24, 0.31, 0.31)))).with_child(label(text,12.0,GOLD));
            }
        });
        parent.spawn(label("RIGHT move   |   SHIFT + RIGHT attack-move   |   Q/E orbit   WHEEL zoom   |   C center hero   |   F5 save   F9 load   F12 screenshot", 11.0, Color::srgb(0.54, 0.64, 0.65)));
    });
    commands.spawn((
        HelpText,
        label("", 17.0, Color::srgb(0.87, 0.9, 0.86)),
        Node {
            position_type: PositionType::Absolute,
            top: px(110),
            left: px(24),
            padding: UiRect::all(px(20)),
            max_width: px(540),
            ..default()
        },
        BackgroundColor(Color::srgba(0.025, 0.04, 0.05, 0.95)),
        Visibility::Hidden,
    ));
    commands.spawn((
        DragRect,
        Node {
            position_type: PositionType::Absolute,
            border: UiRect::all(px(1)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.3, 0.9, 0.8, 0.10)),
        BorderColor::all(TEAL),
        Visibility::Hidden,
    ));
    commands.spawn((CreatorPanel, Node {
        position_type: PositionType::Absolute, left: px(24), top: px(105), width: px(610),
        max_width: percent(95), padding: UiRect::all(px(20)), row_gap: px(10),
        flex_direction: FlexDirection::Column, ..default()
    }, GlobalZIndex(20), BackgroundColor(UI_BG), BorderColor::all(GOLD))).with_children(|p| {
        p.spawn(label("CREATE YOUR WANDERER", 24.0, GOLD));
        p.spawn((CreatorSummary, label("", 14.0, Color::srgb(0.82, 0.88, 0.85))));
        p.spawn(Node { flex_wrap: FlexWrap::Wrap, column_gap: px(5), row_gap: px(5), ..default() }).with_children(|buttons| {
            for (index, class) in rpg::catalogue().classes.iter().enumerate() {
                buttons.spawn((UiButton, Hovered::default(), Action::ChooseClass(index),
                    Node { width: px(136), padding: UiRect::all(px(7)), ..default() },
                    BackgroundColor(Color::srgb(0.075, 0.105, 0.12))))
                    .with_child(label(&class.name, 12.0, GOLD));
            }
        });
        p.spawn(Node { flex_wrap: FlexWrap::Wrap, column_gap: px(6), row_gap: px(5), ..default() }).with_children(|buttons| {
            for (text, action) in [("EDIT NAME", Action::EditName), ("BODY", Action::Body), ("SKIN", Action::Skin), ("HAIR", Action::Hair), ("HAIRSTYLE", Action::HairStyle), ("EYES", Action::Eyes), ("ACCENT", Action::Accent), ("STATURE", Action::Stature)] {
                buttons.spawn((UiButton, Hovered::default(), action,
                    Node { padding: UiRect::axes(px(10), px(8)), ..default() },
                    BackgroundColor(Color::srgb(0.075, 0.105, 0.12))))
                    .with_child(label(text, 12.0, TEAL));
            }
        });
        p.spawn(label("Appearance is independent of class. Procedural preview; production character art is still to come.", 11.0, Color::srgb(0.57, 0.67, 0.67)));
        p.spawn(Node { column_gap: px(12), ..default() }).with_children(|buttons| {
            for (text, action) in [("BEGIN NEW JOURNEY", Action::Begin), ("RETURN / CANCEL", Action::CloseCreator)] {
                buttons.spawn((UiButton, Hovered::default(), action,
                    Node { padding: UiRect::axes(px(16), px(11)), ..default() },
                    BackgroundColor(Color::srgb(0.075, 0.105, 0.12))))
                    .with_child(label(text, 14.0, GOLD));
            }
        });
    });
}

fn update_campaign_ui(
    battle: Res<Battle>,
    mut panels: Query<CampaignPanelData<'_>, CampaignPanelFilter>,
    mut summary: Single<&mut Text, With<CreatorSummary>>,
) {
    for (mut node, creator, campaign, skirmish) in &mut panels {
        let visible = (creator && battle.creator.is_some())
            || (campaign && battle.world.campaign.is_some() && battle.creator.is_none())
            || (skirmish && battle.world.campaign.is_none() && battle.creator.is_none());
        node.display = if visible {
            Display::Flex
        } else {
            Display::None
        };
    }
    if let Some(draft) = &battle.creator {
        let profile = rpg::class(&draft.class_id).expect("creator class");
        summary.0 = format!(
            "NAME: {}{}\n{}  |  {}\n{}\n\nBODY {}   STATURE {}%   HAIRSTYLE {}\nSkin {}  Hair {}  Eyes {}  Accent {}",
            draft.name,
            if battle.editing_name {
                " |  TYPE TO EDIT | ENTER TO FINISH"
            } else {
                ""
            },
            profile.name,
            profile.branches.join(" / "),
            profile.description,
            draft.body,
            draft.stature,
            draft.hair_style,
            draft.skin,
            draft.hair,
            draft.eyes,
            draft.accent
        );
        if let Err(error) = draft.validate() {
            summary.0.push_str(&format!("\n{error}"));
        }
    }
}

fn camera_transform(orbit: &Orbit) -> Transform {
    let offset = Vec3::new(orbit.yaw.sin() * 42.0, 46.0, orbit.yaw.cos() * 42.0);
    Transform::from_translation(orbit.focus + offset).looking_at(orbit.focus, Vec3::Y)
}
fn camera_input(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut wheel: MessageReader<MouseWheel>,
    mut orbit: ResMut<Orbit>,
    mut camera: Single<(&mut Transform, &mut Projection), With<MainCamera>>,
    battle: Res<Battle>,
    mut creator_was_open: Local<bool>,
) {
    let dt = time.delta_secs();
    let right = Vec3::new(orbit.yaw.cos(), 0.0, -orbit.yaw.sin());
    let forward = Vec3::new(-orbit.yaw.sin(), 0.0, -orbit.yaw.cos());
    let speed = orbit.height * 0.55 * dt;
    if battle.world.campaign.is_some() {
        if let Some(hero) = battle.world.commander() {
            orbit.focus = point3(hero.position, 0.0);
        }
        if *creator_was_open != battle.creator.is_some() {
            orbit.height = if battle.creator.is_some() { 11.0 } else { 24.0 };
            *creator_was_open = battle.creator.is_some();
        }
    }
    if keys.pressed(KeyCode::KeyW) && battle.world.campaign.is_none() {
        orbit.focus += forward * speed;
    }
    if keys.pressed(KeyCode::KeyS) && battle.world.campaign.is_none() {
        orbit.focus -= forward * speed;
    }
    if keys.pressed(KeyCode::KeyA) && battle.world.campaign.is_none() {
        orbit.focus -= right * speed;
    }
    if keys.pressed(KeyCode::KeyD) && battle.world.campaign.is_none() {
        orbit.focus += right * speed;
    }
    if keys.pressed(KeyCode::KeyQ) && !battle.editing_name {
        orbit.yaw -= dt;
    }
    if keys.pressed(KeyCode::KeyE) && !battle.editing_name {
        orbit.yaw += dt;
    }
    for event in wheel.read() {
        orbit.height = (orbit.height - event.y * 1.8).clamp(9.0, 65.0);
    }
    orbit.focus.x = orbit.focus.x.clamp(-4.0, 52.0);
    orbit.focus.z = orbit.focus.z.clamp(-4.0, 40.0);
    let (transform, projection) = &mut *camera;
    **transform = camera_transform(&orbit);
    if let Projection::Orthographic(p) = &mut **projection {
        p.scaling_mode = ScalingMode::FixedVertical {
            viewport_height: orbit.height,
        };
    }
}

fn keyboard_input(
    (keys, cli): (Res<ButtonInput<KeyCode>>, Res<Cli>),
    mut text_events: MessageReader<KeyboardInput>,
    mut battle: ResMut<Battle>,
    mut orbit: ResMut<Orbit>,
    mut exit: MessageWriter<AppExit>,
    mut commands: Commands,
    mut last_walk: Local<u64>,
) {
    if *last_walk > battle.world.tick {
        *last_walk = 0;
    }
    if battle.ui_keyboard_consumed {
        battle.ui_keyboard_consumed = false;
        text_events.clear();
        return;
    }
    if battle.creator.is_some() {
        if battle.editing_name {
            if keys.just_pressed(KeyCode::Backspace) {
                battle.creator.as_mut().expect("draft").name.pop();
            }
            if keys.pressed(KeyCode::ControlLeft) && keys.just_pressed(KeyCode::KeyA) {
                battle.creator.as_mut().expect("draft").name.clear();
            }
            for event in text_events.read() {
                if event.state == ButtonState::Pressed
                    && !keys.pressed(KeyCode::ControlLeft)
                    && !keys.pressed(KeyCode::ControlRight)
                    && let Some(text) = &event.text
                {
                    let draft = battle.creator.as_mut().expect("draft");
                    for ch in text.chars().filter(|ch| !ch.is_control()) {
                        if draft.name.chars().count() < 32 {
                            draft.name.push(ch);
                        }
                    }
                }
            }
            if keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::Escape) {
                battle.editing_name = false;
            }
        } else {
            text_events.clear();
            if keys.just_pressed(KeyCode::Enter) {
                battle.begin_journey();
            }
            if keys.just_pressed(KeyCode::Escape) {
                perform_action(Action::CloseCreator, &mut battle);
            }
        }
        return;
    }
    text_events.clear();
    if keys.just_pressed(KeyCode::Escape) {
        if battle.panel != CampaignPanel::Closed {
            battle.panel = CampaignPanel::Closed;
        } else if battle.help {
            battle.help = false;
        } else {
            exit.write(AppExit::Success);
        }
    }
    if keys.just_pressed(KeyCode::KeyC)
        && let Some(unit) = battle.world.commander()
    {
        let (id, pos) = (unit.id, point3(unit.position, 0.0));
        battle.selected.clear();
        battle.selected.insert(id);
        orbit.focus = pos;
    }
    for (key, action) in [
        (KeyCode::KeyP, Action::Pause),
        (KeyCode::F1, Action::Help),
        (KeyCode::KeyR, Action::Restart),
    ] {
        if keys.just_pressed(key) {
            perform_action(action, &mut battle);
        }
    }
    if battle.world.campaign.is_some() {
        for (key, action) in [
            (KeyCode::KeyJ, Action::Interact),
            (KeyCode::KeyK, Action::Skills),
            (KeyCode::KeyI, Action::Inventory),
            (KeyCode::BracketLeft, Action::Previous),
            (KeyCode::BracketRight, Action::Next),
            (KeyCode::KeyL, Action::Learn),
            (KeyCode::KeyU, Action::Equip),
            (KeyCode::KeyT, Action::Respec),
            (KeyCode::F2, Action::Creator),
        ] {
            if keys.just_pressed(key) {
                perform_action(action, &mut battle);
            }
        }
        for (slot, key) in [
            KeyCode::Digit1,
            KeyCode::Digit2,
            KeyCode::Digit3,
            KeyCode::Digit4,
            KeyCode::Digit5,
            KeyCode::Digit6,
        ]
        .iter()
        .enumerate()
        {
            if keys.just_pressed(*key) {
                let index = battle.world.campaign.as_ref().and_then(|c| {
                    c.class()
                        .skills
                        .iter()
                        .enumerate()
                        .filter(|(_, s)| s.kind == "active" && c.learned.contains(&s.id))
                        .nth(slot)
                        .map(|(i, _)| i)
                });
                if let Some(index) = index {
                    battle.skill_index = index;
                    if let Some(target) = battle
                        .cursor_world
                        .or_else(|| battle.world.commander().map(|u| u.position))
                    {
                        battle.cast_selected(target);
                    }
                }
            }
        }
        if !battle.paused && battle.world.tick.saturating_sub(*last_walk) >= 4 {
            let x = i32::from(keys.pressed(KeyCode::KeyD)) - i32::from(keys.pressed(KeyCode::KeyA));
            let z = i32::from(keys.pressed(KeyCode::KeyS)) - i32::from(keys.pressed(KeyCode::KeyW));
            if (x != 0 || z != 0)
                && let Some(hero) = battle.world.commander()
            {
                let id = hero.id;
                let right = Vec3::new(orbit.yaw.cos(), 0.0, -orbit.yaw.sin());
                let forward = Vec3::new(orbit.yaw.sin(), 0.0, orbit.yaw.cos());
                let position = point3(hero.position, 0.0)
                    + (right * x as f32 + forward * z as f32).normalize() * 2.0;
                if let Some(target) = battle.world.map.nearest_walkable(point2(position)) {
                    battle.command(SimCommand::Move {
                        units: vec![id],
                        target,
                        attack_move: true,
                    });
                }
                *last_walk = battle.world.tick;
            }
        }
    } else {
        for (key, action) in [
            (KeyCode::Tab, Action::Army),
            (KeyCode::Digit1, Action::Vanguard),
            (KeyCode::Digit2, Action::Ranger),
            (KeyCode::KeyH, Action::Hold),
        ] {
            if keys.just_pressed(key) {
                perform_action(action, &mut battle);
            }
        }
    }
    if keys.just_pressed(KeyCode::Space)
        && let Some(target) = battle
            .cursor_world
            .or_else(|| battle.world.commander().map(|u| u.position))
    {
        battle.cast_selected(target);
    }
    if keys.just_pressed(KeyCode::F5) {
        match battle.world.save_json() {
            Ok(data) => match std::fs::write(&cli.save_path, data) {
                Ok(()) => battle.notice = format!("Saved {}", cli.save_path),
                Err(error) => battle.notice = format!("Save failed: {error}"),
            },
            Err(error) => battle.notice = format!("Save failed: {error:?}"),
        }
        battle.notice_ticks = 180;
    }
    if keys.just_pressed(KeyCode::F9) {
        let result = std::fs::read_to_string(&cli.save_path)
            .map_err(|e| format!("{e}"))
            .and_then(|data| SimWorld::load_json(&data).map_err(|e| format!("{e:?}")));
        match result {
            Ok(world) => {
                battle.replace_world(world);
                battle.notice = "Save restored and validated.".into();
            }
            Err(error) => battle.notice = format!("Load rejected: {error}"),
        }
        battle.notice_ticks = 180;
    }
    if keys.just_pressed(KeyCode::F12) {
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk("dreadmarch-screenshot.png"));
    }
}

fn pointer_input(
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
    buttons: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut battle: ResMut<Battle>,
    mut rect: Single<(&mut Node, &mut Visibility), With<DragRect>>,
) {
    if battle.creator.is_some() {
        return;
    }
    let Some(cursor) = window.cursor_position() else {
        return;
    };
    let (camera, transform) = *camera;
    let over_ui = cursor.y < 90.0
        || cursor.y
            > window.height()
                - if battle.world.campaign.is_some() {
                    220.0
                } else {
                    145.0
                }
        || ((battle.help || battle.panel != CampaignPanel::Closed)
            && cursor.x < 600.0
            && cursor.y < 690.0);
    let ground = camera
        .viewport_to_world(transform, cursor)
        .ok()
        .and_then(|r| r.plane_intersection_point(Vec3::ZERO, InfinitePlane3d::new(Vec3::Y)));
    battle.cursor_world = ground.map(point2);
    if battle.world.campaign.is_some() {
        if !over_ui && let Some(target) = battle.cursor_world {
            if buttons.just_pressed(MouseButton::Left) && battle.cast_mode {
                battle.cast_selected(target);
                battle.cast_mode = false;
            } else if (buttons.just_pressed(MouseButton::Right)
                || buttons.just_pressed(MouseButton::Left))
                && let Some(hero) = battle.world.commander()
            {
                let id = hero.id;
                battle.command(SimCommand::Move {
                    units: vec![id],
                    target,
                    attack_move: true,
                });
                battle.order_marker = Some((point3(target, 0.16), 1.3));
                battle.cast_mode = false;
            }
        }
        return;
    }
    if buttons.just_pressed(MouseButton::Left) && !over_ui {
        if battle.cast_mode {
            if let Some(target) = battle.cursor_world {
                battle.command(SimCommand::Cast { target });
            }
            battle.cast_mode = false;
        } else {
            battle.drag_start = Some(cursor);
        }
    }
    if buttons.just_pressed(MouseButton::Right) && !over_ui {
        battle.cast_mode = false;
        if let Some(target) = battle.cursor_world {
            let ids = battle.selected_ids();
            let previous_failures = battle.command_failures;
            battle.command(SimCommand::Move {
                units: ids,
                target,
                attack_move: keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight),
            });
            if battle.command_failures == previous_failures {
                battle.order_marker = Some((point3(target, 0.16), 1.3));
            }
        }
    }
    let (node, visibility) = &mut *rect;
    **visibility = Visibility::Hidden;
    if let Some(start) = battle.drag_start {
        let min = start.min(cursor);
        let max = start.max(cursor);
        node.left = px(min.x);
        node.top = px(min.y);
        node.width = px(max.x - min.x);
        node.height = px(max.y - min.y);
        if start.distance(cursor) > 5.0 {
            **visibility = Visibility::Visible;
        }
        if buttons.just_released(MouseButton::Left) {
            let additive = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
            let mut chosen = Vec::new();
            if start.distance(cursor) > 7.0 {
                for unit in &battle.world.units {
                    if unit.team == Team::Player
                        && unit.hp > 0
                        && let Ok(p) =
                            camera.world_to_viewport(transform, point3(unit.position, 0.5))
                        && p.x >= min.x
                        && p.x <= max.x
                        && p.y >= min.y
                        && p.y <= max.y
                    {
                        chosen.push(unit.id);
                    }
                }
            } else {
                let mut nearest = (22.0_f32, None);
                for unit in &battle.world.units {
                    if unit.team == Team::Player
                        && unit.hp > 0
                        && let Ok(p) =
                            camera.world_to_viewport(transform, point3(unit.position, 0.5))
                    {
                        let d = p.distance(cursor);
                        if d < nearest.0 {
                            nearest = (d, Some(unit.id));
                        }
                    }
                }
                if let Some(id) = nearest.1 {
                    chosen.push(id);
                }
            }
            if !additive {
                battle.selected.clear();
            }
            battle.selected.extend(chosen);
            battle.drag_start = None;
            **visibility = Visibility::Hidden;
        }
    }
}

fn button_input(
    mut buttons: Query<(&Hovered, Has<Pressed>, &Action, &mut BackgroundColor), With<UiButton>>,
    battle: Res<Battle>,
) {
    for (hovered, pressed, action, mut bg) in &mut buttons {
        let selected = if let Action::ChooseClass(index) = action {
            battle
                .creator
                .as_ref()
                .is_some_and(|draft| rpg::catalogue().classes[*index].id == draft.class_id)
        } else {
            false
        };
        bg.0 = if pressed {
            Color::srgb(0.23, 0.30, 0.28)
        } else if selected {
            Color::srgb(0.18, 0.30, 0.27)
        } else if hovered.get() {
            Color::srgb(0.14, 0.22, 0.24)
        } else {
            Color::srgb(0.075, 0.105, 0.12)
        };
    }
}
fn activate_button(
    event: On<Activate>,
    actions: Query<&Action>,
    mut battle: ResMut<Battle>,
    mut focus: Option<ResMut<bevy::input_focus::InputFocus>>,
    keys: Option<Res<ButtonInput<KeyCode>>>,
) {
    if let Ok(action) = actions.get(event.entity) {
        perform_action(*action, &mut battle);
        // UI activation owns its keypress. Release button focus so a typed space
        // in the name editor cannot activate its previous button again.
        battle.ui_keyboard_consumed = keys.is_some_and(|keys| {
            keys.just_pressed(KeyCode::Space) || keys.just_pressed(KeyCode::Enter)
        });
        if let Some(focus) = &mut focus {
            focus.clear();
        }
    }
}

fn cycle_color(color: &mut String, choices: &[&str]) {
    let index = choices
        .iter()
        .position(|c| *c == color.as_str())
        .unwrap_or(0);
    *color = choices[(index + 1) % choices.len()].into();
}

fn perform_action(action: Action, battle: &mut Battle) {
    match action {
        Action::Vanguard => battle.command(SimCommand::Recruit {
            kind: UnitKind::Vanguard,
        }),
        Action::Ranger => battle.command(SimCommand::Recruit {
            kind: UnitKind::Ranger,
        }),
        Action::Spell => {
            battle.cast_mode = true;
            battle.notice = if let Some(skill) = battle.selected_skill() {
                format!("Choose a ground target for {}.", skill.name)
            } else {
                "Choose a ground target for Soulfire.".into()
            };
            battle.notice_ticks = 150;
        }
        Action::Army => battle.all_army(),
        Action::Hold => {
            let units = battle.selected_ids();
            battle.command(SimCommand::Hold { units });
        }
        Action::Pause => battle.paused = !battle.paused,
        Action::Restart => battle.restart(),
        Action::Help => {
            battle.help = !battle.help;
            battle.panel = CampaignPanel::Closed;
        }
        Action::Skills => {
            battle.panel = if battle.panel == CampaignPanel::Skills {
                CampaignPanel::Closed
            } else {
                CampaignPanel::Skills
            };
            battle.help = false;
        }
        Action::Inventory => {
            battle.panel = if battle.panel == CampaignPanel::Inventory {
                CampaignPanel::Closed
            } else {
                CampaignPanel::Inventory
            };
            battle.help = false;
        }
        Action::Previous => battle.cycle_selection(false),
        Action::Next => battle.cycle_selection(true),
        Action::Learn => battle.learn_selected(),
        Action::Equip => battle.equip_selected(),
        Action::Interact => battle.interact_nearest(),
        Action::Respec => battle.command(SimCommand::Respec),
        Action::Creator => {
            if let Some(c) = &battle.world.campaign {
                battle.creator = Some(c.character.clone());
                battle.editing_name = false;
                battle.paused = true;
                battle.help = false;
                battle.panel = CampaignPanel::Closed;
            }
        }
        Action::ChooseClass(index) => {
            if let Some(draft) = &mut battle.creator
                && let Some(profile) = rpg::catalogue().classes.get(index)
            {
                draft.class_id = profile.id.clone();
                draft.accent = profile.color.clone();
            }
        }
        Action::EditName => battle.editing_name = !battle.editing_name,
        Action::Body => {
            if let Some(draft) = &mut battle.creator {
                draft.body = match draft.body.as_str() {
                    "athletic" => "slender",
                    "slender" => "stocky",
                    _ => "athletic",
                }
                .into();
            }
        }
        Action::Skin => {
            if let Some(draft) = &mut battle.creator {
                cycle_color(
                    &mut draft.skin,
                    &["#f2d2b4", "#c69b7b", "#aa7655", "#704a37", "#422d25"],
                );
            }
        }
        Action::Hair => {
            if let Some(draft) = &mut battle.creator {
                cycle_color(
                    &mut draft.hair,
                    &[
                        "#2d2521", "#aa7447", "#e1c998", "#a44e36", "#bcbac2", "#151820",
                    ],
                );
            }
        }
        Action::HairStyle => {
            if let Some(draft) = &mut battle.creator {
                draft.hair_style = (draft.hair_style + 1) % 6;
            }
        }
        Action::Eyes => {
            if let Some(draft) = &mut battle.creator {
                cycle_color(
                    &mut draft.eyes,
                    &["#81b7b0", "#638eb9", "#704b2d", "#b89b44", "#b2a1ce"],
                );
            }
        }
        Action::Accent => {
            if let Some(draft) = &mut battle.creator {
                cycle_color(
                    &mut draft.accent,
                    &[
                        "#d9b66c", "#4cd5be", "#b45649", "#a18dc7", "#6e9fd1", "#c1c5c8",
                    ],
                );
            }
        }
        Action::Stature => {
            if let Some(draft) = &mut battle.creator {
                draft.stature = if draft.stature >= 115 {
                    85
                } else {
                    draft.stature + 5
                };
            }
        }
        Action::Begin => battle.begin_journey(),
        Action::CloseCreator => {
            battle.creator = None;
            battle.editing_name = false;
            battle.paused = false;
        }
    }
}

fn step_simulation(mut battle: ResMut<Battle>, cli: Res<Cli>) {
    if !battle.paused && battle.creator.is_none() && !cli.smoke {
        battle.advance();
    }
    if battle.notice_ticks > 0 {
        battle.notice_ticks -= 1;
    }
}

fn character_color(hex: &str) -> Color {
    let component =
        |start| u8::from_str_radix(&hex[start..start + 2], 16).unwrap_or(128) as f32 / 255.0;
    Color::srgb(component(1), component(3), component(5))
}

fn spawn_unit(
    commands: &mut Commands,
    assets: &VisualAssets,
    unit: &dread_sim::Unit,
    appearance: Option<&CharacterCreation>,
    materials: &mut Assets<StandardMaterial>,
) {
    let ally = unit.team == Team::Player;
    let commander = unit.kind == UnitKind::Commander;
    let brute = unit.kind == UnitKind::Brute;
    let profile = appearance.and_then(|c| rpg::class(&c.class_id));
    let ranger =
        unit.kind == UnitKind::Ranger || profile.is_some_and(|c| c.combat_style == "ranged");
    let caster = profile.is_some_and(|c| c.combat_style == "caster");
    let scale = if commander {
        1.25
    } else if brute {
        1.55
    } else {
        0.95
    };
    let body = if ally {
        assets.steel.clone()
    } else {
        assets.dark.clone()
    };
    let accent = if let Some(c) = appearance {
        material(materials, character_color(&c.accent), 0.45)
    } else if ally {
        if commander {
            assets.gold.clone()
        } else {
            assets.teal.clone()
        }
    } else {
        assets.red.clone()
    };
    let glow = if ally {
        assets.teal.clone()
    } else {
        assets.ember.clone()
    };
    let skin = appearance
        .map(|c| material(materials, character_color(&c.skin), 0.0))
        .unwrap_or_else(|| assets.bone.clone());
    let hair = appearance
        .map(|c| material(materials, character_color(&c.hair), 0.0))
        .unwrap_or_else(|| accent.clone());
    let eyes = appearance
        .map(|c| material(materials, character_color(&c.eyes), 0.0))
        .unwrap_or_else(|| glow.clone());
    let stature = appearance.map_or(1.0, |c| c.stature as f32 / 100.0);
    let width = appearance.map_or(1.0, |c| match c.body.as_str() {
        "slender" => 0.82,
        "stocky" => 1.22,
        _ => 1.0,
    });
    let hair_style = appearance.map_or(3, |c| c.hair_style);
    commands
        .spawn((
            UnitVisual {
                id: unit.id,
                kind: unit.kind,
                team: unit.team,
                previous: point3(unit.position, 0.08),
                current: point3(unit.position, 0.08),
                tick: 0,
                appearance: appearance.cloned(),
            },
            Transform::from_translation(point3(unit.position, 0.08)).with_scale(Vec3::new(
                scale * stature * width,
                scale * stature,
                scale * stature,
            )),
            Visibility::default(),
        ))
        .with_children(|parent| {
            for (mesh, mat, pos, size) in [
                (
                    assets.cone.clone(),
                    if appearance.is_some() {
                        accent.clone()
                    } else if ally {
                        assets.cloth.clone()
                    } else {
                        assets.red.clone()
                    },
                    Vec3::new(0.0, 0.45, 0.12),
                    Vec3::new(0.75, 0.85, 0.65),
                ),
                (
                    assets.cube.clone(),
                    body.clone(),
                    Vec3::new(0.0, 0.75, 0.0),
                    Vec3::new(0.55, 0.62, 0.38),
                ),
                (
                    assets.sphere.clone(),
                    skin.clone(),
                    Vec3::new(0.0, 1.19, 0.0),
                    Vec3::splat(0.40),
                ),
                (
                    assets.cube.clone(),
                    hair.clone(),
                    Vec3::new(0.0, 1.35, 0.0),
                    if hair_style == 0 {
                        Vec3::ZERO
                    } else {
                        Vec3::new(0.42, 0.10 + hair_style as f32 * 0.025, 0.42)
                    },
                ),
                (
                    assets.cube.clone(),
                    accent.clone(),
                    Vec3::new(-0.36, 0.95, 0.0),
                    Vec3::new(0.3, 0.3, 0.48),
                ),
                (
                    assets.cube.clone(),
                    accent.clone(),
                    Vec3::new(0.36, 0.95, 0.0),
                    Vec3::new(0.3, 0.3, 0.48),
                ),
                (
                    assets.cube.clone(),
                    assets.dark.clone(),
                    Vec3::new(-0.15, 0.22, 0.0),
                    Vec3::new(0.2, 0.4, 0.24),
                ),
                (
                    assets.cube.clone(),
                    assets.dark.clone(),
                    Vec3::new(0.15, 0.22, 0.0),
                    Vec3::new(0.2, 0.4, 0.24),
                ),
                (
                    assets.cube.clone(),
                    eyes.clone(),
                    Vec3::new(0.0, 1.21, -0.20),
                    Vec3::new(0.27, 0.055, 0.05),
                ),
            ] {
                parent.spawn((
                    Mesh3d(mesh),
                    MeshMaterial3d(mat),
                    Transform::from_translation(pos).with_scale(size),
                ));
            }
            if ranger {
                parent.spawn((
                    Mesh3d(assets.ring.clone()),
                    MeshMaterial3d(assets.gold.clone()),
                    Transform::from_xyz(0.48, 0.8, -0.1)
                        .with_rotation(Quat::from_rotation_x(FRAC_PI_2))
                        .with_scale(Vec3::new(0.8, 0.4, 1.5)),
                ));
            } else if caster {
                parent.spawn((
                    Mesh3d(assets.cube.clone()),
                    MeshMaterial3d(assets.dark.clone()),
                    Transform::from_xyz(0.55, 0.85, 0.0).with_scale(Vec3::new(0.09, 1.8, 0.09)),
                ));
                parent.spawn((
                    Mesh3d(assets.sphere.clone()),
                    MeshMaterial3d(accent.clone()),
                    Transform::from_xyz(0.55, 1.8, 0.0).with_scale(Vec3::splat(0.35)),
                ));
            } else {
                parent.spawn((
                    Mesh3d(assets.cube.clone()),
                    MeshMaterial3d(if commander {
                        assets.teal.clone()
                    } else {
                        assets.steel.clone()
                    }),
                    Transform::from_xyz(0.56, 0.94, -0.15)
                        .with_rotation(Quat::from_rotation_z(-0.22))
                        .with_scale(Vec3::new(0.12, 1.1, 0.05)),
                ));
                parent.spawn((
                    Mesh3d(assets.cube.clone()),
                    MeshMaterial3d(assets.gold.clone()),
                    Transform::from_xyz(0.45, 0.48, -0.15).with_scale(Vec3::new(0.35, 0.08, 0.1)),
                ));
            }
            if commander && appearance.is_none() {
                parent.spawn((
                    Mesh3d(assets.cone.clone()),
                    MeshMaterial3d(assets.gold.clone()),
                    Transform::from_xyz(0.0, 1.6, 0.0).with_scale(Vec3::new(0.28, 0.48, 0.28)),
                ));
            }
            if appearance.is_some() && hair_style >= 2 {
                for side in if hair_style == 4 {
                    vec![-0.23, 0.23]
                } else {
                    vec![0.0]
                } {
                    parent.spawn((
                        Mesh3d(assets.cone.clone()),
                        MeshMaterial3d(hair.clone()),
                        Transform::from_xyz(side, if hair_style == 3 { 1.57 } else { 1.1 }, 0.15)
                            .with_scale(Vec3::new(
                                if hair_style == 5 { 0.65 } else { 0.22 },
                                0.6,
                                0.30,
                            )),
                    ));
                }
            }
            if brute {
                for x in [-0.32, 0.32] {
                    parent.spawn((
                        Mesh3d(assets.cone.clone()),
                        MeshMaterial3d(assets.bone.clone()),
                        Transform::from_xyz(x, 1.45, 0.0)
                            .with_rotation(Quat::from_rotation_z(-x))
                            .with_scale(Vec3::new(0.18, 0.65, 0.18)),
                    ));
                }
            }
        });
}

fn update_units(
    mut commands: Commands,
    (assets, mut materials): (Res<VisualAssets>, ResMut<Assets<StandardMaterial>>),
    battle: Res<Battle>,
    (time, fixed, cli): (Res<Time>, Res<Time<Fixed>>, Res<Cli>),
    mut scratch: Local<RenderScratch>,
    mut visuals: Query<(Entity, &mut UnitVisual, &mut Transform)>,
    mut relics: Query<
        (
            &RelicVisual,
            &mut Transform,
            &mut MeshMaterial3d<StandardMaterial>,
        ),
        Without<UnitVisual>,
    >,
) {
    // Reuse lookup capacity across frames. No unit cap or state reduction is used.
    scratch.indices.clear();
    scratch.existing.clear();
    scratch.indices.extend(
        battle
            .world
            .units
            .iter()
            .enumerate()
            .filter(|(_, u)| u.hp > 0)
            .map(|(index, unit)| (unit.id, index)),
    );
    let alpha = if battle.paused || cli.smoke || battle.world.status != MatchStatus::Playing {
        1.0
    } else {
        fixed.overstep_fraction()
    };
    let appearance = battle
        .creator
        .as_ref()
        .or_else(|| battle.world.campaign.as_ref().map(|c| &c.character));
    for (entity, mut visual, mut transform) in &mut visuals {
        if let Some(&index) = scratch.indices.get(&visual.id) {
            let unit = &battle.world.units[index];
            // A restored save may reuse an ID for a different recruited class.
            // Recreate its presentation rather than retaining an obsolete mesh.
            let desired_appearance =
                if unit.kind == UnitKind::Commander && unit.team == Team::Player {
                    appearance
                } else {
                    None
                };
            if unit.kind != visual.kind
                || unit.team != visual.team
                || visual.appearance.as_ref() != desired_appearance
            {
                commands.entity(entity).despawn();
                continue;
            }
            scratch.existing.insert(unit.id);
            let desired = point3(unit.position, 0.08);
            let delta = desired - visual.current;
            if visual.tick != battle.world.tick || desired != visual.current {
                visual.previous = if battle.world.tick <= visual.tick {
                    desired
                } else {
                    visual.current
                };
                visual.current = desired;
                visual.tick = battle.world.tick;
            }
            transform.translation = visual.previous.lerp(visual.current, alpha);
            if delta.length_squared() > 0.00001 {
                transform.rotation = Quat::from_rotation_y(delta.x.atan2(delta.z) + PI);
            }
        } else {
            commands.entity(entity).despawn();
        }
    }
    for unit in battle.world.units.iter().filter(|u| u.hp > 0) {
        if !scratch.existing.contains(&unit.id) {
            spawn_unit(
                &mut commands,
                &assets,
                unit,
                if unit.kind == UnitKind::Commander && unit.team == Team::Player {
                    appearance
                } else {
                    None
                },
                &mut materials,
            );
        }
    }
    for (relic, mut transform, mut material) in &mut relics {
        transform.rotation = Quat::from_rotation_y(time.elapsed_secs() * 0.4);
        if let Some(objective) = battle.world.objectives.iter().find(|o| o.id == relic.0) {
            material.0 = if objective.captured {
                assets.teal.clone()
            } else {
                assets.gold.clone()
            };
        }
    }
}

fn update_hud(
    battle: Res<Battle>,
    mut hud: Single<&mut Text, With<Hud>>,
    mut status: Single<&mut Text, (With<StatusLine>, Without<Hud>)>,
    mut help: Single<(&mut Text, &mut Visibility), HelpFilter>,
) {
    if let Some(draft) = &battle.creator {
        let profile = rpg::class(&draft.class_id).expect("creator class");
        hud.0 = format!(
            "{}  |  {}\nCHARACTER CREATOR  |  {} CLASS IDENTITIES",
            draft.name,
            profile.name,
            rpg::catalogue().classes.len()
        );
        status.0 = "Choose a class, edit your appearance, then begin your journey.\nEDIT NAME: type your name; Backspace removes a character, Ctrl+A clears, Enter finishes.".into();
        *help.1 = Visibility::Hidden;
        return;
    }
    if let Some(c) = &battle.world.campaign {
        let hero = battle.world.commander();
        let health = hero.map_or("FALLEN".into(), |u| format!("{}/{}", u.hp, u.max_hp));
        hud.0 = format!(
            "{}  |  {}  |  LEVEL {}\n{}   HP {}   FOCUS {}   XP {}/{}   POINTS {}",
            c.character.name,
            c.class().name,
            c.level,
            rpg::region_name(&c.region),
            health,
            c.focus,
            c.experience,
            c.next_level_experience(),
            c.skill_points
        );
        let skill = battle
            .selected_skill()
            .map_or("No selected skill".into(), |s| {
                let recovery = c
                    .cooldowns
                    .get(&s.id)
                    .copied()
                    .unwrap_or(0)
                    .div_ceil(TICK_HZ);
                format!(
                    "{} | {} | {} focus{}",
                    s.name,
                    if c.learned.contains(&s.id) {
                        "LEARNED"
                    } else {
                        "UNLEARNED"
                    },
                    s.focus_cost,
                    if recovery > 0 {
                        format!(" | {recovery}s")
                    } else {
                        String::new()
                    }
                )
            });
        let nearby = battle
            .nearest_interaction()
            .map_or(String::new(), |(_, name, distance)| {
                if distance <= 3_500u64.pow(2) {
                    format!("J: {name}")
                } else {
                    format!("Nearby: {name}")
                }
            });
        status.0 = format!(
            "{}  |  {}  |  {}\nQUEST: {}\n{}{}",
            if battle.world.status == MatchStatus::Defeat {
                "FALLEN | F9 LOAD OR R RESTART"
            } else if battle.paused {
                "PAUSED"
            } else {
                "THE ASHEN CONCORD"
            },
            skill,
            nearby,
            c.quest_text(),
            c.dialogue,
            if battle.notice_ticks > 0 {
                format!("\n{}", battle.notice)
            } else {
                String::new()
            }
        );
        let (text, visibility) = &mut *help;
        **visibility =
            if battle.creator.is_none() && (battle.help || battle.panel != CampaignPanel::Closed) {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        text.0 = match battle.panel {
            CampaignPanel::Skills => {
                let rows = c.class().skills.iter().enumerate().map(|(i, s)| format!("{} {} | {} | level {}{}",
                    if i == battle.skill_index { ">" } else { " " }, s.name, s.branch, s.minimum_level,
                    if c.learned.contains(&s.id) { "  [learned]" } else { "" })).collect::<Vec<_>>().join("\n");
                let selected = battle.selected_skill().map_or(String::new(), |s| format!("\n\n{}\n{}\n{} | {} focus | {}s recovery\nPrerequisite: {}",
                    s.name, s.description, s.kind, s.focus_cost, s.cooldown_ticks.div_ceil(TICK_HZ),
                    s.requires.as_ref().and_then(|id| c.class().skills.iter().find(|p| &p.id == id)).map_or("None", |p| p.name.as_str())));
                format!("{} DISCIPLINES  |  {} POINTS\n[ / ] select | L learn | Space cast\n\n{}{}\n\nT: retrain freely in Hearthmere. K: close.", c.class().name.to_uppercase(), c.skill_points, rows, selected)
            }
            CampaignPanel::Inventory => {
                let start = battle.item_index / 12 * 12;
                let rows = c.inventory.iter().enumerate().skip(start).take(12).map(|(i, item)| format!("{} {}\n    {} | {} | power {}{}",
                    if i == battle.item_index { ">" } else { " " }, item.name, item.rarity, item.slot, item.power,
                    if c.equipped.get(&item.slot) == Some(&item.id) { " | EQUIPPED" } else { "" })).collect::<Vec<_>>().join("\n");
                format!("INVENTORY  |  {} ITEMS\n[ / ] select | U equip | I close\n\n{}\n\nApproach a dropped item and press J to collect it.",
                    c.inventory.len(), if rows.is_empty() { "Your pack is empty." } else { &rows })
            }
            CampaignPanel::Closed => "THE ASHEN CONCORD\n\nCreate a wanderer and follow Maelin's quest.\nAll encounters and progression work solo.\n\nRight-click or WASD moves your hero.\nNearby enemies are attacked automatically.\nSpace casts the selected class skill at the cursor.\n1-6 cast your learned active skills in tree order.\nJ talks, opens caches, collects loot or uses a gate.\n\nK opens disciplines. [ / ] chooses a skill.\nL learns it with an available point.\nI opens inventory; [ / ] selects and U equips.\nT retrains freely in Hearthmere.\nF2 opens the creator; Begin starts a new journey.\n\nQ/E orbit | Wheel zoom | C center\nP pause | R restart journey | F5 save | F9 load\nF12 screenshot | Esc close/exit | F1 close help\n\nPlayable foundation with procedural character art.\nThe full campaign and production class kits\nremain development goals.".into(),
        };
        return;
    }
    let army = battle
        .world
        .units
        .iter()
        .filter(|u| u.team == Team::Player && u.hp > 0)
        .count();
    let enemy = battle
        .world
        .units
        .iter()
        .filter(|u| u.team == Team::Enemy && u.hp > 0)
        .count();
    let captured = battle
        .world
        .objectives
        .iter()
        .filter(|o| o.captured)
        .count();
    let cooldown = battle.world.commander_cooldown.div_ceil(TICK_HZ);
    let next_wave = if battle.world.wave < TOTAL_WAVES {
        format!(
            "NEXT {}s",
            battle.world.wave_ticks_remaining.div_ceil(TICK_HZ)
        )
    } else {
        "FINAL WAVE".into()
    };
    hud.0 = format!(
        "ESSENCE {}     ARMY {}     HOSTILES {}\nRELICS {}/{}     WAVE {}/{}     {}     KILLS {}",
        battle.world.economy.essence,
        army,
        enemy,
        captured,
        battle.world.objectives.len(),
        battle.world.wave,
        TOTAL_WAVES,
        next_wave,
        battle.world.economy.kills
    );
    let state = match battle.world.status {
        MatchStatus::Playing => {
            if battle.paused {
                "PAUSED"
            } else {
                "MARCH IN PROGRESS"
            }
        }
        MatchStatus::Victory => "VICTORY - THE RELICS ARE OURS",
        MatchStatus::Defeat => "DEFEAT - THE COMMANDER HAS FALLEN",
    };
    let notice = if battle.notice_ticks > 0 {
        battle.notice.as_str()
    } else {
        "Capture relics for essence. Recruit units and protect your commander."
    };
    status.0 = format!(
        "{}  |  {} selected  |  SOULFIRE {}  |  {}",
        state,
        battle.selected.len(),
        if cooldown == 0 {
            "READY".into()
        } else {
            format!("{cooldown}s")
        },
        notice
    );
    let (text, visibility) = &mut *help;
    **visibility = if battle.help {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };
    text.0="COMMAND THE HOLLOW MARCH\n\nSecure all three ancient relics and survive five waves.\nKeep the commander alive; losing them ends the match.\n\nLeft-click a warrior, or drag to select a squad.\nShift adds to your selection. Tab selects the army.\nRight-click moves in formation. Shift + right-click\norders an attack-move. H stops the selected squad.\nC selects and centers the commander.\n\nSpace casts Soulfire at the cursor. The button lets\nyou choose the target with your next click.\n1 recruits a Vanguard. 2 recruits a Ranger.\n\nWASD pan / Q E orbit / Wheel zoom\nP pause / R restart / F5 save / F9 load\nF12 screenshot / Esc exit / F1 close help\n\nFoundation playtest: original procedural geometry.\nProduction art, audio, campaign and netcode remain\ntracked in the repository roadmap.".into();
}

fn draw_tactical(mut gizmos: Gizmos, mut battle: ResMut<Battle>, time: Res<Time>) {
    if let Some(c) = &battle.world.campaign {
        for poi in rpg::points_of_interest(&c.region) {
            gizmos.circle(
                Isometry3d::new(point3(poi.position, 0.13), Quat::from_rotation_x(FRAC_PI_2)),
                if poi.kind == "gate" { 2.0 } else { 0.9 },
                GOLD,
            );
        }
        for loot in &c.ground_loot {
            gizmos.circle(
                Isometry3d::new(
                    point3(loot.position, 0.18),
                    Quat::from_rotation_x(FRAC_PI_2),
                ),
                0.5,
                TEAL,
            );
            gizmos.line(point3(loot.position, 0.2), point3(loot.position, 1.4), GOLD);
        }
    }
    for unit in &battle.world.units {
        if unit.hp <= 0 {
            continue;
        }
        let pos = point3(unit.position, 0.14);
        if battle.selected.contains(&unit.id) {
            gizmos.circle(
                Isometry3d::new(pos, Quat::from_rotation_x(FRAC_PI_2)),
                if unit.kind == UnitKind::Commander {
                    0.88
                } else {
                    0.68
                },
                TEAL,
            );
            if let Some(target) = unit.target {
                gizmos.line(
                    pos,
                    point3(target, 0.14),
                    Color::srgba(0.3, 0.75, 0.67, 0.4),
                );
            }
        }
        let h = if unit.kind == UnitKind::Brute {
            2.8
        } else if unit.kind == UnitKind::Commander {
            2.4
        } else {
            1.8
        };
        let left = point3(unit.position, h) + Vec3::new(-0.48, 0.0, 0.0);
        gizmos.line(left, left + Vec3::X * 0.96, Color::srgb(0.18, 0.13, 0.12));
        gizmos.line(
            left,
            left + Vec3::X * (0.96 * unit.hp.max(0) as f32 / unit.max_hp as f32),
            if unit.team == Team::Player {
                TEAL
            } else {
                Color::srgb(1.0, 0.22, 0.08)
            },
        );
    }
    for objective in &battle.world.objectives {
        gizmos.circle(
            Isometry3d::new(
                point3(objective.position, 0.15),
                Quat::from_rotation_x(FRAC_PI_2),
            ),
            CAPTURE_RADIUS as f32 / 1000.0,
            if objective.captured { TEAL } else { GOLD },
        );
        if !objective.captured && objective.progress > 0 {
            let center = point3(objective.position, 0.25);
            let end = 48 * objective.progress / CAPTURE_TICKS;
            for segment in 0..end {
                let a = segment as f32 * PI / 24.0;
                let b = (segment + 1) as f32 * PI / 24.0;
                gizmos.line(
                    center + Vec3::new(a.cos(), 0.0, a.sin()) * 2.3,
                    center + Vec3::new(b.cos(), 0.0, b.sin()) * 2.3,
                    TEAL,
                );
            }
        }
    }
    if battle.cast_mode
        && let Some(target) = battle.cursor_world
    {
        gizmos.circle(
            Isometry3d::new(point3(target, 0.17), Quat::from_rotation_x(FRAC_PI_2)),
            battle.selected_skill().map_or(ABILITY_RADIUS, |s| s.radius) as f32 / 1000.0,
            GOLD,
        );
    }
    if let Some((pos, remaining)) = &mut battle.order_marker {
        *remaining -= time.delta_secs();
        gizmos.circle(
            Isometry3d::new(*pos, Quat::from_rotation_x(FRAC_PI_2)),
            0.45 + (1.3 - *remaining) * 0.8,
            TEAL,
        );
        if *remaining <= 0.0 {
            battle.order_marker = None;
        }
    }
    if let Some((pos, remaining)) = &mut battle.spell_effect {
        *remaining -= time.delta_secs();
        let radius = (0.85 - *remaining) * 5.2;
        for i in 0..3 {
            gizmos.circle(
                Isometry3d::new(
                    *pos + Vec3::Y * (i as f32 * 0.22),
                    Quat::from_rotation_x(FRAC_PI_2),
                ),
                radius + i as f32 * 0.2,
                if i == 1 { GOLD } else { TEAL },
            );
        }
        for i in 0..12 {
            let a = i as f32 * PI / 6.0;
            let edge = *pos + Vec3::new(a.cos(), 0.0, a.sin()) * radius;
            gizmos.line(edge, edge + Vec3::Y * (*remaining * 3.0), TEAL);
        }
        if *remaining <= 0.0 {
            battle.spell_effect = None;
        }
    }
    for impact in &mut battle.impacts {
        impact.lifetime -= time.delta_secs();
        let scale = if impact.death { 0.8 } else { 0.28 };
        for i in 0..6 {
            let angle = i as f32 * PI / 3.0;
            let offset = Vec3::new(angle.cos(), 0.5, angle.sin()) * scale;
            gizmos.line(
                impact.position + offset * 0.3,
                impact.position + offset,
                if impact.death {
                    Color::srgb(0.95, 0.3, 0.12)
                } else {
                    GOLD
                },
            );
        }
    }
    battle.impacts.retain(|impact| impact.lifetime > 0.0);
}

fn smoke_driver(
    (mut cli, mut battle): (ResMut<Cli>, ResMut<Battle>),
    mut commands: Commands,
    mut exit: MessageWriter<AppExit>,
    offscreen: Option<Res<OffscreenTarget>>,
    hud: Query<(&ComputedNode, &ComputedUiTargetCamera), With<Hud>>,
    buttons: Query<(Entity, &Action)>,
    creator: Query<&ComputedNode, With<CreatorPanel>>,
) {
    cli.frames += 1;
    if cli.campaign_smoke {
        if battle.creator.is_none() {
            for _ in 0..if cli.campaign_stage < 4 { 4 } else { 1 } {
                battle.advance();
            }
        }
        let action = match cli.frames {
            2 => battle
                .creator
                .as_ref()
                .and_then(|c| {
                    rpg::catalogue()
                        .classes
                        .iter()
                        .position(|p| p.id == c.class_id)
                })
                .map(Action::ChooseClass),
            4 => Some(Action::Body),
            6 => Some(Action::HairStyle),
            92 => Some(Action::Begin),
            98 => {
                battle.skill_index = battle
                    .world
                    .campaign
                    .as_ref()
                    .expect("campaign")
                    .class()
                    .skills
                    .iter()
                    .position(|s| {
                        s.kind == "active"
                            && s.minimum_level == 1
                            && s.requires.is_none()
                            && !battle
                                .world
                                .campaign
                                .as_ref()
                                .expect("campaign")
                                .learned
                                .contains(&s.id)
                    })
                    .expect("each class needs an alternative starting discipline");
                Some(Action::Learn)
            }
            _ => None,
        };
        if let Some(action) = action {
            let entity = buttons
                .iter()
                .find(|(_, a)| **a == action)
                .expect("real UI action must exist")
                .0;
            commands.trigger(Activate { entity });
        }
        if cli.frames == 8 {
            assert!(
                !creator.single().expect("creator UI").is_empty(),
                "creator must have a visible computed layout"
            );
        }
        // Warm the real 3D/text pipelines before capturing the creator.
        if cli.frames == 90
            && let Some(path) = &cli.screenshot
        {
            let path = std::path::Path::new(path);
            let stem = path.file_stem().unwrap_or_default().to_string_lossy();
            let creator_path = path.with_file_name(format!("{stem}-creator.png"));
            let capture = if let Some(target) = offscreen.as_ref() {
                Screenshot::image(target.0.clone())
            } else {
                Screenshot::primary_window()
            };
            commands
                .spawn(capture)
                .observe(save_to_disk(creator_path))
                .observe(|_: On<ScreenshotCaptured>, mut cli: ResMut<Cli>| {
                    cli.creator_captured = true;
                });
        }
        if cli.frames == 100 {
            battle.skill_index = 0;
            let target = battle.world.commander().expect("hero").position;
            battle.cast_selected(target);
        }
        if cli.frames == 102 {
            let id = battle.world.commander().expect("hero").id;
            battle.command(SimCommand::Move {
                units: vec![id],
                target: Point::new(11_500, 18_500),
                attack_move: false,
            });
            cli.campaign_stage = 1;
        }
        if cli.frames > 102 {
            let origin = battle
                .world
                .commander()
                .expect("living smoke hero")
                .position;
            match cli.campaign_stage {
                1 if origin.distance_squared(Point::new(11_500, 18_500)) <= 3_300u64.pow(2) => {
                    battle.command(SimCommand::Interact {
                        target: "elder".into(),
                    });
                    let id = battle.world.commander().expect("hero").id;
                    battle.command(SimCommand::Move {
                        units: vec![id],
                        target: Point::new(7_500, 25_500),
                        attack_move: false,
                    });
                    cli.campaign_stage = 2;
                }
                2 if origin.distance_squared(Point::new(7_500, 25_500)) <= 3_300u64.pow(2) => {
                    battle.command(SimCommand::Interact {
                        target: "chest".into(),
                    });
                    battle.equip_selected();
                    let id = battle.world.commander().expect("hero").id;
                    battle.command(SimCommand::Move {
                        units: vec![id],
                        target: Point::new(43_500, 18_500),
                        attack_move: false,
                    });
                    cli.campaign_stage = 3;
                }
                3 if origin.distance_squared(Point::new(43_500, 18_500)) <= 3_300u64.pow(2) => {
                    battle.command(SimCommand::Interact {
                        target: "moor_gate".into(),
                    });
                    cli.campaign_stage = 4;
                }
                4 => {
                    battle.skill_index = battle
                        .world
                        .campaign
                        .as_ref()
                        .expect("campaign")
                        .class()
                        .skills
                        .iter()
                        .position(|s| {
                            s.kind == "active"
                                && s.id
                                    != battle
                                        .world
                                        .campaign
                                        .as_ref()
                                        .expect("campaign")
                                        .class()
                                        .skills[0]
                                        .id
                                && battle
                                    .world
                                    .campaign
                                    .as_ref()
                                    .expect("campaign")
                                    .learned
                                    .contains(&s.id)
                        })
                        .expect("learned second skill");
                    battle.cast_selected(origin);
                    cli.campaign_stage = 5;
                }
                _ => {}
            }
        }
    } else if cli.smoke {
        battle.advance();
        if cli.frames == 12 {
            let units = battle.selected_ids();
            battle.command(SimCommand::Move {
                units,
                target: Point::new(18_000, 18_000),
                attack_move: true,
            });
            battle.command(SimCommand::Recruit {
                kind: UnitKind::Ranger,
            });
        }
        if cli.frames == 70 {
            battle.command(SimCommand::Cast {
                target: Point::new(18_000, 18_000),
            });
        }
    }
    let ready = !cli.campaign_smoke || cli.campaign_stage >= 5;
    if cli.frames >= 90 && ready && !cli.hud_validated {
        let (node, target) = hud.single().expect("smoke must include the game HUD");
        assert!(
            !node.is_empty() && target.get().is_some(),
            "HUD must have a real camera and nonzero layout"
        );
        cli.hud_validated = true;
        if let Some(path) = cli.screenshot.clone() {
            let capture = if let Some(target) = offscreen {
                Screenshot::image(target.0.clone())
            } else {
                Screenshot::primary_window()
            };
            commands.spawn(capture).observe(save_to_disk(path)).observe(
                |_: On<ScreenshotCaptured>, mut cli: ResMut<Cli>| {
                    cli.captured = true;
                },
            );
        }
    }
    if cli.smoke
        && cli.frames >= 120
        && ready
        && (cli.screenshot.is_none()
            || (cli.captured && (!cli.campaign_smoke || cli.creator_captured)))
    {
        let saved = battle.world.save_json().expect("smoke save serialization");
        let restored = SimWorld::load_json(&saved).expect("smoke save validation");
        assert_eq!(
            battle.world.checksum(),
            restored.checksum(),
            "save roundtrip must preserve exact state"
        );
        if cli.campaign_smoke {
            let c = restored.campaign.as_ref().expect("campaign saved");
            assert_eq!(
                c.main_quest, 1,
                "native interaction must accept the main quest"
            );
            assert_eq!(
                c.region, "hollow_moor",
                "native interaction must travel between regions"
            );
            assert!(
                c.learned.len() >= 2 && !c.equipped.is_empty(),
                "native learning and equipment must persist"
            );
            assert!(battle.creator.is_none(), "creator must commit to gameplay");
            assert!(
                battle.command_count >= 10,
                "campaign smoke must exercise progression and travel"
            );
        } else {
            assert_eq!(
                battle.command_count, 3,
                "legacy smoke must issue movement, recruitment and ability"
            );
        }
        assert_eq!(battle.command_failures, 0, "smoke commands must succeed");
        if let Some(path) = &cli.screenshot {
            assert!(
                std::fs::metadata(path).is_ok_and(|m| m.len() > 0),
                "captured PNG must exist"
            );
        }
        println!(
            "{} frames={} tick={} units={} checksum={:016x} screenshot_captured={} commands_ok={} save_roundtrip=true hud_validated={}",
            if cli.campaign_smoke {
                "DREADMARCH_CAMPAIGN_RENDER_SMOKE_OK"
            } else {
                "DREADMARCH_RENDER_SMOKE_OK"
            },
            cli.frames,
            battle.world.tick,
            battle.world.units.len(),
            battle.world.checksum(),
            cli.captured,
            battle.command_count,
            cli.hud_validated
        );
        exit.write(AppExit::Success);
    } else if cli.smoke && cli.frames >= if cli.campaign_smoke { 480 } else { 240 } {
        eprintln!(
            "DREADMARCH_RENDER_SMOKE_FAILED: command path or GPU capture exceeded the finite frame budget; campaign_stage={}",
            cli.campaign_stage
        );
        exit.write(AppExit::error());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_creator_buttons_commit_class_appearance_and_progression_to_portable_save() {
        let mut app = App::new();
        app.insert_resource(Battle::campaign(CharacterCreation::default(), true))
            .add_observer(activate_button);
        let warlock = rpg::catalogue()
            .classes
            .iter()
            .position(|c| c.id == "warlock")
            .unwrap();
        for action in [
            Action::ChooseClass(warlock),
            Action::Body,
            Action::HairStyle,
            Action::Begin,
        ] {
            let entity = app.world_mut().spawn(action).id();
            app.world_mut().trigger(Activate { entity });
        }
        {
            let mut battle = app.world_mut().resource_mut::<Battle>();
            assert!(battle.creator.is_none() && !battle.paused);
            let c = battle.world.campaign.as_ref().unwrap();
            assert_eq!(c.character.class_id, "warlock");
            assert_eq!(c.character.body, "slender");
            assert_eq!(c.character.hair_style, 1);
            battle.skill_index = 3;
        }
        let entity = app.world_mut().spawn(Action::Learn).id();
        app.world_mut().trigger(Activate { entity });
        let battle = app.world().resource::<Battle>();
        assert_eq!(battle.command_failures, 0);
        assert_eq!(battle.world.campaign.as_ref().unwrap().learned.len(), 2);
        let exported = battle.world.save_json().unwrap();
        let imported = SimWorld::load_json(&exported).unwrap();
        assert_eq!(battle.world.checksum(), imported.checksum());
    }

    #[test]
    fn real_button_activation_routes_recruitment_pause_and_restart() {
        let mut app = App::new();
        app.insert_resource(Battle::default())
            .add_observer(activate_button);
        let recruit = app.world_mut().spawn(Action::Ranger).id();
        let pause = app.world_mut().spawn(Action::Pause).id();
        let restart = app.world_mut().spawn(Action::Restart).id();
        let before = app.world().resource::<Battle>().world.economy.essence;
        app.world_mut().trigger(Activate { entity: recruit });
        assert_eq!(
            app.world().resource::<Battle>().world.economy.essence,
            before - UnitKind::Ranger.stats().recruitment_cost.unwrap()
        );
        app.world_mut().trigger(Activate { entity: pause });
        assert!(app.world().resource::<Battle>().paused);
        app.world_mut().trigger(Activate { entity: restart });
        assert!(!app.world().resource::<Battle>().paused);
        assert_eq!(
            app.world().resource::<Battle>().world.checksum(),
            SimWorld::new(SEED).checksum()
        );
    }

    #[test]
    fn surviving_squad_can_still_receive_orders_after_a_selected_unit_dies() {
        let mut battle = Battle::default();
        let victim = battle
            .world
            .units
            .iter()
            .find(|u| u.kind == UnitKind::Vanguard)
            .unwrap()
            .id;
        battle.world.units.retain(|u| u.id != victim);
        battle.advance();
        assert!(!battle.selected.contains(&victim));
        let survivors = battle.selected_ids();
        assert!(!survivors.is_empty());
        battle.command(SimCommand::Move {
            units: survivors,
            target: Point {
                x: 10_500,
                z: 18_500,
            },
            attack_move: true,
        });
        assert_eq!(battle.command_failures, 0);
    }

    #[test]
    fn rejected_spell_does_not_display_a_success_effect() {
        let mut battle = Battle::default();
        battle.command(SimCommand::Cast {
            target: Point {
                x: -1_000,
                z: 1_000,
            },
        });
        assert!(battle.spell_effect.is_none());
        assert_eq!(battle.command_failures, 1);
        assert_eq!(battle.world.commander_cooldown, 0);
    }
}
