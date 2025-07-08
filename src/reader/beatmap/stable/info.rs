use rosu_mem::process::{Process};
use crate::reader::structs::State;
use crate::reader::beatmap::stable::offset::BEATMAP_OFFSET;
use crate::reader::beatmap::stable::read_from_beatmap_ptr_string;
use rosu_mem::process::ProcessTraits;
use crate::reader::beatmap::common::BeatmapStats;
use crate::reader::beatmap::stable::get_beatmap_addr;
use crate::common::GameMode;
use crate::reader::beatmap::common::BeatmapStatus;

pub fn get_beatmap_md5(p: &Process, state: &mut State) -> eyre::Result<String>
{
    Ok(read_from_beatmap_ptr_string(p,state, BEATMAP_OFFSET.technical.md5)?)
}

pub fn get_beatmap_id(p: &Process, state: &mut State) -> eyre::Result<i32>
{
    Ok(p.read_i32(get_beatmap_addr(p, state)? + BEATMAP_OFFSET.technical.id)?)
}


pub fn get_beatmap_set_id(p: &Process, state: &mut State) -> eyre::Result<i32>
{
    Ok(p.read_i32(get_beatmap_addr(p, state)? + BEATMAP_OFFSET.technical.set_id)?)
}


/// TODO REALLY IMPORTANT
pub fn get_beatmap_mode(p: &Process, state: &mut State) -> eyre::Result<GameMode>
{
    Ok(GameMode::from(p.read_i32(get_beatmap_addr(p, state)? + BEATMAP_OFFSET.technical.mode)?))
}

pub fn get_beatmap_length(p: &Process, state: &mut State) -> eyre::Result<i32>
{
    Ok(p.read_i32(get_beatmap_addr(p, state)? + BEATMAP_OFFSET.stats.total_length)?)
}


pub fn get_beatmap_status(p: &Process, state: &mut State) -> eyre::Result<BeatmapStatus>
{
    Ok(BeatmapStatus::from(p.read_i32(get_beatmap_addr(p, state)? + BEATMAP_OFFSET.technical.ranked_status)?))
}

pub fn get_author(p: &Process, state: &mut State) -> eyre::Result<String>
{
    Ok(read_from_beatmap_ptr_string(p,state,BEATMAP_OFFSET.metadata.author)?)
}

pub fn get_creator(p: &Process, state: &mut State) -> eyre::Result<String>
{
    Ok(read_from_beatmap_ptr_string(p,state,BEATMAP_OFFSET.metadata.creator)?)

}

pub fn get_title_romanized(p: &Process, state: &mut State) -> eyre::Result<String>
{
    Ok(read_from_beatmap_ptr_string(p,state,BEATMAP_OFFSET.metadata.title_romanized)?)

}

pub fn get_title_original(p: &Process, state: &mut State) -> eyre::Result<String>
{
    Ok(read_from_beatmap_ptr_string(p,state,BEATMAP_OFFSET.metadata.title_original)?)
}

pub fn get_difficulty(p: &Process, state: &mut State) -> eyre::Result<String>
{
    Ok(read_from_beatmap_ptr_string(p,state,BEATMAP_OFFSET.metadata.difficulty)?)
}

pub fn get_beatmap_od(p: &Process, state: &mut State) -> eyre::Result<f32>
{
    let beatmap_addr = get_beatmap_addr(p, state)?;
    Ok(p.read_f32(beatmap_addr + BEATMAP_OFFSET.stats.od)?)
}

pub fn get_beatmap_ar(p: &Process, state: &mut State) -> eyre::Result<f32>
{
    let beatmap_addr = get_beatmap_addr(p, state)?;
    Ok(p.read_f32(beatmap_addr + BEATMAP_OFFSET.stats.ar)?)
}

pub fn get_beatmap_cs(p: &Process, state: &mut State) -> eyre::Result<f32>
{
    let beatmap_addr = get_beatmap_addr(p, state)?;
    Ok(p.read_f32(beatmap_addr + BEATMAP_OFFSET.stats.cs)?)
}

pub fn get_beatmap_hp(p: &Process, state: &mut State) -> eyre::Result<f32>
{
    let beatmap_addr = get_beatmap_addr(p, state)?;
    Ok(p.read_f32(beatmap_addr + BEATMAP_OFFSET.stats.hp)?)
}

pub fn get_beatmap_object_count(p: &Process, state: &mut State) -> eyre::Result<u32>
{
    let beatmap_addr = get_beatmap_addr(p, state)?;
    Ok(p.read_u32(beatmap_addr + BEATMAP_OFFSET.stats.object_count)?)
}

pub fn get_beatmap_stats(p: &Process, state: &mut State) -> eyre::Result<BeatmapStats>
{
    let beatmap_addr = get_beatmap_addr(p, state)?;
    Ok(BeatmapStats{
        ar: p.read_f32(beatmap_addr + BEATMAP_OFFSET.stats.ar)?,
        od: p.read_f32(beatmap_addr + BEATMAP_OFFSET.stats.od)?,
        cs: p.read_f32(beatmap_addr + BEATMAP_OFFSET.stats.cs)?,
        hp: p.read_f32(beatmap_addr + BEATMAP_OFFSET.stats.hp)?,
        total_length: 0,
        star_rating: 0.0,
        object_count: 0,
    })
}