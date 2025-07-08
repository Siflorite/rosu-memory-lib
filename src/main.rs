use std::time::Duration;
use rosu_memory_lib::{init_loop, common::stable::get_game_state, common::GameState};
use rosu_memory_lib::reader::beatmap::stable::info::get_beatmap_status;

fn main() -> eyre::Result<()> {
    println!("Initializing osu! memory reader...");
    let (mut state, process) = init_loop(500)?;
    println!("Successfully initialized!");

    loop {
        let game_state = get_game_state(&process, &mut state)?;
        if game_state == GameState::SongSelect {
            match get_beatmap_status(&process, &mut state) {
                Ok(beatmap_status) => println!("Current beatmap status: {:?}", beatmap_status),
                Err(_) => (), // ignore l'erreur silencieusement
            }
        }
        
        // Sleep a bit to avoid hammering the CPU
        std::thread::sleep(Duration::from_millis(100));
    }
} 