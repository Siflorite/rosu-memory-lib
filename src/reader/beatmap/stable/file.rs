


use rosu_mem::process::{Process, ProcessTraits};
use rosu_map::Beatmap as RmBeatmap;
use rosu_map::section::hit_objects::HitObjectKind;
use crate::reader::structs::State;
use crate::reader::common::stable::get_path_folder;
use crate::reader::beatmap::common::{BeatmapInfo, BeatmapTechnicalInfo, BeatmapMetadata, BeatmapLocation, BeatmapStatus, BeatmapStats, BeatmapStarRating};
use crate::reader::beatmap::stable::location::{get_audio, get_filename, get_folder};
use crate::reader::beatmap::stable::{offset::BEATMAP_OFFSET, get_beatmap_addr};
use crate::common::GameMode;




pub fn get_beatmap_path(p: &Process, state: &mut State) -> eyre::Result<String>
{
    let folder = get_folder(p, state)?;
    let filename = get_filename(p, state)?;
    let song_path = get_path_folder(p, state)?;
    Ok(format!("{song_path}/{folder}/{filename}"))
}

pub fn get_audio_path(p: &Process, state: &mut State) -> eyre::Result<String>
{
    let folder = get_folder(p, state)?;
    let audio = get_audio(p, state)?;
    let song_path = get_path_folder(p, state)?;
    Ok(format!("{song_path}/{folder}/{audio}"))
}

pub fn get_beatmap_md5(p: &Process, state: &mut State) -> eyre::Result<String>
{
    // TODO: implement this for now will get from memory
    crate::reader::beatmap::stable::memory::get_beatmap_md5(p, state)
}

pub fn get_beatmap_id(p: &Process, state: &mut State) -> eyre::Result<i32>
{
    let path = get_beatmap_path(p, state)?;
    let b = RmBeatmap::from_path(path)?;
    Ok(b.beatmap_id)
}


pub fn get_beatmap_set_id(p: &Process, state: &mut State) -> eyre::Result<i32>
{
    let path = get_beatmap_path(p, state)?;
    let b = RmBeatmap::from_path(path)?;
    Ok(b.beatmap_set_id)

}

pub fn get_beatmap_mode(p: &Process, state: &mut State) -> eyre::Result<GameMode>
{
    let path = get_beatmap_path(p, state)?;
    let b = RmBeatmap::from_path(path)?;
    Ok(GameMode::from(b.mode as u32))
}

pub fn get_beatmap_tags(p: &Process, state: &mut State) -> eyre::Result<String>
{
    let path = get_beatmap_path(p, state)?;
    let b = RmBeatmap::from_path(path)?;
    Ok(b.tags)
}

pub fn get_beatmap_length(p: &Process, state: &mut State) -> eyre::Result<i32>
{

    // implement this later for now will get from memory
    crate::reader::beatmap::stable::memory::get_beatmap_length(p, state)
}

pub fn get_beatmap_drain_time(p: &Process, state: &mut State) -> eyre::Result<i32>
{
    let path = get_beatmap_path(p, state)?;
    let b = RmBeatmap::from_path(path)?;
    let drain_time = b.hit_objects.last().unwrap().start_time - b.hit_objects.first().unwrap().start_time;
    Ok(drain_time as i32)
}

pub fn get_beatmap_status(p: &Process, state: &mut State) -> eyre::Result<BeatmapStatus>
{
    // cant do this in file mode 
    crate::reader::beatmap::stable::memory::get_beatmap_status(p, state)
}

pub fn get_author(p: &Process, state: &mut State) -> eyre::Result<String>
{
    let path = get_beatmap_path(p, state)?;
    let b = RmBeatmap::from_path(path)?;
    Ok(b.artist)
}

pub fn get_creator(p: &Process, state: &mut State) -> eyre::Result<String>
{
    let path = get_beatmap_path(p, state)?;
    let b = RmBeatmap::from_path(path)?;
    Ok(b.creator)

}

pub fn get_title_romanized(p: &Process, state: &mut State) -> eyre::Result<String>
{
    let path = get_beatmap_path(p, state)?;
    let b = RmBeatmap::from_path(path)?;
    Ok(b.title)
}

pub fn get_title_original(p: &Process, state: &mut State) -> eyre::Result<String>
{
    let path = get_beatmap_path(p, state)?;
    let b = RmBeatmap::from_path(path)?;
    Ok(b.title_unicode)
}

pub fn get_difficulty(p: &Process, state: &mut State) -> eyre::Result<String>
{
    let path = get_beatmap_path(p, state)?;
    let b = RmBeatmap::from_path(path)?;
    Ok(b.version)
}

pub fn get_beatmap_od(p: &Process, state: &mut State) -> eyre::Result<f32>
{
    let path = get_beatmap_path(p, state)?;
    let b = RmBeatmap::from_path(path)?;
    Ok(b.overall_difficulty)
}

