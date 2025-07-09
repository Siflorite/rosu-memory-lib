use rosu_mem::process::{Process, ProcessTraits};
use crate::reader::beatmap::stable::{offset::BEATMAP_OFFSET, get_beatmap_addr, read_from_beatmap_ptr_string};
use crate::reader::beatmap::common::{BeatmapInfo, BeatmapTechnicalInfo, BeatmapMetadata, BeatmapLocation, BeatmapStatus, BeatmapStats};
use crate::common::GameMode;
use crate::reader::structs::State;

pub fn get_beatmap_md5(p: &Process, state: &mut State) -> eyre::Result<String>
{
    read_from_beatmap_ptr_string(p,state, BEATMAP_OFFSET.technical.md5)
}

pub fn get_beatmap_id(p: &Process, state: &mut State) -> eyre::Result<i32>
{
    Ok(p.read_i32(get_beatmap_addr(p, state)? + BEATMAP_OFFSET.technical.id)?)
}


pub fn get_beatmap_set_id(p: &Process, state: &mut State) -> eyre::Result<i32>
{
    Ok(p.read_i32(get_beatmap_addr(p, state)? + BEATMAP_OFFSET.technical.set_id)?)
}

pub fn get_beatmap_mode(p: &Process, state: &mut State) -> eyre::Result<GameMode>
{
    Ok(GameMode::from(p.read_i32(get_beatmap_addr(p, state)? + BEATMAP_OFFSET.technical.mode)?))
}

pub fn get_beatmap_tags(p: &Process, state: &mut State) -> eyre::Result<String>
{
    read_from_beatmap_ptr_string(p,state,BEATMAP_OFFSET.metadata.tags)
}

pub fn get_beatmap_length(p: &Process, state: &mut State) -> eyre::Result<i32>
{
    Ok(p.read_i32(get_beatmap_addr(p, state)? + BEATMAP_OFFSET.stats.total_length)?)
}

pub fn get_beatmap_drain_time(p: &Process, state: &mut State) -> eyre::Result<i32>
{
    Ok(p.read_i32(get_beatmap_addr(p, state)? + BEATMAP_OFFSET.stats.drain_time)?)
}

pub fn get_beatmap_status(p: &Process, state: &mut State) -> eyre::Result<BeatmapStatus>
{
    Ok(BeatmapStatus::from(p.read_i32(get_beatmap_addr(p, state)? + BEATMAP_OFFSET.technical.ranked_status)?))
}

pub fn get_author(p: &Process, state: &mut State) -> eyre::Result<String>
{
    read_from_beatmap_ptr_string(p,state,BEATMAP_OFFSET.metadata.author)
}

pub fn get_creator(p: &Process, state: &mut State) -> eyre::Result<String>
{
    read_from_beatmap_ptr_string(p,state,BEATMAP_OFFSET.metadata.creator)

}

pub fn get_title_romanized(p: &Process, state: &mut State) -> eyre::Result<String>
{
    read_from_beatmap_ptr_string(p,state,BEATMAP_OFFSET.metadata.title_romanized)

}

pub fn get_title_original(p: &Process, state: &mut State) -> eyre::Result<String>
{
    read_from_beatmap_ptr_string(p,state,BEATMAP_OFFSET.metadata.title_original)
}

pub fn get_difficulty(p: &Process, state: &mut State) -> eyre::Result<String>
{
    read_from_beatmap_ptr_string(p,state,BEATMAP_OFFSET.metadata.difficulty)
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

pub fn get_beatmap_slider_count(p: &Process, state: &mut State) -> eyre::Result<i32>
{
    let beatmap_addr = get_beatmap_addr(p, state)?;
    Ok(p.read_i32(beatmap_addr + BEATMAP_OFFSET.stats.slider_count)?)
}




pub fn get_beatmap_stats(p: &Process, state: &mut State) -> eyre::Result<BeatmapStats>
{
    let beatmap_addr = get_beatmap_addr(p, state)?;
    Ok(BeatmapStats{
        ar: p.read_f32(beatmap_addr + BEATMAP_OFFSET.stats.ar)?,
        od: p.read_f32(beatmap_addr + BEATMAP_OFFSET.stats.od)?,
        cs: p.read_f32(beatmap_addr + BEATMAP_OFFSET.stats.cs)?,
        hp: p.read_f32(beatmap_addr + BEATMAP_OFFSET.stats.hp)?,
        total_length: p.read_i32(beatmap_addr + BEATMAP_OFFSET.stats.total_length)?,
        star_rating: crate::reader::beatmap::stable::file::get_beatmap_star_rating(p, state)?,
        object_count: p.read_i32(beatmap_addr + BEATMAP_OFFSET.stats.object_count)?,
        slider_count: p.read_i32(beatmap_addr + BEATMAP_OFFSET.stats.slider_count)?,
    })
}


pub fn get_beatmap_info(p: &Process, state: &mut State) -> eyre::Result<BeatmapInfo>
{
    let beatmap_addr = get_beatmap_addr(p, state)?;

    // done like that to be more efficient reading the string one by one would need to reload addr everytime which cost more
    Ok(BeatmapInfo {
        technical: BeatmapTechnicalInfo{
            md5: p.read_string(beatmap_addr + BEATMAP_OFFSET.technical.md5)?,
            id: p.read_i32(beatmap_addr + BEATMAP_OFFSET.technical.id)?,
            set_id: p.read_i32(beatmap_addr + BEATMAP_OFFSET.technical.set_id)?,
            mode: GameMode::Osu,
            ranked_status: BeatmapStatus::from(p.read_i32(beatmap_addr + BEATMAP_OFFSET.technical.ranked_status)?),
        },
        metadata: BeatmapMetadata{
            author: p.read_string(beatmap_addr + BEATMAP_OFFSET.metadata.author)?,
            creator: p.read_string(beatmap_addr + BEATMAP_OFFSET.metadata.creator)?,
            title_romanized: p.read_string(beatmap_addr + BEATMAP_OFFSET.metadata.title_romanized)?,
            title_original: p.read_string(beatmap_addr + BEATMAP_OFFSET.metadata.title_original)?,
            difficulty: p.read_string(beatmap_addr + BEATMAP_OFFSET.metadata.difficulty)?,
            tags: p.read_string(beatmap_addr + BEATMAP_OFFSET.metadata.tags)?,
        },
        stats: BeatmapStats{
            ar: p.read_f32(beatmap_addr + BEATMAP_OFFSET.stats.ar)?,
            od: p.read_f32(beatmap_addr + BEATMAP_OFFSET.stats.od)?,
            cs: p.read_f32(beatmap_addr + BEATMAP_OFFSET.stats.cs)?,
            hp: p.read_f32(beatmap_addr + BEATMAP_OFFSET.stats.hp)?,
            total_length: p.read_i32(beatmap_addr + BEATMAP_OFFSET.stats.total_length)?,
            star_rating: crate::reader::beatmap::stable::file::get_beatmap_star_rating(p,state)?,
            object_count: p.read_i32(beatmap_addr + BEATMAP_OFFSET.stats.object_count)?,
            slider_count: p.read_i32(beatmap_addr + BEATMAP_OFFSET.stats.slider_count)?,
        },
        location: BeatmapLocation {
            folder: p.read_string(beatmap_addr + BEATMAP_OFFSET.location.folder)?,
            filename: p.read_string(beatmap_addr + BEATMAP_OFFSET.location.filename)?,
            audio: p.read_string(beatmap_addr + BEATMAP_OFFSET.location.audio)?,
            cover: p.read_string(beatmap_addr + BEATMAP_OFFSET.location.cover)?,
        },
    })
}

