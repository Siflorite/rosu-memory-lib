
use std::collections::HashMap;
use rosu_mem::process::{Process, ProcessTraits};
use crate::reader::structs::{Hit, ResultScreenValues, State};


pub struct ResultScreenOffset {
    pub ptr : i32,
}

pub(crate) static RESULT_SCREEN_OFFSET: ResultScreenOffset = ResultScreenOffset {
    ptr: 0xb,
    addr: 0x4,
    score_base: 0x38,
    username: 0x28,
    score: 0x78,
    max_combo: 0x68,
    mode: 0x64,
    hits: 0x8A,
    hits_300: 0x8A,
    hits_100: 0x88,
    hits_miss: 0x92,
    hits_50: 0x8C,
    hits_geki: 0x8E,
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

pub fn get_result_username(p: &Process, state: &mut State) -> String {
    let score_base = get_score_base(p, state)?;
    return p.read_string(score_base + RESULT_SCREEN_OFFSET.username).unwrap();
}
pub fn get_result_score(p: &Process, state: &mut State) -> i32 {
    let score_base = get_score_base(p, state)?;
    return p.read_i32(score_base + RESULT_SCREEN_OFFSET.score).unwrap();
}
pub fn get_result_mode(p: &Process, state: &mut State) -> GameMode {
    let score_base = get_score_base(p, state)?;
    return GameMode::from(p.read_i32(score_base + RESULT_SCREEN_OFFSET.mode).unwrap());
}

pub fn get_result_hit_300(p: &Process, state: &mut State) -> i16 {
    let ruleset_addr = p.read_i32(state.addresses.rulesets - 0xb).unwrap();
    let ruleset_addr = p.read_i32(ruleset_addr + 0x4).unwrap();
    let result_base = p.read_i32(ruleset_addr + 0x38).unwrap();
    return p.read_i16(result_base + 0x8A).unwrap();
}
pub fn get_result_hit_100(p: &Process, state: &mut State) -> i16 {
    let ruleset_addr = p.read_i32(state.addresses.rulesets - 0xb).unwrap();
    let ruleset_addr = p.read_i32(ruleset_addr + 0x4).unwrap();
    let result_base = p.read_i32(ruleset_addr + 0x38).unwrap();
    return p.read_i16(result_base + 0x88).unwrap();
}
pub fn get_result_hit_50(p: &Process, state: &mut State) -> i16 {
    let ruleset_addr = p.read_i32(state.addresses.rulesets - 0xb).unwrap();
    let ruleset_addr = p.read_i32(ruleset_addr + 0x4).unwrap();
    let result_base = p.read_i32(ruleset_addr + 0x38).unwrap();
    return p.read_i16(result_base + 0x8C).unwrap();
}
pub fn get_result_hit_geki(p: &Process, state: &mut State) -> i16 {
    let ruleset_addr = p.read_i32(state.addresses.rulesets - 0xb).unwrap();
    let ruleset_addr = p.read_i32(ruleset_addr + 0x4).unwrap();
    let result_base = p.read_i32(ruleset_addr + 0x38).unwrap();
    return p.read_i16(result_base + 0x8E).unwrap();
}
pub fn get_result_hit_katu(p: &Process, state: &mut State) -> i16 {
    let ruleset_addr = p.read_i32(state.addresses.rulesets - 0xb).unwrap();
    let ruleset_addr = p.read_i32(ruleset_addr + 0x4).unwrap();
    let result_base = p.read_i32(ruleset_addr + 0x38).unwrap();
    return p.read_i16(result_base + 0x90).unwrap();
}
pub fn get_result_hit_miss(p: &Process, state: &mut State) -> i16 {
    let ruleset_addr = p.read_i32(state.addresses.rulesets - 0xb).unwrap();
    let ruleset_addr = p.read_i32(ruleset_addr + 0x4).unwrap();
    let result_base = p.read_i32(ruleset_addr + 0x38).unwrap();
    return p.read_i16(result_base + 0x92).unwrap();
}


pub fn get_result_hits(p: &Process, state: &mut State) -> Hit {
    let ruleset_addr = p.read_i32(state.addresses.rulesets - 0xb).unwrap();
    let ruleset_addr = p.read_i32(ruleset_addr + 0x4).unwrap();
    let base = p.read_i32(ruleset_addr + 0x38).unwrap();
    Hit {
        _300: p.read_i16(base + 0x8A).unwrap(),
        _100: p.read_i16(base + 0x88).unwrap(),
        _50: p.read_i16(base + 0x8C).unwrap(),
        _miss: p.read_i16(base + 0x92).unwrap(),
        _geki: p.read_i16(base + 0x8E).unwrap(),
        _katu: p.read_i16(base + 0x90).unwrap(),
    }
}
fn calculate_accuracy(
    gamemode: u8,
    hit: Hit
) -> f64 {
    let (numerator, denominator) = match gamemode {
        0 => (
                hit._300 as f64 * 6.0 + hit._100 as f64 * 2.0 + hit._50 as f64,
                (hit._300 + hit._100 + hit._50 + hit._miss) as f64 * 6.0
        ),
        1 => (
                hit._300 as f64 * 2.0 + hit._100 as f64,
                (hit._300 + hit._100 + hit._50 + hit._miss) as f64 * 2.0
        ),
        2 => (
                (hit._300 + hit._100 + hit._50) as f64,
                (hit._300 + hit._100 + hit._50 + hit._katu + hit._miss) as f64
        ),
        3 => (
                (hit._geki + hit._300) as f64 * 6.0 + hit._katu as f64 * 4.0 + hit._100 as f64 * 2.0 + hit._50 as f64,
                (hit._geki + hit._300 + hit._katu + hit._100 + hit._50 + hit._miss) as f64 * 6.0
        ),
        _ => panic!("Unsupported Gamemode : {}", gamemode)
    };

    numerator / denominator
}

pub fn get_result_accuracy(p: &Process, state: &mut State) -> f64 {
    calculate_accuracy(
        get_result_mode(p, state),
        get_result_hits(p,state),
    )
}
pub fn get_result_max_combo(p: &Process, state: &mut State) -> i16 {
    let ruleset_addr = p.read_i32(state.addresses.rulesets - 0xb).unwrap();
    let ruleset_addr = p.read_i32(ruleset_addr + 0x4).unwrap();
    let score_base = p.read_i32(ruleset_addr + 0x38).unwrap();
    p.read_i16(score_base + 0x68).unwrap()
}

pub fn get_result_screen(p: &Process, state: &mut State) -> ResultScreenValues {
    ResultScreenValues{
            username: get_result_username(p,state),
            mode :  get_result_mode(p,state),
            max_combo : get_result_max_combo(p,state),
            score : get_result_score(p,state),
            hit : get_result_hits(p,state),
            accuracy : get_result_accuracy(p,state),
    }
}
