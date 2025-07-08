
use rosu_mem::process::{Process, ProcessTraits};
use crate::reader::structs::{Hit, State};
use crate::reader::resultscreen::ResultScreenValues;
use crate::reader::common::GameMode;

pub struct ResultScreenOffset {
    pub ptr : i32,
    pub addr : i32,
    pub score_base : i32,
    pub username : i32,
    pub score : i32,
    pub max_combo : i32,
    pub mode : i32,
    pub hits_300 : i32,
    pub hits_100 : i32,
    pub hits_miss : i32,
    pub hits_50 : i32,
    pub hits_geki : i32,
    pub hits_katu : i32,
}

pub(crate) static RESULT_SCREEN_OFFSET: ResultScreenOffset = ResultScreenOffset {
    ptr: 0xb,
    addr: 0x4,
    score_base: 0x38,
    username: 0x28,
    score: 0x78,
    max_combo: 0x68,
    mode: 0x64,
    hits_300: 0x8A,
    hits_100: 0x88,
    hits_miss: 0x92,
    hits_50: 0x8C,
    hits_geki: 0x8E,
    hits_katu: 0x90,
};

pub struct ResultScreenHitsOffset{
    pub _300: i32,
    pub _100: i32,
    pub _50: i32,
    pub _miss: i32,
    pub _geki: i32,
    pub _katu: i32,
}



pub(crate) fn get_score_base(p: &Process, state: &mut State) -> eyre::Result<i32> {
    let ruleset_addr = p.read_i32(state.addresses.rulesets - RESULT_SCREEN_OFFSET.ptr).unwrap();
    let ruleset_addr = p.read_i32(ruleset_addr + RESULT_SCREEN_OFFSET.addr).unwrap();
    Ok(p.read_i32(ruleset_addr + RESULT_SCREEN_OFFSET.score_base)?)
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
    Ok(p.read_i16(score_base + RESULT_SCREEN_OFFSET.hits_300)?)
}
pub fn get_result_hit_100(p: &Process, state: &mut State) -> eyre::Result<i16> {
    let score_base = get_score_base(p, state)?;
    Ok(p.read_i16(score_base + RESULT_SCREEN_OFFSET.hits_100)?)
}
pub fn get_result_hit_50(p: &Process, state: &mut State) -> eyre::Result<i16> {
    let score_base = get_score_base(p, state)?;
    Ok(p.read_i16(score_base + RESULT_SCREEN_OFFSET.hits_50)?)
}
pub fn get_result_hit_geki(p: &Process, state: &mut State) -> eyre::Result<i16> {
    let score_base = get_score_base(p, state)?;
    Ok(p.read_i16(score_base + RESULT_SCREEN_OFFSET.hits_geki)?)
}
pub fn get_result_hit_katu(p: &Process, state: &mut State) -> eyre::Result<i16> {
    let score_base = get_score_base(p, state)?;
    Ok(p.read_i16(score_base + RESULT_SCREEN_OFFSET.hits_katu)?)
}
pub fn get_result_hit_miss(p: &Process, state: &mut State) -> eyre::Result<i16> {
    let score_base = get_score_base(p, state)?;
    Ok(p.read_i16(score_base + RESULT_SCREEN_OFFSET.hits_miss)?)
}


pub fn get_result_hits(p: &Process, state: &mut State) -> eyre::Result<Hit> {
    let score_base = get_score_base(p, state)?;
    Ok(Hit {
        _300: p.read_i16(score_base + RESULT_SCREEN_OFFSET.hits_300).unwrap(),
        _100: p.read_i16(score_base + RESULT_SCREEN_OFFSET.hits_100).unwrap(),
        _50: p.read_i16(score_base + RESULT_SCREEN_OFFSET.hits_50).unwrap(),
        _miss: p.read_i16(score_base + RESULT_SCREEN_OFFSET.hits_miss).unwrap(),
        _geki: p.read_i16(score_base + RESULT_SCREEN_OFFSET.hits_geki).unwrap(),
        _katu: p.read_i16(score_base + RESULT_SCREEN_OFFSET.hits_katu).unwrap(),
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

    Ok(numerator / denominator)
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

pub fn get_result_screen(p: &Process, state: &mut State) -> eyre::Result<ResultScreenValues> {
    let score_base = get_score_base(p, state)?;

    let hit = get_result_hits(p, state)?;
    let mode = get_result_mode(p, state)?;
    let accuracy = calculate_accuracy(&mode, &hit)?;
    Ok(ResultScreenValues{
            username: p.read_string(score_base + RESULT_SCREEN_OFFSET.username)?,
            mode :  mode,
            max_combo : p.read_i16(score_base + RESULT_SCREEN_OFFSET.max_combo)?,
            score : p.read_i32(score_base + RESULT_SCREEN_OFFSET.score)?,
            hit : hit,
            accuracy : accuracy,
    })
}
