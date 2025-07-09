
use crate::reader::common::GameMode;
use rosu_mem::process::{Process, ProcessTraits};
use crate::reader::structs::{Hit, State};
use crate::reader::resultscreen::common::ResultScreenInfo;
use crate::reader::resultscreen::stable::offset::RESULT_SCREEN_OFFSET;
use crate::reader::common::GameState;
use crate::reader::common::stable::check_game_state;
use crate::reader::common::Error;

pub(crate) fn get_score_base(p: &Process, state: &mut State) -> eyre::Result<i32> {
    if check_game_state(p, state, GameState::ResultScreen)? {
        let ruleset_addr = match p.read_i32(state.addresses.rulesets - RESULT_SCREEN_OFFSET.ptr) {
            Ok(val) => val,
            Err(_) => return Err(eyre::eyre!(Error::NotAvailable("Still loading".to_string())))
        };
        let ruleset_addr = match p.read_i32(ruleset_addr + RESULT_SCREEN_OFFSET.addr) {
            Ok(val) => val,
            Err(_) => return Err(eyre::eyre!(Error::NotAvailable("Still loading".to_string())))
        };
        Ok(p.read_i32(ruleset_addr + RESULT_SCREEN_OFFSET.score_base)?)
    } else {
        Err(eyre::eyre!(Error::NotAvailable("Not in ResultScreen".to_string())))
    }
}

pub fn get_result_username(p: &Process, state: &mut State) -> eyre::Result<String> {
    let score_base = get_score_base(p, state)?;
    Ok(p.read_string(score_base + RESULT_SCREEN_OFFSET.username)?)
}
pub fn get_result_score(p: &Process, state: &mut State) -> eyre::Result<i32> {
    let score_base = get_score_base(p, state)?;
    Ok(p.read_i32(score_base + RESULT_SCREEN_OFFSET.score)?)
}
pub fn get_result_mode(p: &Process, state: &mut State) -> eyre::Result<GameMode> {
    let score_base = get_score_base(p, state)?;
    Ok(GameMode::from(p.read_i32(score_base + RESULT_SCREEN_OFFSET.mode)?))
}

pub fn get_result_hit_300(p: &Process, state: &mut State) -> eyre::Result<i16> {
    let score_base = get_score_base(p, state)?;
    Ok(p.read_i16(score_base + RESULT_SCREEN_OFFSET.hits._300)?)
}
pub fn get_result_hit_100(p: &Process, state: &mut State) -> eyre::Result<i16> {
    let score_base = get_score_base(p, state)?;
    Ok(p.read_i16(score_base + RESULT_SCREEN_OFFSET.hits._100)?)
}
pub fn get_result_hit_50(p: &Process, state: &mut State) -> eyre::Result<i16> {
    let score_base = get_score_base(p, state)?;
    Ok(p.read_i16(score_base + RESULT_SCREEN_OFFSET.hits._50)?)
}
pub fn get_result_hit_geki(p: &Process, state: &mut State) -> eyre::Result<i16> {
    let score_base = get_score_base(p, state)?;
    Ok(p.read_i16(score_base + RESULT_SCREEN_OFFSET.hits._geki)?)
}
pub fn get_result_hit_katu(p: &Process, state: &mut State) -> eyre::Result<i16> {
    let score_base = get_score_base(p, state)?;
    Ok(p.read_i16(score_base + RESULT_SCREEN_OFFSET.hits._katu)?)
}
pub fn get_result_hit_miss(p: &Process, state: &mut State) -> eyre::Result<i16> {
    let score_base = get_score_base(p, state)?;
    Ok(p.read_i16(score_base + RESULT_SCREEN_OFFSET.hits._miss)?)
}


pub fn get_result_hits(p: &Process, state: &mut State) -> eyre::Result<Hit> {
    let score_base = get_score_base(p, state)?;
    Ok(Hit {
        _300: p.read_i16(score_base + RESULT_SCREEN_OFFSET.hits._300)?,
        _100: p.read_i16(score_base + RESULT_SCREEN_OFFSET.hits._100)?,
        _50: p.read_i16(score_base + RESULT_SCREEN_OFFSET.hits._50)?,
        _miss: p.read_i16(score_base + RESULT_SCREEN_OFFSET.hits._miss)?,
        _geki: p.read_i16(score_base + RESULT_SCREEN_OFFSET.hits._geki)?,
        _katu: p.read_i16(score_base + RESULT_SCREEN_OFFSET.hits._katu)?,
    })
}
fn calculate_accuracy(
    gamemode: &GameMode,
    hit: &Hit
) -> eyre::Result<f64> {
    let (numerator, denominator) = match gamemode {
        GameMode::Osu => (
                hit._300 as f64 * 6.0 + hit._100 as f64 * 2.0 + hit._50 as f64,
                (hit._300 + hit._100 + hit._50 + hit._miss) as f64 * 6.0
        ),
        GameMode::Taiko => (
                hit._300 as f64 * 2.0 + hit._100 as f64,
                (hit._300 + hit._100 + hit._50 + hit._miss) as f64 * 2.0
        ),
        GameMode::Catch => (
                (hit._300 + hit._100 + hit._50) as f64,
                (hit._300 + hit._100 + hit._50 + hit._katu + hit._miss) as f64
        ),
        GameMode::Mania => (
                (hit._geki + hit._300) as f64 * 6.0 + hit._katu as f64 * 4.0 + hit._100 as f64 * 2.0 + hit._50 as f64,
                (hit._geki + hit._300 + hit._katu + hit._100 + hit._50 + hit._miss) as f64 * 6.0
        ),
        _ => (0.0, 0.0)
    };

    Ok((numerator / denominator) * 100.0)
}

pub fn get_result_accuracy(p: &Process, state: &mut State) -> eyre::Result<f64> {
    calculate_accuracy(
        &get_result_mode(p, state)?,
        &get_result_hits(p,state)?,
    )
}
pub fn get_result_max_combo(p: &Process, state: &mut State) -> eyre::Result<i16> {
    let score_base = get_score_base(p, state)?;
    Ok(p.read_i16(score_base + RESULT_SCREEN_OFFSET.max_combo)?)
}

pub fn get_result_screen(p: &Process, state: &mut State) -> eyre::Result<ResultScreenInfo> {
    let score_base = get_score_base(p, state)?;

    let hit = get_result_hits(p, state)?;
    let mode = get_result_mode(p, state)?;
    let accuracy = calculate_accuracy(&mode, &hit)?;
    Ok(ResultScreenInfo{
            username: p.read_string(score_base + RESULT_SCREEN_OFFSET.username)?,
            mode :  mode,
            max_combo : p.read_i16(score_base + RESULT_SCREEN_OFFSET.max_combo)?,
            score : p.read_i32(score_base + RESULT_SCREEN_OFFSET.score)?,
            hit : hit,
            accuracy : accuracy,
    })
}
