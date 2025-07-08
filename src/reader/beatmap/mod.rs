mod common;
mod stable;

use rosu_mem::process::{Process};
use crate::reader::beatmap::common::BeatmapInfo;
use crate::reader::structs::State;
use crate::reader::common::OsuType;



pub(crate) fn read_beatmap_info(p: &Process, state: &mut State, osu_type: OsuType) -> eyre::Result<BeatmapInfo>
{
    match osu_type {
        OsuType::Stable => stable::read_beatmap_info(p, state),
        _ => Err(eyre::eyre!("Unsupported osu type for now")),
    }
}
