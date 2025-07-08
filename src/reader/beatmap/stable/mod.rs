pub mod memory;
pub mod offset;
pub mod location;
pub mod file;

use rosu_mem::process::{Process, ProcessTraits};
use crate::reader::structs::State;
use crate::reader::beatmap::stable::offset::*;





pub(crate) fn get_beatmap_addr(p: &Process, state: &mut State) -> eyre::Result<i32>
{
    let beatmap_ptr = p.read_i32(state.addresses.base - BEATMAP_OFFSET.ptr)?;
    Ok(p.read_i32(beatmap_ptr)?)
}

pub(crate) fn read_from_beatmap_ptr_string(p: &Process, state: &mut State, offset: i32) -> eyre::Result<String>
{
    let beatmap_addr = get_beatmap_addr(p, state)?;
    Ok(p.read_string(beatmap_addr + offset)?)
}


