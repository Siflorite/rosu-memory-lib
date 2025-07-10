pub mod memory;
pub mod offset;
pub mod location;
pub mod file;

use rosu_mem::process::{Process, ProcessTraits};
use crate::reader::structs::State;
use crate::reader::beatmap::stable::offset::*;
use crate::reader::common::GameState;
use crate::reader::common::Error;
use crate::reader::common::stable::check_game_state;

pub(crate) fn get_beatmap_addr(p: &Process, state: &mut State) -> eyre::Result<i32>
{
    match check_game_state(p, state, GameState::SongSelect)? || check_game_state(p, state, GameState::Editor)? 
    || check_game_state(p, state, GameState::Playing)? || check_game_state(p, state, GameState::ResultScreen)? {
        true => Ok(p.read_i32(p.read_i32(state.addresses.base - BEATMAP_OFFSET.ptr)?)?),
        false => Err(eyre::eyre!(Error::NotAvailable("Not in song select".to_string()))),
    }
}

pub(crate) fn read_from_beatmap_ptr_string(p: &Process, state: &mut State, offset: i32) -> eyre::Result<String>
{
    let beatmap_addr = get_beatmap_addr(p, state)?;
    Ok(p.read_string(beatmap_addr + offset)?)
}


