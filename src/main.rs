use std::time::Duration;
use crate::reader::{init_loop, common::stable::get_game_state};

mod reader;

fn main() -> eyre::Result<()> {
    println!("Initializing osu! memory reader...");
    let (mut state, process) = init_loop(500)?;
    println!("Successfully initialized!");

    loop {
        let game_state = get_game_state(&process, &mut state)?;
        println!("Current game state: {:?}", game_state);
        
        // Sleep a bit to avoid hammering the CPU
        std::thread::sleep(Duration::from_millis(100));
    }
} 