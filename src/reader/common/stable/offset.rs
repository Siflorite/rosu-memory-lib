use rosu_mem::process::{Process, ProcessTraits};
use crate::reader::structs::State;

pub struct CommonOffset {
    pub settings_ptr: i32,
    pub settings_addr: i32,
    pub path: i32,
    pub status: i32,
}


pub(crate) static COMMON_OFFSET: CommonOffset = CommonOffset {
    settings_ptr: 0x8,
    settings_addr: 0xb8,
    path: 0x4,
    status: 0x4,
};

pub(crate) fn get_path_folder(p: &Process, state: &mut State) -> eyre::Result<String> {

    let settings_ptr = p.read_i32(state.addresses.settings+COMMON_OFFSET.settings_ptr)?;
    let settings_addr = p.read_i32(settings_ptr+COMMON_OFFSET.settings_addr)?;
    let path = p.read_string(settings_addr+COMMON_OFFSET.path)?;
    // default (relative path)
    if path == "Songs" {
        return Ok(format!("{}/Songs", p.executable_dir.clone().unwrap().display()));
    }
    // custom user path (absolute path)
    Ok(path)
}