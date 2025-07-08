use pyo3::prelude::*;
use rosu_memory_lib::reader::beatmap::common::{BeatmapInfo, BeatmapLocation, BeatmapMetadata, BeatmapStats, BeatmapStatus};
use crate::reader::common::PyGameMode;

#[pyclass(name = "BeatmapStatus")]
#[derive(Clone)]
pub struct PyBeatmapStatus(pub(crate) BeatmapStatus);

#[pymethods]
impl PyBeatmapStatus {
    #[new]
    fn new(status: i32) -> Self {
        PyBeatmapStatus(BeatmapStatus::from(status))
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }
}

#[pyclass(name = "BeatmapStats")]
#[derive(Clone)]
pub struct PyBeatmapStats {
    #[pyo3(get)]
    pub ar: f32,
    #[pyo3(get)]
    pub od: f32,
    #[pyo3(get)]
    pub cs: f32,
    #[pyo3(get)]
    pub hp: f32,
    #[pyo3(get)]
    pub total_length: i32,
    #[pyo3(get)]
    pub star_rating: f32,
    #[pyo3(get)]
    pub object_count: i32,
    #[pyo3(get)]
    pub slider_count: i32,
}

#[pyclass(name = "BeatmapLocation")]
#[derive(Clone)]
pub struct PyBeatmapLocation {
    #[pyo3(get)]
    pub folder: String,
    #[pyo3(get)]
    pub filename: String,
    #[pyo3(get)]
    pub audio: String,
    #[pyo3(get)]
    pub cover: String,
}

#[pyclass(name = "BeatmapMetadata")]
#[derive(Clone)]
pub struct PyBeatmapMetadata {
    #[pyo3(get)]
    pub author: String,
    #[pyo3(get)]
    pub creator: String,
    #[pyo3(get)]
    pub title_romanized: String,
    #[pyo3(get)]
    pub title_original: String,
    #[pyo3(get)]
    pub difficulty: String,
    #[pyo3(get)]
    pub tags: String,
}

#[pyclass(name = "BeatmapTechnicalInfo")]
#[derive(Clone)]
pub struct PyBeatmapTechnicalInfo {
    #[pyo3(get)]
    pub md5: String,
    #[pyo3(get)]
    pub id: i32,
    #[pyo3(get)]
    pub set_id: i32,
    #[pyo3(get)]
    pub mode: PyGameMode,
    #[pyo3(get)]
    pub ranked_status: PyBeatmapStatus,
}

#[pyclass(name = "BeatmapInfo")]
#[derive(Clone)]
pub struct PyBeatmapInfo {
    #[pyo3(get)]
    pub technical: PyBeatmapTechnicalInfo,
    #[pyo3(get)]
    pub metadata: PyBeatmapMetadata,
    #[pyo3(get)]
    pub stats: PyBeatmapStats,
    #[pyo3(get)]
    pub location: PyBeatmapLocation,
} 