use rosu_memory_lib::reader::beatmap::{BeatmapReader};
use rosu_memory_lib::reader::common::OsuType;

use rosu_memory_lib::init_loop;


fn main() -> eyre::Result<()> {
    let (mut state, process) = init_loop(500)?;
    println!("Successfully initialized!");
    let mut beatmap_reader = BeatmapReader::new(&process, &mut state, OsuType::Stable)?;
    loop {
        match beatmap_reader.get_beatmap_info() {
                Ok(beatmap_info) => println!("Current beatmap info: {beatmap_info:?}"),
                Err(e) => println!("Error: {e:?}"),
        }
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
