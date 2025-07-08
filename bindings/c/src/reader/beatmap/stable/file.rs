use libc::{c_char, c_float, c_int};
use std::ffi::{CStr, CString};
use rosu_memory_lib::reader::beatmap::stable::file as rust_file;
use crate::reader::beatmap::common::{
    RosuBeatmapInfo, RosuBeatmapLocation, RosuBeatmapMetadata,
    RosuBeatmapStats, RosuBeatmapTechnicalInfo,
};
use crate::reader::common::{RosuGameMode, RosuBeatmapStatus};

#[no_mangle]
pub unsafe extern "C" fn rosu_file_get_beatmap_info(path: *const c_char) -> *mut RosuBeatmapInfo {
    if path.is_null() {
        return std::ptr::null_mut();
    }

    let c_str = CStr::from_ptr(path);
    let path_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let result = rust_file::get_beatmap_info(path_str);
    match result {
        Ok(info) => {
            let technical = RosuBeatmapTechnicalInfo {
                md5: CString::new(info.technical.md5).unwrap().into_raw(),
                id: info.technical.id,
                set_id: info.technical.set_id,
                mode: RosuGameMode::from(info.technical.mode),
                ranked_status: match info.technical.ranked_status {
                    BeatmapStatus::Unknown => RosuBeatmapStatus::Unknown,
                    BeatmapStatus::NotSubmitted => RosuBeatmapStatus::NotSubmitted,
                    BeatmapStatus::Pending => RosuBeatmapStatus::Pending,
                    BeatmapStatus::UpdateAvailable => RosuBeatmapStatus::UpdateAvailable,
                    BeatmapStatus::Ranked => RosuBeatmapStatus::Ranked,
                    BeatmapStatus::Approved => RosuBeatmapStatus::Approved,
                    BeatmapStatus::Qualified => RosuBeatmapStatus::Qualified,
                    BeatmapStatus::Loved => RosuBeatmapStatus::Loved,
                },
            };

            let metadata = RosuBeatmapMetadata {
                author: CString::new(info.metadata.author).unwrap().into_raw(),
                creator: CString::new(info.metadata.creator).unwrap().into_raw(),
                title_romanized: CString::new(info.metadata.title_romanized).unwrap().into_raw(),
                title_original: CString::new(info.metadata.title_original).unwrap().into_raw(),
                difficulty: CString::new(info.metadata.difficulty).unwrap().into_raw(),
                tags: CString::new(info.metadata.tags).unwrap().into_raw(),
            };

            let stats = RosuBeatmapStats {
                ar: info.stats.ar,
                od: info.stats.od,
                cs: info.stats.cs,
                hp: info.stats.hp,
                total_length: info.stats.total_length,
                star_rating: info.stats.star_rating,
                object_count: info.stats.object_count,
                slider_count: info.stats.slider_count,
            };

            let location = RosuBeatmapLocation {
                folder: CString::new(info.location.folder).unwrap().into_raw(),
                filename: CString::new(info.location.filename).unwrap().into_raw(),
                audio: CString::new(info.location.audio).unwrap().into_raw(),
                cover: CString::new(info.location.cover).unwrap().into_raw(),
            };

            let c_info = Box::new(RosuBeatmapInfo {
                technical,
                metadata,
                stats,
                location,
            });
            Box::into_raw(c_info)
        }
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn rosu_free_beatmap_info(ptr: *mut RosuBeatmapInfo) {
    if !ptr.is_null() {
        let info = Box::from_raw(ptr);
        
        // Free all strings
        rosu_free_string(info.technical.md5);
        rosu_free_string(info.metadata.author);
        rosu_free_string(info.metadata.creator);
        rosu_free_string(info.metadata.title_romanized);
        rosu_free_string(info.metadata.title_original);
        rosu_free_string(info.metadata.difficulty);
        rosu_free_string(info.metadata.tags);
        rosu_free_string(info.location.folder);
        rosu_free_string(info.location.filename);
        rosu_free_string(info.location.audio);
        rosu_free_string(info.location.cover);
        
        // The Box will be dropped here
    }
} 