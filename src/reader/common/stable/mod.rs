pub mod offset;
use rosu_mem::process::{Process, ProcessTraits};
use crate::reader::structs::State;
use crate::reader::common::stable::offset::COMMON_OFFSET;
use crate::reader::common::GameState;

pub fn get_game_state(p: &Process, state: &mut State) -> eyre::Result<GameState> {
    let status_ptr = p.read_i32(state.addresses.status - COMMON_OFFSET.status)?;
    Ok(GameState::from(p.read_u32(status_ptr)?))
}
