//! Exercise a complete skirmish using only the public player-command interface.
//! This is a deterministic test controller, not an alternate game simulation.
use dread_sim::{ABILITY_RANGE, Command, MatchStatus, Team, UnitKind, World};

fn main() {
    let mut world = World::new(2026);
    let mut restored_mid_battle = false;
    for step in 0..9_000 {
        if world.status != MatchStatus::Playing {
            break;
        }
        if step % 90 == 0
            && let Some(objective) = world.objectives.iter().find(|o| !o.captured)
        {
            let target = objective.position;
            world
                .apply(Command::Move {
                    units: world.player_ids(),
                    target,
                    attack_move: true,
                })
                .expect("reachable relic order");
        }
        if step % 120 == 0 && world.economy.essence >= 35 {
            world
                .apply(Command::Recruit {
                    kind: if step % 240 == 0 {
                        UnitKind::Ranger
                    } else {
                        UnitKind::Vanguard
                    },
                })
                .expect("affordable recruitment");
        }
        if world.commander_cooldown == 0
            && let Some(commander) = world.commander()
        {
            let position = commander.position;
            if let Some(enemy) = world
                .units
                .iter()
                .filter(|u| {
                    u.team == Team::Enemy
                        && u.position.distance_squared(position) <= (ABILITY_RANGE as u64).pow(2)
                })
                .min_by_key(|u| (u.position.distance_squared(position), u.id))
            {
                world
                    .apply(Command::Cast {
                        target: enemy.position,
                    })
                    .expect("in-range pulse");
            }
        }
        world.step();
        if step == 2_000 {
            world = World::load_json(&world.save_json().expect("serialize battle"))
                .expect("restore battle");
            restored_mid_battle = true;
        }
    }
    world.validate().expect("valid end state");
    println!(
        "{{\"status\":\"{:?}\",\"tick\":{},\"waves\":{},\"captures\":{},\"kills\":{},\"survivors\":{},\"mid_battle_save_restore\":{},\"checksum\":\"{:016x}\"}}",
        world.status,
        world.tick,
        world.wave,
        world.objectives.iter().filter(|o| o.captured).count(),
        world.economy.kills,
        world
            .units
            .iter()
            .filter(|u| u.team == Team::Player)
            .count(),
        restored_mid_battle,
        world.checksum()
    );
    assert_eq!(
        world.status,
        MatchStatus::Victory,
        "scripted public input should complete the full default skirmish"
    );
    assert!(restored_mid_battle);
}