pub fn get_beatmap_ar(p: &Process, state: &mut State) -> eyre::Result<f32>
{
    let path = get_beatmap_path(p, state)?;
    let b = RmBeatmap::from_path(path)?;
    Ok(b.approach_rate)
}

pub fn get_beatmap_cs(p: &Process, state: &mut State) -> eyre::Result<f32>
{
    let path = get_beatmap_path(p, state)?;
    let b = RmBeatmap::from_path(path)?;
    Ok(b.circle_size)
}

pub fn get_beatmap_hp(p: &Process, state: &mut State) -> eyre::Result<f32>
{
    let path = get_beatmap_path(p, state)?;
    let b = RmBeatmap::from_path(path)?;
    Ok(b.hp_drain_rate)
}

pub fn get_beatmap_object_count(p: &Process, state: &mut State) -> eyre::Result<u32>
{
    let path = get_beatmap_path(p, state)?;
    let b = RmBeatmap::from_path(path)?;
    Ok(b.hit_objects.len() as u32)
}

pub fn get_beatmap_slider_count(p: &Process, state: &mut State) -> eyre::Result<i32>
{
    let path = get_beatmap_path(p, state)?;
    let b = RmBeatmap::from_path(path)?;
    Ok(b.hit_objects.iter().filter(|h| matches!(h.kind, HitObjectKind::Slider(_))).count() as i32)
}


pub fn get_beatmap_star_rating(p: &Process, state: &mut State) -> eyre::Result<BeatmapStarRating>
{
    let folder = get_folder(p, state)?;
    let filename = get_filename(p, state)?;
    let song_path = get_path_folder(p, state)?;
    let path = format!("{song_path}/{folder}/{filename}");
    let b = rosu_pp::Beatmap::from_path(path)?;
    let diff_attrs = rosu_pp::Difficulty::new().calculate(&b);
    let diff_dt = rosu_pp::Difficulty::new().mods(64).calculate(&b);
    let diff_ht = rosu_pp::Difficulty::new().mods(256).calculate(&b);
    let no_mod = diff_attrs.stars();
    let dt = diff_dt.stars();
    let ht = diff_ht.stars();
    Ok(BeatmapStarRating{
        no_mod,
        dt,
        ht,
    })

}

pub fn get_beatmap_stats(p: &Process, state: &mut State) -> eyre::Result<BeatmapStats>
{
    let beatmap_addr = get_beatmap_path(p, state)?;
    let b = RmBeatmap::from_path(beatmap_addr)?;
    Ok(BeatmapStats{
        ar: b.approach_rate,
        od: b.overall_difficulty,
        cs: b.circle_size,
        hp: b.hp_drain_rate,
        total_length: b.hit_objects.last().unwrap().start_time as i32 - b.hit_objects.first().unwrap().start_time as i32,
        star_rating: get_beatmap_star_rating(p, state)?,
        object_count: b.hit_objects.len() as i32,
        slider_count: b.hit_objects.iter().filter(|h| matches!(h.kind, HitObjectKind::Slider(_))).count() as i32,
    })
}



pub fn get_beatmap_info(p: &Process, state: &mut State) -> eyre::Result<BeatmapInfo>
{
    let beatmap_file = get_beatmap_path(p, state)?;
    println!("Beatmap file: {}", beatmap_file);
    let beatmap_addr = get_beatmap_addr(p, state)?;
    let b = RmBeatmap::from_path(beatmap_file)?;
       // done like that to be more efficient reading the string one by one would need to reload addr everytime which cost more
       Ok(BeatmapInfo {
        technical: BeatmapTechnicalInfo{
            md5: crate::reader::beatmap::stable::file::get_beatmap_md5(p, state)?,
            id: b.beatmap_id,
            set_id: b.beatmap_set_id,
            mode: GameMode::Osu,
            ranked_status: crate::reader::beatmap::stable::file::get_beatmap_status(p, state)?,
        },
        metadata: BeatmapMetadata{
            author: b.artist,
            creator: b.creator,
            title_romanized: b.title,
            title_original: b.title_unicode,
            difficulty: b.version,
            tags: b.tags,
        },
        stats: BeatmapStats{
            ar: b.approach_rate,
            od: b.overall_difficulty,
            cs: b.circle_size,
            hp: b.hp_drain_rate,
            total_length: b.hit_objects.last().unwrap().start_time as i32 - b.hit_objects.first().unwrap().start_time as i32,
            star_rating: get_beatmap_star_rating(p,state)?,
            object_count: b.hit_objects.len() as i32,
            slider_count: b.hit_objects.iter().filter(|h| matches!(h.kind, HitObjectKind::Slider(_))).count() as i32,
        },
        location: BeatmapLocation {
            folder: p.read_string(beatmap_addr + BEATMAP_OFFSET.location.folder)?,
            filename: p.read_string(beatmap_addr + BEATMAP_OFFSET.location.filename)?,
            audio: p.read_string(beatmap_addr + BEATMAP_OFFSET.location.audio)?,
            cover: p.read_string(beatmap_addr + BEATMAP_OFFSET.location.cover)?,
        },
    })
    
}   
