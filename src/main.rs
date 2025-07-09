use std::time::{Duration, Instant};
use rosu_memory_lib::{init_loop};
use rosu_memory_lib::reader::resultscreen::stable::memory::get_result_screen;
use rosu_memory_lib::reader::user::stable::{get_user_profile};
use rosu_memory_lib::reader::beatmap::stable::memory::get_beatmap_info;

fn main() -> eyre::Result<()> {
    println!("Initializing osu! memory reader...");
    let (mut state, process) = init_loop(500)?;
    println!("Successfully initialized!");

    //loop {
        let start: Instant = Instant::now();
        match get_beatmap_info(&process, &mut state) {
                Ok(user) => println!(" user: {user:?} \n Time taken: {:?}", start.elapsed()),
                Err(e) => println!("Error: {e}"),
        }

        
        // Sleep a bit to avoid hammering the CPU
        std::thread::sleep(Duration::from_millis(100));
        Ok(())
    //}
} 