use pyo3::prelude::*;

mod reader;

use reader::{
    beatmap::{
        common::{PyBeatmapInfo, PyBeatmapLocation, PyBeatmapMetadata, PyBeatmapStats, PyBeatmapStatus, PyBeatmapTechnicalInfo},
        stable::{memory, file},
    },
    common::PyGameMode,
};

/// Python module for rosu-memory-lib
#[pymodule]
fn rosu_memory_python(_py: Python, m: &PyModule) -> PyResult<()> {
    // Add common types
    m.add_class::<PyGameMode>()?;
    m.add_class::<PyBeatmapStatus>()?;
    m.add_class::<PyBeatmapStats>()?;
    m.add_class::<PyBeatmapLocation>()?;
    m.add_class::<PyBeatmapMetadata>()?;
    m.add_class::<PyBeatmapTechnicalInfo>()?;
    m.add_class::<PyBeatmapInfo>()?;

    // Add beatmap memory functions
    m.add_function(wrap_pyfunction!(memory::get_beatmap_md5, m)?)?;
    m.add_function(wrap_pyfunction!(memory::get_beatmap_id, m)?)?;
    m.add_function(wrap_pyfunction!(memory::get_beatmap_set_id, m)?)?;
    m.add_function(wrap_pyfunction!(memory::get_beatmap_mode, m)?)?;
    m.add_function(wrap_pyfunction!(memory::get_beatmap_tags, m)?)?;
    m.add_function(wrap_pyfunction!(memory::get_beatmap_length, m)?)?;
    m.add_function(wrap_pyfunction!(memory::get_beatmap_drain_time, m)?)?;
    m.add_function(wrap_pyfunction!(memory::get_beatmap_status, m)?)?;
    m.add_function(wrap_pyfunction!(memory::get_author, m)?)?;
    m.add_function(wrap_pyfunction!(memory::get_creator, m)?)?;
    m.add_function(wrap_pyfunction!(memory::get_title_romanized, m)?)?;
    m.add_function(wrap_pyfunction!(memory::get_title_original, m)?)?;
    m.add_function(wrap_pyfunction!(memory::get_difficulty, m)?)?;
    m.add_function(wrap_pyfunction!(memory::get_beatmap_od, m)?)?;
    m.add_function(wrap_pyfunction!(memory::get_beatmap_ar, m)?)?;
    m.add_function(wrap_pyfunction!(memory::get_beatmap_cs, m)?)?;
    m.add_function(wrap_pyfunction!(memory::get_beatmap_hp, m)?)?;
    m.add_function(wrap_pyfunction!(memory::get_beatmap_object_count, m)?)?;
    m.add_function(wrap_pyfunction!(memory::get_beatmap_slider_count, m)?)?;
    m.add_function(wrap_pyfunction!(memory::get_beatmap_stats, m)?)?;
    m.add_function(wrap_pyfunction!(memory::get_beatmap_info, m)?)?;

    // Add beatmap file functions
    m.add_function(wrap_pyfunction!(file::get_beatmap_path, m)?)?;
    m.add_function(wrap_pyfunction!(file::get_audio_path, m)?)?;
    m.add_function(wrap_pyfunction!(file::get_beatmap_md5, m)?)?;
    m.add_function(wrap_pyfunction!(file::get_beatmap_id, m)?)?;
    m.add_function(wrap_pyfunction!(file::get_beatmap_set_id, m)?)?;
    m.add_function(wrap_pyfunction!(file::get_beatmap_mode, m)?)?;
    m.add_function(wrap_pyfunction!(file::get_beatmap_tags, m)?)?;
    m.add_function(wrap_pyfunction!(file::get_beatmap_length, m)?)?;
    m.add_function(wrap_pyfunction!(file::get_beatmap_drain_time, m)?)?;
    m.add_function(wrap_pyfunction!(file::get_beatmap_status, m)?)?;
    m.add_function(wrap_pyfunction!(file::get_author, m)?)?;
    m.add_function(wrap_pyfunction!(file::get_creator, m)?)?;
    m.add_function(wrap_pyfunction!(file::get_title_romanized, m)?)?;
    m.add_function(wrap_pyfunction!(file::get_title_original, m)?)?;
    m.add_function(wrap_pyfunction!(file::get_difficulty, m)?)?;
    m.add_function(wrap_pyfunction!(file::get_beatmap_od, m)?)?;
    m.add_function(wrap_pyfunction!(file::get_beatmap_ar, m)?)?;
    m.add_function(wrap_pyfunction!(file::get_beatmap_cs, m)?)?;
    m.add_function(wrap_pyfunction!(file::get_beatmap_hp, m)?)?;
    m.add_function(wrap_pyfunction!(file::get_beatmap_object_count, m)?)?;
    m.add_function(wrap_pyfunction!(file::get_beatmap_slider_count, m)?)?;
    m.add_function(wrap_pyfunction!(file::get_beatmap_star_rating, m)?)?;
    m.add_function(wrap_pyfunction!(file::get_beatmap_stats, m)?)?;
    m.add_function(wrap_pyfunction!(file::get_beatmap_info, m)?)?;

    Ok(())
} 