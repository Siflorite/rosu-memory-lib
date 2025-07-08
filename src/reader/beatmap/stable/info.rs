use rosu_mem::process::{Process};
use crate::reader::structs::State;
use crate::reader::beatmap::stable::offset::BEATMAP_OFFSET;
use crate::reader::beatmap::stable::read_from_beatmap_ptr_string;

pub fn get_beatmap_md5(p: &Process, state: &mut State) -> eyre::Result<String>
{
    Ok(read_from_beatmap_ptr_string(p,state, BEATMAP_OFFSET.md5)?)
}

pub fn get_author(p: &Process, state: &mut State) -> eyre::Result<String>
{
    Ok(read_from_beatmap_ptr_string(p,state,BEATMAP_OFFSET.author)?)
}

pub fn get_creator(p: &Process, state: &mut State) -> eyre::Result<String>
{
    Ok(read_from_beatmap_ptr_string(p,state,BEATMAP_OFFSET.creator)?)

}

pub fn get_title(p: &Process, state: &mut State) -> eyre::Result<String>
{
    Ok(read_from_beatmap_ptr_string(p,state,BEATMAP_OFFSET.title)?)

}

pub fn get_difficulty(p: &Process, state: &mut State) -> eyre::Result<String>
{
    Ok(read_from_beatmap_ptr_string(p,state,BEATMAP_OFFSET.difficulty)?)

}

