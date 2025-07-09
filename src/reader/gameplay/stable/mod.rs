pub mod memory;
pub mod offset;
use crate::reader::structs::State;
use crate::reader::gameplay::stable::offset::GAMEPLAY_OFFSET;
use rosu_mem::process::Process;
use rosu_mem::process::ProcessTraits;
use crate::reader::gameplay::common::GameplayInfo;

use crate::reader::structs::Hit;

pub fn get_gameplay_info(p: &Process, state: &mut State) -> Result<GameplayInfo, eyre::Error> {
    let base = memory::get_base(p, state)?;
    let base2 = p.read_i32(base + GAMEPLAY_OFFSET.base2).unwrap();

    let hp = p.read_f64(p.read_i32(base + GAMEPLAY_OFFSET.hp_base)?+ GAMEPLAY_OFFSET.hp)?;
    let mods = memory::get_mods(p, state)?;
 
    
    Ok(
        GameplayInfo {
        score: p.read_i32(base2 + GAMEPLAY_OFFSET.score).unwrap(),
        mods: mods,
        combo: p.read_i16(base2 + GAMEPLAY_OFFSET.combo).unwrap(),
        max_combo: p.read_i16(base2 + GAMEPLAY_OFFSET.max_combo).unwrap(),
        hp: hp,
        username: p.read_string(base2 + GAMEPLAY_OFFSET.username).unwrap(),
        ig_time: memory::get_ig_time(p, state)?, // different base
        retries: memory::get_retries(p, state)?, // different base
        hits : Hit {
            _300: p.read_i16(base2 + GAMEPLAY_OFFSET.hits._300)?,
            _100: p.read_i16(base2 + GAMEPLAY_OFFSET.hits._100)?,
            _50: p.read_i16(base2 + GAMEPLAY_OFFSET.hits._50)?,
            _miss: p.read_i16(base2 + GAMEPLAY_OFFSET.hits._miss)?,
            _geki: p.read_i16(base2 + GAMEPLAY_OFFSET.hits._geki)?,
            _katu: p.read_i16(base2 + GAMEPLAY_OFFSET.hits._katu)?,
        },
    })
}