use std::time::{Duration, Instant};
use rosu_memory_lib::{init_loop};
use rosu_memory_lib::reader::user::stable::{memory::get_user_profile};

fn main() -> eyre::Result<()> {
    println!("Initializing osu! memory reader...");
    let (mut state, process) = init_loop(500)?;
    println!("Successfully initialized!");

    //loop {
        let start: Instant = Instant::now();
        match get_user_profile(&process, &mut state) {
                Ok(user) => println!(" user: {user:?} \n Time taken: {:?}", start.elapsed()),
                Err(e) => println!("Error: {e}"),
        }

        
        // Sleep a bit to avoid hammering the CPU
        std::thread::sleep(Duration::from_millis(100));
        Ok(())
    //}
} 