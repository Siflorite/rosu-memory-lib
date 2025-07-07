mod info;
mod offset;
mod location;

use rosu_mem::process::{Process, ProcessTraits};
use crate::reader::beatmap::common::BeatmapInfo;
use crate::reader::structs::State;
use crate::reader::beatmap::stable::offset::BEATMAP_OFFSET;


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


pub(crate) fn read_beatmap_info(p: &Process, state: &mut State) -> eyre::Result<BeatmapInfo>
{
    let beatmap_addr = get_beatmap_addr(p, state)?;

    // done like that to be more efficient reading the string one by one would need to reload addr everytime which cost more
    Ok(BeatmapInfo {
        author: p.read_string(beatmap_addr + BEATMAP_OFFSET.author)?,
        cover: p.read_string(beatmap_addr + BEATMAP_OFFSET.cover)?,
        creator: p.read_string(beatmap_addr + BEATMAP_OFFSET.creator)?,
        title: p.read_string(beatmap_addr + BEATMAP_OFFSET.title)?,
        difficulty: p.read_string(beatmap_addr + BEATMAP_OFFSET.difficulty)?,
        folder: p.read_string(beatmap_addr + BEATMAP_OFFSET.folder)?,
        filename: p.read_string(beatmap_addr + BEATMAP_OFFSET.filename)?,
        audio: p.read_string(beatmap_addr + BEATMAP_OFFSET.audio)?,
    })
}