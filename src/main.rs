use std::time::{Duration, Instant};
use rosu_memory_lib::{init_loop, common::stable::get_game_state, common::GameState};
use rosu_memory_lib::reader::beatmap::stable::file::get_beatmap_info;

fn main() -> eyre::Result<()> {
    println!("Initializing osu! memory reader...");
    let (mut state, process) = init_loop(500)?;
    println!("Successfully initialized!");

    loop {
        let game_state = get_game_state(&process, &mut state)?;
        if game_state == GameState::SongSelect {
            
            let start = Instant::now();
            match get_beatmap_info(&process, &mut state) {
                Ok(beatmap_info) => println!("Current beatmap info: {:?}\nTime taken: {:?}", beatmap_info, start.elapsed()),
                Err(e) => println!("Error during scan: {}", e),
            }
        }
        
        // Sleep a bit to avoid hammering the CPU
        std::thread::sleep(Duration::from_millis(100));
    }
} 