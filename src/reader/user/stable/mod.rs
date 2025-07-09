use rosu_mem::process::{Process, ProcessTraits};
use crate::reader::structs::State;
use std::fs::OpenOptions;
use std::io::Write;


pub fn get_user_profile_base(p: &Process, state: &mut State) -> eyre::Result<i32> 
{    
    Ok(p.read_i32(p.read_i32(state.addresses.user_profile + 0x7)?)?)
}

pub fn get_user_bancho_status(p: &Process, state: &mut State) -> eyre::Result<i32> {
    let user_profile_base = get_user_profile_base(p, state)?;
    Ok(p.read_i32(user_profile_base +  0x8c)?)
}

pub fn get_user_username(p: &Process, state: &mut State) -> eyre::Result<String> {
    let user_profile_base = get_user_profile_base(p, state)?;
    let username_ptr = p.read_i32(user_profile_base + 0x30)?;
    Ok(p.read_string(username_ptr)?)
}

pub fn get_user_pp(p: &Process, state: &mut State) -> eyre::Result<i32> {
    let user_profile_base = get_user_profile_base(p, state)?;
    Ok(p.read_i32(user_profile_base + 0x88)?)
}

pub fn get_user_rankedscore(p: &Process, state: &mut State) -> eyre::Result<i64> {
    let user_profile_base = get_user_profile_base(p, state)?;
    Ok(p.read_i64(user_profile_base + 0xC)?)
}

pub fn get_user_level(p: &Process, state: &mut State) -> eyre::Result<f32> {
    let user_profile_base = get_user_profile_base(p, state)?;
    Ok(p.read_f32(user_profile_base + 0x74)?)
}

pub fn get_user_playcount(p: &Process, state: &mut State) -> eyre::Result<i32> {
    let user_profile_base = get_user_profile_base(p, state)?;
    Ok(p.read_i32(user_profile_base + 0x7C)?)
}

pub fn get_user_rank(p: &Process, state: &mut State) -> eyre::Result<i32> {
    let user_profile_base = get_user_profile_base(p, state)?;
    Ok(p.read_i32(user_profile_base + 0x84)?)
}

pub fn get_user_playmode(p: &Process, state: &mut State) -> eyre::Result<i32> {
    let user_profile_base = get_user_profile_base(p, state)?;
    Ok(p.read_i32(user_profile_base + 0x80)?)
}

pub fn get_user_accuracy(p: &Process, state: &mut State) -> eyre::Result<f64> {
    let user_profile_base = get_user_profile_base(p, state)?;
    Ok(p.read_f64(user_profile_base + 0x4)?)
}

#[derive(Debug, Clone)]
pub struct UserProfile {
    id: i32,
    username: String,
    pp: i32,
    rankedscore: i64,
    level: f32,
    playcount: i32,
    rank: i32,
    playmode: i32,
    accuracy: f64,
    country_code: i32,
    bancho_status: i32,
}

pub fn get_user_profile(p: &Process, state: &mut State) -> eyre::Result<UserProfile> {
    let user_profile_base = get_user_profile_base(p, state)?;
    let user_profile = UserProfile {
        id: p.read_i32(user_profile_base + 0x70)?,
        username: p.read_string(p.read_i32(user_profile_base + 0x30)?)?, // need a fix idk how
        pp: p.read_i32(user_profile_base + 0x88)?,
        rankedscore: p.read_i64(user_profile_base + 0xC)?,
        level: p.read_f32(user_profile_base + 0x74)?,
        playcount: p.read_i32(user_profile_base + 0x7C)?,
        rank: p.read_i32(user_profile_base + 0x84)?,
        playmode: p.read_i32(user_profile_base + 0x80)?,
        accuracy: p.read_f64(user_profile_base + 0x4)?,
        country_code: p.read_i32(user_profile_base + 0x9c)?,
        bancho_status: p.read_i32(user_profile_base + 0x8c)?,
    };
    Ok(user_profile)
}


/*
pub fn get_sessions_plays(p: &Process, state: &mut State) -> eyre::Result<i32>
{
    println!("Base: {:#x}", state.addresses.base);
    let beatmap_ptr = p.read_i32(state.addresses.base - 0x33)?;
    println!("Beatmap ptr: {:#x}", beatmap_ptr);
    Ok(p.read_i32(beatmap_ptr + 0xC)?)
}

pub fn get_current_game_mode(p: &Process, state: &mut State) -> eyre::Result<GameMode>
{
    Ok(GameMode::from(p.read_i32(p.read_i32(state.addresses.base - 0x33)?)?))
}

 */