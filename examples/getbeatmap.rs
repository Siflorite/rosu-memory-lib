use rosu_memory_lib::reader::beatmap::stable::memory::get_beatmap_info;
use rosu_memory_lib::init_loop;
use rosu_memory_lib::reader::common::GameState;

fn main() -> eyre::Result<()> {
    let (mut state, process) = init_loop(500)?;
    println!("Successfully initialized!");
    loop {
        match get_beatmap_info(&process, &mut state,Some(GameState::Editor)) {
                Ok(beatmap_info) => println!("Current beatmap info: {:?}", beatmap_info),
                Err(e) => println!("Error: {}", e),

        }
    }
}
