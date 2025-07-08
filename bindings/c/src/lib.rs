use libc::{c_char, c_float, c_int, c_uint};
use std::ffi::{CStr, CString};
use rosu_memory_lib::reader::{
    beatmap::{
        common::{BeatmapInfo, BeatmapStats, BeatmapStatus},
        stable::memory,
    },
    structs::State,
};
use rosu_memory_lib::common::GameMode;

#[repr(C)]
pub struct RosuBeatmapStats {
    ar: c_float,
    od: c_float,
    cs: c_float,
    hp: c_float,
    total_length: c_int,
    star_rating: c_float,
    object_count: c_int,
    slider_count: c_int,
}

#[no_mangle]
pub unsafe extern "C" fn rosu_get_beatmap_md5(process: *const Process, state: *mut State) -> *mut c_char {
    let result = memory::get_beatmap_md5(&*process, &mut *state);
    match result {
        Ok(md5) => CString::new(md5).unwrap().into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn rosu_get_beatmap_id(process: *const Process, state: *mut State) -> c_int {
    match memory::get_beatmap_id(&*process, &mut *state) {
        Ok(id) => id,
        Err(_) => -1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn rosu_get_beatmap_stats(
    process: *const Process,
    state: *mut State,
) -> *mut RosuBeatmapStats {
    let result = memory::get_beatmap_stats(&*process, &mut *state);
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