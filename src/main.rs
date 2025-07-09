use std::time::{Duration, Instant};
use rosu_memory_lib::{init_loop};
use rosu_memory_lib::reader::resultscreen::stable::memory::get_result_screen;

fn main() -> eyre::Result<()> {
    println!("Initializing osu! memory reader...");
    let (mut state, process) = init_loop(500)?;
    println!("Successfully initialized!");

    loop {
        let start: Instant = Instant::now();
        match get_result_screen(&process, &mut state) {
                Ok(beatmap_info) => println!("Current beatmap info: {:?}\nTime taken: {:?}", beatmap_info, start.elapsed()),
                Err(e) => println!("Error: {}", e),
        }
    
        
        // Sleep a bit to avoid hammering the CPU
        std::thread::sleep(Duration::from_millis(100));
    }
} 