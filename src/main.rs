use std::time::Duration;
use rosu_memory_lib::{init_loop, common::stable::get_game_state, common::GameState, common::OsuType, reader::beatmap::get_beatmap_info};

fn main() -> eyre::Result<()> {
    println!("Initializing osu! memory reader...");
    let (mut state, process) = init_loop(500)?;
    println!("Successfully initialized!");

    loop {
        let game_state = get_game_state(&process, &mut state)?;
        if game_state == GameState::ResultScreen {
            let beamtap = get_beatmap_info(&process, &mut state, OsuType::Stable)?;
            println!("Beatmap info: {:?}", beamtap);
        }
        
        // Sleep a bit to avoid hammering the CPU
        std::thread::sleep(Duration::from_millis(100));
    }
} 