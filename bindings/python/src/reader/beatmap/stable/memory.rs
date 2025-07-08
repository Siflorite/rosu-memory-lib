use pyo3::prelude::*;
use rosu_memory_lib::reader::{
    beatmap::{
        common::{BeatmapInfo, BeatmapLocation, BeatmapMetadata, BeatmapStats, BeatmapStatus, BeatmapTechnicalInfo},
        stable::memory as rust_memory,
    },
    structs::State,
};
use rosu_memory_lib::common::GameMode;
use crate::reader::beatmap::common::{PyBeatmapInfo, PyBeatmapLocation, PyBeatmapMetadata, PyBeatmapStats, PyBeatmapStatus, PyBeatmapTechnicalInfo};
use crate::reader::common::PyGameMode;

#[pyfunction]
pub fn get_beatmap_md5(process: &Process, state: &mut State) -> PyResult<String> {
    Ok(rust_memory::get_beatmap_md5(process, state)?)
}

#[pyfunction]
pub fn get_beatmap_id(process: &Process, state: &mut State) -> PyResult<i32> {
    Ok(rust_memory::get_beatmap_id(process, state)?)
}

#[pyfunction]
pub fn get_beatmap_set_id(process: &Process, state: &mut State) -> PyResult<i32> {
    Ok(rust_memory::get_beatmap_set_id(process, state)?)
}

#[pyfunction]
pub fn get_beatmap_mode(process: &Process, state: &mut State) -> PyResult<PyGameMode> {
    Ok(PyGameMode(rust_memory::get_beatmap_mode(process, state)?))
}

#[pyfunction]
pub fn get_beatmap_tags(process: &Process, state: &mut State) -> PyResult<String> {
    Ok(rust_memory::get_beatmap_tags(process, state)?)
}

#[pyfunction]
pub fn get_beatmap_length(process: &Process, state: &mut State) -> PyResult<i32> {
    Ok(rust_memory::get_beatmap_length(process, state)?)
}

#[pyfunction]
pub fn get_beatmap_drain_time(process: &Process, state: &mut State) -> PyResult<i32> {
    Ok(rust_memory::get_beatmap_drain_time(process, state)?)
}

#[pyfunction]
pub fn get_beatmap_status(process: &Process, state: &mut State) -> PyResult<PyBeatmapStatus> {
    Ok(PyBeatmapStatus(rust_memory::get_beatmap_status(process, state)?))
}

#[pyfunction]
pub fn get_author(process: &Process, state: &mut State) -> PyResult<String> {
    Ok(rust_memory::get_author(process, state)?)
}

#[pyfunction]
pub fn get_creator(process: &Process, state: &mut State) -> PyResult<String> {
    Ok(rust_memory::get_creator(process, state)?)
}

#[pyfunction]
pub fn get_title_romanized(process: &Process, state: &mut State) -> PyResult<String> {
    Ok(rust_memory::get_title_romanized(process, state)?)
}

#[pyfunction]
pub fn get_title_original(process: &Process, state: &mut State) -> PyResult<String> {
    Ok(rust_memory::get_title_original(process, state)?)
}

#[pyfunction]
pub fn get_difficulty(process: &Process, state: &mut State) -> PyResult<String> {
    Ok(rust_memory::get_difficulty(process, state)?)
}

#[pyfunction]
pub fn get_beatmap_od(process: &Process, state: &mut State) -> PyResult<f32> {
    Ok(rust_memory::get_beatmap_od(process, state)?)
}

#[pyfunction]
pub fn get_beatmap_ar(process: &Process, state: &mut State) -> PyResult<f32> {
    Ok(rust_memory::get_beatmap_ar(process, state)?)
}

#[pyfunction]
pub fn get_beatmap_cs(process: &Process, state: &mut State) -> PyResult<f32> {
    Ok(rust_memory::get_beatmap_cs(process, state)?)
}

#[pyfunction]
pub fn get_beatmap_hp(process: &Process, state: &mut State) -> PyResult<f32> {
    Ok(rust_memory::get_beatmap_hp(process, state)?)
}

#[pyfunction]
pub fn get_beatmap_object_count(process: &Process, state: &mut State) -> PyResult<u32> {
    Ok(rust_memory::get_beatmap_object_count(process, state)?)
}

#[pyfunction]
pub fn get_beatmap_slider_count(process: &Process, state: &mut State) -> PyResult<i32> {
    Ok(rust_memory::get_beatmap_slider_count(process, state)?)
}

#[pyfunction]
pub fn get_beatmap_stats(process: &Process, state: &mut State) -> PyResult<PyBeatmapStats> {
    let stats = rust_memory::get_beatmap_stats(process, state)?;
    Ok(PyBeatmapStats {
        ar: stats.ar,
        od: stats.od,
        cs: stats.cs,
        hp: stats.hp,
        total_length: stats.total_length,
        star_rating: stats.star_rating,
        object_count: stats.object_count,
        slider_count: stats.slider_count,
    })
}

#[pyfunction]
pub fn get_beatmap_info(process: &Process, state: &mut State) -> PyResult<PyBeatmapInfo> {
    let info = rust_memory::get_beatmap_info(process, state)?;
    Ok(PyBeatmapInfo {
        technical: PyBeatmapTechnicalInfo {
            md5: info.technical.md5,
            id: info.technical.id,
            set_id: info.technical.set_id,
            mode: PyGameMode(info.technical.mode),
            ranked_status: PyBeatmapStatus(info.technical.ranked_status),
        },
        metadata: PyBeatmapMetadata {
            author: info.metadata.author,
            creator: info.metadata.creator,
            title_romanized: info.metadata.title_romanized,
            title_original: info.metadata.title_original,
            difficulty: info.metadata.difficulty,
            tags: info.metadata.tags,
        },
        stats: PyBeatmapStats {
            ar: info.stats.ar,
            od: info.stats.od,
            cs: info.stats.cs,
            hp: info.stats.hp,
            total_length: info.stats.total_length,
            star_rating: info.stats.star_rating,
            object_count: info.stats.object_count,
            slider_count: info.stats.slider_count,
        },
        location: PyBeatmapLocation {
            folder: info.location.folder,
            filename: info.location.filename,
            audio: info.location.audio,
            cover: info.location.cover,
        },
    })
} 