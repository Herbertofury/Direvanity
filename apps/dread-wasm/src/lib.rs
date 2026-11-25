//! Small C ABI boundary. Both clients execute the same authoritative Rust rules.
use dread_sim::{Command, World};
use std::cell::RefCell;

struct Bridge {
    world: World,
    input: Vec<u8>,
    output: Vec<u8>,
}

thread_local! {
    static BRIDGE: RefCell<Bridge> = RefCell::new(Bridge {
        world: World::new(20261124), input: Vec::new(), output: Vec::new(),
    });
}

#[unsafe(no_mangle)]
pub extern "C" fn new_game(seed: u32) {
    BRIDGE.with(|bridge| {
        let mut bridge = bridge.borrow_mut();
        bridge.world = World::new(u64::from(seed));
        bridge.input.clear();
        bridge.output.clear();
    });
}

/// Create a campaign from validated UTF-8 CharacterCreation JSON in input_buffer.
#[unsafe(no_mangle)]
pub extern "C" fn new_campaign(seed: u32) -> i32 {
    BRIDGE.with(|bridge| {
        let mut bridge = bridge.borrow_mut();
        let result = serde_json::from_slice::<dread_sim::rpg::CharacterCreation>(&bridge.input)
            .map_err(|e| e.to_string())
            .and_then(|creation| {
                World::new_campaign(u64::from(seed), creation).map_err(|e| e.to_string())
            });
        match result {
            Ok(world) => {
                bridge.world = world;
                bridge.output.clear();
                0
            }
            Err(error) => {
                bridge.output = error.into_bytes();
                1
            }
        }
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn class_catalogue() -> usize {
    BRIDGE.with(|bridge| {
        let mut bridge = bridge.borrow_mut();
        bridge.output = serde_json::to_vec(dread_sim::rpg::catalogue()).expect("valid catalogue");
        bridge.output.len()
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn campaign_locations() -> usize {
    BRIDGE.with(|bridge| {
        let mut bridge = bridge.borrow_mut();
        let points = bridge
            .world
            .campaign
            .as_ref()
            .map(dread_sim::rpg::campaign_points_of_interest)
            .unwrap_or_default();
        bridge.output = serde_json::to_vec(&points).expect("valid locations");
        bridge.output.len()
    })
}

/// Authoritative journal objectives, including oath, side quests and expedition state.
#[unsafe(no_mangle)]
pub extern "C" fn campaign_journal() -> usize {
    BRIDGE.with(|bridge| {
        let mut bridge = bridge.borrow_mut();
        let entries = bridge
            .world
            .campaign
            .as_ref()
            .map(dread_sim::rpg::journal)
            .unwrap_or_default();
        bridge.output = serde_json::to_vec(&entries).expect("valid journal");
        bridge.output.len()
    })
}
#[unsafe(no_mangle)]
pub extern "C" fn campaign_atlas() -> usize {
    BRIDGE.with(|bridge| {
        let mut bridge = bridge.borrow_mut();
        bridge.output = serde_json::to_vec(&dread_sim::rpg::atlas()).expect("valid atlas");
        bridge.output.len()
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn step(ticks: u32) {
    BRIDGE.with(|bridge| {
        let mut bridge = bridge.borrow_mut();
        for _ in 0..ticks {
            bridge.world.step();
        }
    });
}

/// Returns a writable command/save buffer, valid until the next bridge call.
#[unsafe(no_mangle)]
pub extern "C" fn input_buffer(len: usize) -> *mut u8 {
    BRIDGE.with(|bridge| {
        let mut bridge = bridge.borrow_mut();
        bridge.input.resize(len, 0);
        bridge.input.as_mut_ptr()
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn command() -> i32 {
    BRIDGE.with(|bridge| {
        let mut bridge = bridge.borrow_mut();
        match serde_json::from_slice::<Command>(&bridge.input) {
            Ok(command) => match bridge.world.apply(command) {
                Ok(()) => {
                    bridge.output.clear();
                    0
                }
                Err(error) => {
                    bridge.output = error.to_string().into_bytes();
                    2
                }
            },
            Err(error) => {
                bridge.output = error.to_string().into_bytes();
                1
            }
        }
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn snapshot() -> usize {
    BRIDGE.with(|bridge| {
        let mut bridge = bridge.borrow_mut();
        bridge.output = serde_json::to_vec(&bridge.world).expect("valid world serialization");
        bridge.output.len()
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn output_buffer() -> *const u8 {
    BRIDGE.with(|bridge| bridge.borrow().output.as_ptr())
}

/// Return a complete save as opaque UTF-8 JSON bytes in the output buffer.
/// JavaScript may parse a snapshot to render UI, but must preserve these exact
/// bytes for saving: u64 RNG/tick fields can exceed Number.MAX_SAFE_INTEGER.
/// Reacquire memory.buffer after this call, because serialization may grow WASM
/// memory and detach an earlier JavaScript ArrayBuffer view.
#[unsafe(no_mangle)]
pub extern "C" fn save_game() -> usize {
    snapshot()
}

#[unsafe(no_mangle)]
pub extern "C" fn output_len() -> usize {
    BRIDGE.with(|bridge| bridge.borrow().output.len())
}

#[unsafe(no_mangle)]
pub extern "C" fn load_game() -> i32 {
    BRIDGE.with(|bridge| {
        let mut bridge = bridge.borrow_mut();
        let result = std::str::from_utf8(&bridge.input)
            .map_err(|error| error.to_string())
            .and_then(|json| World::load_json(json).map_err(|error| error.to_string()));
        match result {
            Ok(world) => {
                bridge.world = world;
                bridge.output.clear();
                0
            }
            Err(error) => {
                bridge.output = error.into_bytes();
                1
            }
        }
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn checksum_low() -> u32 {
    BRIDGE.with(|bridge| bridge.borrow().world.checksum() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bridge_exercises_real_simulation() {
        new_game(7);
        let before = checksum_low();
        step(30);
        assert_ne!(before, checksum_low());
        assert!(snapshot() > 100);
    }
    #[test]
    fn malformed_command_is_rejected() {
        BRIDGE.with(|bridge| bridge.borrow_mut().input = b"{bad}".to_vec());
        assert_eq!(command(), 1);
        assert!(output_len() > 0);
    }

    #[test]
    fn invalid_save_preserves_the_entire_current_world() {
        new_game(7);
        step(150);
        let before = BRIDGE.with(|bridge| bridge.borrow().world.save_json().unwrap());
        BRIDGE.with(|bridge| bridge.borrow_mut().input = b"{broken".to_vec());
        assert_eq!(load_game(), 1);
        BRIDGE.with(|bridge| assert_eq!(bridge.borrow().world.save_json().unwrap(), before));
    }

    #[test]
    fn command_errors_use_player_facing_messages() {
        new_game(7);
        BRIDGE.with(|bridge| bridge.borrow_mut().input = br#"{"type":"hold","units":[]}"#.to_vec());
        assert_eq!(command(), 2);
        BRIDGE.with(|bridge| {
            assert_eq!(
                std::str::from_utf8(&bridge.borrow().output).unwrap(),
                "Select one or more allied units first."
            );
        });
    }
}
