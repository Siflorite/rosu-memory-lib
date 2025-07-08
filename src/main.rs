use std::time::Duration;
use rosu_memory_lib::{init_loop, common::stable::get_game_state, common::GameState};
use rosu_memory_lib::reader::beatmap::stable::file::get_beatmap_drain_time;

fn main() -> eyre::Result<()> {
    println!("Initializing osu! memory reader...");
    let (mut state, process) = init_loop(500)?;
    println!("Successfully initialized!");

    loop {
        let game_state = get_game_state(&process, &mut state)?;
        if game_state == GameState::SongSelect {
            match get_beatmap_drain_time(&process, &mut state) {
                Ok(drain_time) => println!("Current beatmap drain time: {:?}", drain_time),
                Err(e) => println!("Error during scan: {}", e),
            }
        }
        
        // Sleep a bit to avoid hammering the CPU
        std::thread::sleep(Duration::from_millis(100));
    }
} 