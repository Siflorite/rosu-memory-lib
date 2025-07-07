use rosu_mem::process::{Process};
use crate::reader::structs::State;
use crate::reader::beatmap::stable::offset::BEATMAP_OFFSET;
use crate::reader::beatmap::stable::read_from_beatmap_ptr_string;

pub(crate) fn get_beatmap_md5(p: &Process, state: &mut State) -> eyre::Result<String>
{
    Ok(read_from_beatmap_ptr_string(p,state, BEATMAP_OFFSET.md5)?)
}

pub(crate) fn get_author(p: &Process, state: &mut State) -> eyre::Result<String>
{
    Ok(read_from_beatmap_ptr_string(p,state,BEATMAP_OFFSET.author)?)
}

pub(crate) fn get_creator(p: &Process, state: &mut State) -> eyre::Result<String>
{
    Ok(read_from_beatmap_ptr_string(p,state,BEATMAP_OFFSET.creator)?)

}

pub(crate) fn get_title(p: &Process, state: &mut State) -> eyre::Result<String>
{
    Ok(read_from_beatmap_ptr_string(p,state,BEATMAP_OFFSET.title)?)

}

pub(crate) fn get_difficulty(p: &Process, state: &mut State) -> eyre::Result<String>
{
    Ok(read_from_beatmap_ptr_string(p,state,BEATMAP_OFFSET.difficulty)?)

}

