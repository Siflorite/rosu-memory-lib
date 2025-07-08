use libc::{c_char, c_float, c_int, c_uint};
use std::ffi::CString;
use rosu_memory_lib::reader::{
    beatmap::stable::memory as rust_memory,
    structs::State,
};
use crate::reader::beatmap::common::{
    RosuBeatmapInfo, RosuBeatmapLocation, RosuBeatmapMetadata,
    RosuBeatmapStats, RosuBeatmapTechnicalInfo,
};
use crate::reader::common::{RosuGameMode, RosuBeatmapStatus};

#[no_mangle]
pub unsafe extern "C" fn rosu_memory_get_beatmap_md5(process: *const Process, state: *mut State) -> *mut c_char {
    let result = rust_memory::get_beatmap_md5(&*process, &mut *state);
    match result {
        Ok(md5) => CString::new(md5).unwrap().into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn rosu_memory_get_beatmap_id(process: *const Process, state: *mut State) -> c_int {
    match rust_memory::get_beatmap_id(&*process, &mut *state) {
        Ok(id) => id,
        Err(_) => -1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn rosu_memory_get_beatmap_set_id(process: *const Process, state: *mut State) -> c_int {
    match rust_memory::get_beatmap_set_id(&*process, &mut *state) {
        Ok(id) => id,
        Err(_) => -1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn rosu_memory_get_beatmap_mode(process: *const Process, state: *mut State) -> RosuGameMode {
    match rust_memory::get_beatmap_mode(&*process, &mut *state) {
        Ok(mode) => RosuGameMode::from(mode),
        Err(_) => RosuGameMode::Osu,
    }
}

#[no_mangle]
pub unsafe extern "C" fn rosu_memory_get_beatmap_tags(process: *const Process, state: *mut State) -> *mut c_char {
    let result = rust_memory::get_beatmap_tags(&*process, &mut *state);
    match result {
        Ok(tags) => CString::new(tags).unwrap().into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn rosu_memory_get_beatmap_length(process: *const Process, state: *mut State) -> c_int {
    match rust_memory::get_beatmap_length(&*process, &mut *state) {
        Ok(length) => length,
        Err(_) => -1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn rosu_memory_get_beatmap_drain_time(process: *const Process, state: *mut State) -> c_int {
    match rust_memory::get_beatmap_drain_time(&*process, &mut *state) {
        Ok(time) => time,
        Err(_) => -1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn rosu_memory_get_beatmap_status(process: *const Process, state: *mut State) -> RosuBeatmapStatus {
    match rust_memory::get_beatmap_status(&*process, &mut *state) {
        Ok(status) => match status {
            BeatmapStatus::Unknown => RosuBeatmapStatus::Unknown,
            BeatmapStatus::NotSubmitted => RosuBeatmapStatus::NotSubmitted,
            BeatmapStatus::Pending => RosuBeatmapStatus::Pending,
            BeatmapStatus::UpdateAvailable => RosuBeatmapStatus::UpdateAvailable,
            BeatmapStatus::Ranked => RosuBeatmapStatus::Ranked,
            BeatmapStatus::Approved => RosuBeatmapStatus::Approved,
            BeatmapStatus::Qualified => RosuBeatmapStatus::Qualified,
            BeatmapStatus::Loved => RosuBeatmapStatus::Loved,
        },
        Err(_) => RosuBeatmapStatus::Unknown,
    }
}

// ... Ajoutez les autres fonctions de la même manière

#[no_mangle]
pub unsafe extern "C" fn rosu_memory_get_beatmap_stats(
    process: *const Process,
    state: *mut State,
) -> *mut RosuBeatmapStats {
    let result = rust_memory::get_beatmap_stats(&*process, &mut *state);
    match result {
        Ok(stats) => {
            let c_stats = Box::new(RosuBeatmapStats {
                ar: stats.ar,
                od: stats.od,
                cs: stats.cs,
                hp: stats.hp,
                total_length: stats.total_length,
                star_rating: stats.star_rating,
                object_count: stats.object_count,
                slider_count: stats.slider_count,
            });
            Box::into_raw(c_stats)
        }
        Err(_) => std::ptr::null_mut(),
    }
}

// Fonctions de libération de mémoire
#[no_mangle]
pub unsafe extern "C" fn rosu_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        drop(CString::from_raw(ptr));
    }
}

#[no_mangle]
pub unsafe extern "C" fn rosu_free_beatmap_stats(ptr: *mut RosuBeatmapStats) {
    if !ptr.is_null() {
        drop(Box::from_raw(ptr));
    }
} 