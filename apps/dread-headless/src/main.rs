use dread_sim::{Command, Point, Team, UnitKind, World};
use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let verify = args.iter().any(|arg| arg == "--verify");
    let ticks = args
        .windows(2)
        .find(|pair| pair[0] == "--ticks")
        .map_or(900, |pair| {
            pair[1].parse::<u32>().expect("--ticks requires a number")
        });
    let mut first = World::new(20261124);
    let mut replay = World::new(20261124);
    let start = Instant::now();
    for tick in 0..ticks {
        if tick == 0 || tick % 120 == 0 {
            let target = Point { x: 25500, z: 18500 };
            let command = Command::Move {
                units: first
                    .units
                    .iter()
                    .filter(|unit| unit.team == Team::Player && unit.hp > 0)
                    .map(|unit| unit.id)
                    .collect(),
                target,
                attack_move: true,
            };
            first.apply(command.clone()).expect("valid move");
            replay.apply(command).expect("valid replay move");
        }
        if tick % 150 == 0 {
            let command = Command::Recruit {
                kind: UnitKind::Ranger,
            };
            assert_eq!(
                first.apply(command.clone()).is_ok(),
                replay.apply(command).is_ok()
            );
        }
        first.step();
        replay.step();
        if verify {
            assert_eq!(
                first.checksum(),
                replay.checksum(),
                "replay diverged at {tick}"
            );
        }
    }
    let elapsed = start.elapsed();
    let json = first.save_json().expect("save serializes");
    let restored = World::load_json(&json).expect("save validates");
    assert_eq!(
        first.checksum(),
        restored.checksum(),
        "save roundtrip diverged"
    );
    assert_eq!(first.checksum(), replay.checksum(), "replay diverged");
    println!(
        "{}",
        serde_json::json!({
            "schema": 1, "seed":20261124, "requested_ticks":ticks, "actual_tick":first.tick,
            "two_worlds_elapsed_ms":elapsed.as_secs_f64()*1000.0,
            "average_world_step_us_including_script_and_verification":elapsed.as_secs_f64()*1_000_000.0 / (f64::from(ticks.max(1))*2.0),
            "unit_records":first.units.len(), "checksum":format!("{:016x}",first.checksum()),
            "replay_verified":true, "save_roundtrip_verified":true,
            "status":format!("{:?}",first.status),
            "warning":"CPU simulation evidence only; not a rendering or target-hardware FPS benchmark."
        })
    );
}
