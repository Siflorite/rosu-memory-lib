use libc::{c_char, c_float, c_int};
use crate::reader::common::{RosuGameMode, RosuBeatmapStatus};

#[repr(C)]
pub struct RosuBeatmapStats {
    pub ar: c_float,
    pub od: c_float,
    pub cs: c_float,
    pub hp: c_float,
    pub total_length: c_int,
    pub star_rating: c_float,
    pub object_count: c_int,
    pub slider_count: c_int,
}

#[repr(C)]
pub struct RosuBeatmapLocation {
    pub folder: *mut c_char,
    pub filename: *mut c_char,
    pub audio: *mut c_char,
    pub cover: *mut c_char,
}

#[repr(C)]
pub struct RosuBeatmapMetadata {
    pub author: *mut c_char,
    pub creator: *mut c_char,
    pub title_romanized: *mut c_char,
    pub title_original: *mut c_char,
    pub difficulty: *mut c_char,
    pub tags: *mut c_char,
}

#[repr(C)]
pub struct RosuBeatmapTechnicalInfo {
    pub md5: *mut c_char,
    pub id: c_int,
    pub set_id: c_int,
    pub mode: RosuGameMode,
    pub ranked_status: RosuBeatmapStatus,
}

#[repr(C)]
pub struct RosuBeatmapInfo {
    pub technical: RosuBeatmapTechnicalInfo,
    pub metadata: RosuBeatmapMetadata,
    pub stats: RosuBeatmapStats,
    pub location: RosuBeatmapLocation,
}

#[repr(C)]
pub struct RosuBeatmapStarRating {
    pub no_mod: c_float,
    pub dt: c_float,
    pub ht: c_float,
} 