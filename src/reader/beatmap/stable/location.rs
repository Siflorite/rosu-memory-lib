use rosu_mem::process::{Process};
use crate::reader::structs::State;
use crate::reader::beatmap::stable::offset::BEATMAP_OFFSET;
use crate::reader::beatmap::stable::read_from_beatmap_ptr_string;



pub(crate) fn get_folder(p: &Process, state: &mut State) -> eyre::Result<String> {
    Ok(read_from_beatmap_ptr_string(p,state,BEATMAP_OFFSET.folder)?)

}
pub(crate) fn get_filename(p: &Process, state: &mut State) -> eyre::Result<String> {
    Ok(read_from_beatmap_ptr_string(p,state,BEATMAP_OFFSET.filename)?)

}

pub(crate) fn get_audio(p: &Process, state: &mut State) -> eyre::Result<String> {
    Ok(read_from_beatmap_ptr_string(p,state,BEATMAP_OFFSET.audio)?)
}

pub(crate) fn get_beatmap_cover(p: &Process, state: &mut State) -> eyre::Result<String> {
    Ok(read_from_beatmap_ptr_string(p,state,BEATMAP_OFFSET.cover)?)
}
