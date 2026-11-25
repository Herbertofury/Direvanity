//! Dreadmarch's native presentation. The authoritative game state lives in dread-sim.
//! All geometry and visual effects below are original procedural game assets.
use bevy::{
    app::{AppExit, ScheduleRunnerPlugin},
    camera::{RenderTarget, ScalingMode},
    input::mouse::MouseWheel,
    picking::hover::Hovered,
    post_process::bloom::Bloom,
    prelude::*,
    render::{
        render_resource::{TextureFormat, TextureUsages},
        view::screenshot::{Screenshot, ScreenshotCaptured, save_to_disk},
    },
    ui::Pressed,
    ui_widgets::{Activate, Button as UiButton},
    window::{ExitCondition, PrimaryWindow},
    winit::WinitPlugin,
};
use dread_sim::{
    ABILITY_RADIUS, CAPTURE_RADIUS, CAPTURE_TICKS, Command as SimCommand, MatchStatus, Point,
    SimEvent, TICK_HZ, Team, UnitKind, World as SimWorld,
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
        }
    }
}
impl Battle {
    fn command(&mut self, command: SimCommand) {
        self.command_count += 1;
        let cast = match &command {
            SimCommand::Cast { target } => Some(*target),
            _ => None,
        };
        match self.world.apply(command) {
            Ok(()) => {
                if let Some(target) = cast {
                    self.spell_effect = Some((point3(target, 0.14), 0.85));
                    self.notice = "Soulfire unleashed.".into();
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
    screenshot: Option<String>,
    frames: u32,
    captured: bool,
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
}
#[derive(Component)]
struct MainCamera;
#[derive(Component)]
struct UnitVisual {
    id: u32,
    previous: Vec3,
    current: Vec3,
    tick: u64,
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
#[derive(Component)]
struct DragRect;
#[derive(Component, Clone, Copy)]
enum Action {
    Vanguard,
    Ranger,
    Spell,
    Army,
    Hold,
    Pause,
    Restart,
    Help,
}

fn main() {
    let mut cli = Cli::default();
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--smoke-test" => cli.smoke = true,
            "--screenshot" => {
                cli.screenshot = args.next();
                if cli.screenshot.is_none() {
                    eprintln!("--screenshot requires a path");
                    std::process::exit(2);
                }
            }
            "--help" | "-h" => {
                println!(
                    "Dreadmarch native playtest\n  --smoke-test          exercise real rendering + game commands, then exit\n  --screenshot PATH     capture a PNG after scene warmup\nControls: left select/drag, right move, Shift+right attack-move; WASD pan, Q/E orbit, wheel zoom; Space spell, Tab army, C commander; 1/2 recruit; P pause; R restart; F5/F9 save/load; F1 help; Esc exit."
                );
                return;
            }
            _ => {
                eprintln!("Unknown argument: {arg}; use --help");
                std::process::exit(2);
            }
        }
    }
    let smoke = cli.smoke;
    let mut app = App::new();
    app.insert_resource(cli)
        .insert_resource(Battle::default())
        .insert_resource(Orbit::default())
        .insert_resource(Time::<Fixed>::from_hz(f64::from(TICK_HZ)))
        .insert_resource(ClearColor(Color::srgb(0.025, 0.041, 0.05)))
        .insert_resource(GlobalAmbientLight {
            color: Color::srgb(0.45, 0.65, 0.75),
            brightness: 300.0,
            ..default()
        });
    let plugins = DefaultPlugins.set(WindowPlugin {
        primary_window: (!smoke).then(|| Window {
            title: "DREADMARCH | The Hollow March - playable foundation".into(),
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
                camera_input.run_if(interactive),
                keyboard_input.run_if(interactive),
                pointer_input.run_if(interactive),
                button_input.run_if(interactive),
                update_units,
                update_hud,
                draw_tactical,
                smoke_driver,
            )
                .chain(),
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
) {
    commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(mat),
        Transform::from_translation(pos).with_scale(scale),
    ));
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
    battle: Res<Battle>,
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
    // Terrain, ruin and road geometry is driven by the actual navigation grid.
    for z in 0..i32::from(battle.world.map.height) {
        for x in 0..i32::from(battle.world.map.width) {
            let n = noise(x, z);
            let center = Vec3::new(x as f32 + 0.5, 0.0, z as f32 + 0.5);
            if !battle.world.map.is_walkable(x, z) {
                let h = 0.6 + n * 1.9;
                piece(
                    &mut commands,
                    cube.clone(),
                    if n > 0.65 {
                        pale.clone()
                    } else {
                        stone.clone()
                    },
                    center + Vec3::Y * h * 0.5,
                    Vec3::new(0.95, h, 0.95),
                );
                if n > 0.92 {
                    piece(
                        &mut commands,
                        cube.clone(),
                        pale.clone(),
                        center + Vec3::Y * (h + 0.1),
                        Vec3::new(1.13, 0.2, 1.13),
                    );
                }
            } else if (z - 18).abs() < 2 || x % 12 == 0 {
                piece(
                    &mut commands,
                    cube.clone(),
                    if n > 0.5 {
                        stone.clone()
                    } else {
                        basalt.clone()
                    },
                    center + Vec3::Y * 0.045,
                    Vec3::new(0.92, 0.08, 0.92),
                );
            } else if n > 0.9 {
                piece(
                    &mut commands,
                    sphere.clone(),
                    moss.clone(),
                    center + Vec3::Y * 0.12,
                    Vec3::new(0.7, 0.25, 0.65),
                );
            }
        }
    }
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
    for objective in &battle.world.objectives {
        let pos = point3(objective.position, 0.1);
        piece(
            &mut commands,
            cube.clone(),
            dark.clone(),
            pos,
            Vec3::new(2.4, 0.2, 2.4),
        );
        commands.spawn((
            RelicVisual(objective.id),
            Mesh3d(cone.clone()),
            MeshMaterial3d(gold.clone()),
            Transform::from_translation(pos + Vec3::Y * 1.3).with_scale(Vec3::new(0.9, 2.1, 0.9)),
        ));
        commands.spawn((
            Mesh3d(ring.clone()),
            MeshMaterial3d(gold.clone()),
            Transform::from_translation(pos + Vec3::Y * 0.2).with_scale(Vec3::splat(3.8)),
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
                        "THE HOLLOW MARCH    /    ORIGINAL GEOMETRY PLAYTEST",
                        11.0,
                        Color::srgb(0.55, 0.68, 0.69),
                    ));
                });
            parent.spawn((Hud, label("", 16.0, Color::srgb(0.82, 0.88, 0.85))));
        });
    commands.spawn((Node { position_type:PositionType::Absolute, bottom:px(0), left:px(0), right:px(0), min_height:px(138), padding:UiRect::all(px(18)), flex_direction:FlexDirection::Column, row_gap:px(12), ..default() }, BackgroundColor(UI_BG))).with_children(|parent| {
        parent.spawn((StatusLine, label("", 15.0, TEAL)));
        parent.spawn(Node { column_gap:px(8), flex_wrap:FlexWrap::Wrap, row_gap:px(6), ..default() }).with_children(|p| {
            for (text, action) in [("1  VANGUARD",Action::Vanguard),("2  RANGER",Action::Ranger),("SPACE  SOULFIRE",Action::Spell),("TAB  ARMY",Action::Army),("H  HOLD",Action::Hold),("P  PAUSE",Action::Pause),("R  RESTART",Action::Restart),("F1  HELP",Action::Help)] {
                p.spawn((UiButton, Hovered::default(), action, Node { padding:UiRect::axes(px(14),px(11)), border:UiRect::all(px(1)), ..default() }, BackgroundColor(Color::srgb(0.075, 0.105, 0.12)), BorderColor::all(Color::srgb(0.24, 0.31, 0.31)))).with_child(label(text,13.0,GOLD));
            }
        });
        parent.spawn(label("LEFT / DRAG select   |   RIGHT move   |   SHIFT + RIGHT attack-move   |   WASD pan   Q/E orbit   WHEEL zoom   |   F5 save   F9 load", 11.0, Color::srgb(0.54, 0.64, 0.65)));
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
) {
    let dt = time.delta_secs();
    let right = Vec3::new(orbit.yaw.cos(), 0.0, -orbit.yaw.sin());
    let forward = Vec3::new(-orbit.yaw.sin(), 0.0, -orbit.yaw.cos());
    let speed = orbit.height * 0.55 * dt;
    if keys.pressed(KeyCode::KeyW) {
        orbit.focus += forward * speed;
    }
    if keys.pressed(KeyCode::KeyS) {
        orbit.focus -= forward * speed;
    }
    if keys.pressed(KeyCode::KeyA) {
        orbit.focus -= right * speed;
    }
    if keys.pressed(KeyCode::KeyD) {
        orbit.focus += right * speed;
    }
    if keys.pressed(KeyCode::KeyQ) {
        orbit.yaw -= dt;
    }
    if keys.pressed(KeyCode::KeyE) {
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
    keys: Res<ButtonInput<KeyCode>>,
    mut battle: ResMut<Battle>,
    mut orbit: ResMut<Orbit>,
    mut exit: MessageWriter<AppExit>,
    mut commands: Commands,
) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
    if keys.just_pressed(KeyCode::Tab) {
        battle.all_army();
    }
    if keys.just_pressed(KeyCode::KeyC) {
        if let Some(unit) = battle
            .world
            .units
            .iter()
            .find(|u| u.kind == UnitKind::Commander && u.hp > 0)
        {
            let id = unit.id;
            let pos = point3(unit.position, 0.0);
            battle.selected.clear();
            battle.selected.insert(id);
            orbit.focus = pos;
        }
    }
    if keys.just_pressed(KeyCode::KeyP) {
        battle.paused = !battle.paused;
    }
    if keys.just_pressed(KeyCode::F1) {
        battle.help = !battle.help;
    }
    if keys.just_pressed(KeyCode::KeyR) {
        *battle = Battle::default();
    }
    if keys.just_pressed(KeyCode::Digit1) {
        battle.command(SimCommand::Recruit {
            kind: UnitKind::Vanguard,
        });
    }
    if keys.just_pressed(KeyCode::Digit2) {
        battle.command(SimCommand::Recruit {
            kind: UnitKind::Ranger,
        });
    }
    if keys.just_pressed(KeyCode::KeyH) {
        let ids = battle.selected_ids();
        battle.command(SimCommand::Hold { units: ids });
    }
    if keys.just_pressed(KeyCode::Space) {
        if let Some(target) = battle.cursor_world {
            battle.command(SimCommand::Cast { target });
        }
    }
    if keys.just_pressed(KeyCode::F5) {
        match battle.world.save_json() {
            Ok(data) => match std::fs::write("dreadmarch-save.json", data) {
                Ok(()) => battle.notice = "Saved dreadmarch-save.json".into(),
                Err(e) => battle.notice = format!("Save failed: {e}"),
            },
            Err(e) => battle.notice = format!("Save failed: {e:?}"),
        }
        battle.notice_ticks = 180;
    }
    if keys.just_pressed(KeyCode::F9) {
        match std::fs::read_to_string("dreadmarch-save.json") {
            Ok(data) => match SimWorld::load_json(&data) {
                Ok(world) => {
                    battle.world = world;
                    battle.all_army();
                    battle.notice = "Save restored and validated.".into();
                }
                Err(e) => battle.notice = format!("Load rejected: {e:?}"),
            },
            Err(e) => battle.notice = format!("Load failed: {e}"),
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
    let Some(cursor) = window.cursor_position() else {
        return;
    };
    let (camera, transform) = *camera;
    let over_ui = cursor.y < 90.0
        || cursor.y > window.height() - 145.0
        || (battle.help && cursor.x < 570.0 && cursor.y < 580.0);
    let ground = camera
        .viewport_to_world(transform, cursor)
        .ok()
        .and_then(|r| r.plane_intersection_point(Vec3::ZERO, InfinitePlane3d::new(Vec3::Y)));
    battle.cursor_world = ground.map(point2);
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
            battle.command(SimCommand::Move {
                units: ids,
                target,
                attack_move: keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight),
            });
            battle.order_marker = Some((point3(target, 0.16), 1.3));
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
                    if unit.team == Team::Player && unit.hp > 0 {
                        if let Ok(p) =
                            camera.world_to_viewport(transform, point3(unit.position, 0.5))
                        {
                            if p.x >= min.x && p.x <= max.x && p.y >= min.y && p.y <= max.y {
                                chosen.push(unit.id);
                            }
                        }
                    }
                }
            } else {
                let mut nearest = (22.0_f32, None);
                for unit in &battle.world.units {
                    if unit.team == Team::Player && unit.hp > 0 {
                        if let Ok(p) =
                            camera.world_to_viewport(transform, point3(unit.position, 0.5))
                        {
                            let d = p.distance(cursor);
                            if d < nearest.0 {
                                nearest = (d, Some(unit.id));
                            }
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
    mut buttons: Query<(&Hovered, Has<Pressed>, &mut BackgroundColor), With<UiButton>>,
) {
    for (hovered, pressed, mut bg) in &mut buttons {
        bg.0 = if pressed {
            Color::srgb(0.23, 0.30, 0.28)
        } else if hovered.get() {
            Color::srgb(0.14, 0.22, 0.24)
        } else {
            Color::srgb(0.075, 0.105, 0.12)
        };
    }
}
fn activate_button(event: On<Activate>, actions: Query<&Action>, mut battle: ResMut<Battle>) {
    let Ok(action) = actions.get(event.entity) else {
        return;
    };
    match action {
        Action::Vanguard => battle.command(SimCommand::Recruit {
            kind: UnitKind::Vanguard,
        }),
        Action::Ranger => battle.command(SimCommand::Recruit {
            kind: UnitKind::Ranger,
        }),
        Action::Spell => {
            battle.cast_mode = true;
            battle.notice = "Choose a ground target for Soulfire.".into();
            battle.notice_ticks = 150;
        }
        Action::Army => battle.all_army(),
        Action::Hold => {
            let ids = battle.selected_ids();
            battle.command(SimCommand::Hold { units: ids });
        }
        Action::Pause => battle.paused = !battle.paused,
        Action::Restart => *battle = Battle::default(),
        Action::Help => battle.help = !battle.help,
    }
}

fn step_simulation(mut battle: ResMut<Battle>, cli: Res<Cli>) {
    if !battle.paused && !cli.smoke {
        battle.advance();
    }
    if battle.notice_ticks > 0 {
        battle.notice_ticks -= 1;
    }
}

fn spawn_unit(commands: &mut Commands, assets: &VisualAssets, unit: &dread_sim::Unit) {
    let ally = unit.team == Team::Player;
    let commander = unit.kind == UnitKind::Commander;
    let brute = unit.kind == UnitKind::Brute;
    let ranger = unit.kind == UnitKind::Ranger;
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
    let accent = if ally {
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
    commands
        .spawn((
            UnitVisual {
                id: unit.id,
                previous: point3(unit.position, 0.08),
                current: point3(unit.position, 0.08),
                tick: 0,
            },
            Transform::from_translation(point3(unit.position, 0.08)).with_scale(Vec3::splat(scale)),
            Visibility::default(),
        ))
        .with_children(|parent| {
            for (mesh, mat, pos, size) in [
                (
                    assets.cone.clone(),
                    if ally {
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
                    assets.bone.clone(),
                    Vec3::new(0.0, 1.19, 0.0),
                    Vec3::splat(0.40),
                ),
                (
                    assets.cube.clone(),
                    accent.clone(),
                    Vec3::new(0.0, 1.35, 0.0),
                    Vec3::new(0.42, 0.16, 0.42),
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
                    glow.clone(),
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
            if commander {
                parent.spawn((
                    Mesh3d(assets.cone.clone()),
                    MeshMaterial3d(assets.gold.clone()),
                    Transform::from_xyz(0.0, 1.6, 0.0).with_scale(Vec3::new(0.28, 0.48, 0.28)),
                ));
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
    assets: Res<VisualAssets>,
    battle: Res<Battle>,
    time: Res<Time>,
    fixed: Res<Time<Fixed>>,
    cli: Res<Cli>,
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
    for (entity, mut visual, mut transform) in &mut visuals {
        if let Some(&index) = scratch.indices.get(&visual.id) {
            let unit = &battle.world.units[index];
            scratch.existing.insert(unit.id);
            let desired = point3(unit.position, 0.08);
            let delta = desired - visual.current;
            if visual.tick != battle.world.tick {
                visual.previous = if battle.world.tick < visual.tick {
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
            spawn_unit(&mut commands, &assets, unit);
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
    mut help: Single<
        (&mut Text, &mut Visibility),
        (With<HelpText>, Without<Hud>, Without<StatusLine>),
    >,
) {
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
    hud.0 = format!(
        "ESSENCE {}     ARMY {}     HOSTILES {}\nRELICS {}/{}     WAVE {}     KILLS {}",
        battle.world.economy.essence,
        army,
        enemy,
        captured,
        battle.world.objectives.len(),
        battle.world.wave,
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
    text.0="COMMAND THE HOLLOW MARCH\n\nSecure all three ancient relics and survive the invasion.\nKeep the commander alive; losing them ends the match.\n\nLeft-click a warrior, or drag to select a squad.\nShift adds to your selection. Tab selects the army.\nRight-click moves in formation. Shift + right-click\norders an attack-move. H stops the selected squad.\nC selects and centers the commander.\n\nSpace casts Soulfire at the cursor. The button lets\nyou choose the target with your next click.\n1 recruits a Vanguard. 2 recruits a Ranger.\n\nWASD pan / Q E orbit / Wheel zoom\nP pause / R restart / F5 save / F9 load\nF12 screenshot / Esc exit / F1 close help\n\nFoundation playtest: original procedural geometry.\nProduction art, audio, campaign and netcode remain\ntracked in the repository roadmap.".into();
}

fn draw_tactical(mut gizmos: Gizmos, mut battle: ResMut<Battle>, time: Res<Time>) {
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
    if battle.cast_mode {
        if let Some(target) = battle.cursor_world {
            gizmos.circle(
                Isometry3d::new(point3(target, 0.17), Quat::from_rotation_x(FRAC_PI_2)),
                ABILITY_RADIUS as f32 / 1000.0,
                GOLD,
            );
        }
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
    mut cli: ResMut<Cli>,
    mut battle: ResMut<Battle>,
    mut commands: Commands,
    mut exit: MessageWriter<AppExit>,
    offscreen: Option<Res<OffscreenTarget>>,
) {
    cli.frames += 1;
    if cli.smoke {
        battle.advance();
        if cli.frames == 12 {
            let ids = battle.selected_ids();
            battle.command(SimCommand::Move {
                units: ids,
                target: Point {
                    x: 18_000,
                    z: 18_000,
                },
                attack_move: true,
            });
            battle.command(SimCommand::Recruit {
                kind: UnitKind::Ranger,
            });
        }
        if cli.frames == 70 {
            battle.command(SimCommand::Cast {
                target: Point {
                    x: 18_000,
                    z: 18_000,
                },
            });
        }
    }
    if cli.frames == 90 {
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
    if cli.smoke && cli.frames >= 120 && (cli.screenshot.is_none() || cli.captured) {
        let saved = battle.world.save_json().expect("smoke save serialization");
        let restored = SimWorld::load_json(&saved).expect("smoke save validation");
        assert_eq!(
            battle.world.checksum(),
            restored.checksum(),
            "save roundtrip must preserve exact state"
        );
        assert_eq!(
            battle.command_count, 3,
            "smoke must issue movement, recruitment and ability"
        );
        assert_eq!(battle.command_failures, 0, "smoke commands must succeed");
        if let Some(path) = &cli.screenshot {
            assert!(
                std::fs::metadata(path).is_ok_and(|m| m.len() > 0),
                "captured PNG must exist"
            );
        }
        println!(
            "DREADMARCH_RENDER_SMOKE_OK frames={} tick={} units={} checksum={:016x} screenshot_captured={} commands_ok={} save_roundtrip=true",
            cli.frames,
            battle.world.tick,
            battle.world.units.len(),
            battle.world.checksum(),
            cli.captured,
            battle.command_count
        );
        exit.write(AppExit::Success);
    } else if cli.smoke && cli.frames >= 240 {
        eprintln!(
            "DREADMARCH_RENDER_SMOKE_FAILED: GPU capture did not finish within the finite render frame budget"
        );
        exit.write(AppExit::error());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
