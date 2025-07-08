pub mod beatmap;
pub mod common;
pub mod resultscreen;

use eyre::{Result};
use std::time::Duration;
use crate::reader::common::stable::get_game_state;
use crate::reader::common::GameState;
use rosu_mem::process::{Process, ProcessTraits};
use crate::reader::structs::State;
use crate::reader::structs::StaticAddresses;
use rosu_mem::error::ProcessError;

pub mod structs;

static EXCLUDE_WORDS: [&str; 2] = ["umu-run", "waitforexitandrun"];

#[allow(dead_code)]
// Use this function to make callback and get anything you need such as map info or user info or even submit shit
pub fn waiting_for_play<F>(p: &Process, state: &mut State, callback: Option<F>) -> eyre::Result<()> 
where 
    F: Fn(&Process, &mut State) -> eyre::Result<()>
{
    loop {
        if get_game_state(p, state)? == GameState::Playing {
            return Ok(());
        }
        if let Some(f) = &callback {
            f(p, state)?;
        }
    }
}

#[allow(dead_code)]
pub fn init_loop(sleep_duration: u64) -> eyre::Result<(State, Process)> {
    let mut state = State {
        addresses: StaticAddresses::default(),
    };
    
    loop {
        match Process::initialize("osu!.exe", &EXCLUDE_WORDS) {
            Ok(p) => {
                println!("Found process, pid - {}", p.pid);
                
                println!("Reading static signatures...");
                match StaticAddresses::new(&p) {
                    Ok(v) => {
                        state.addresses = v;
                        return Ok((state, p));
                    }
                    Err(e) => {
                        if let Some(pe) = e.downcast_ref::<ProcessError>() {
                            match pe {
                                &ProcessError::ProcessNotFound => {
                                    std::thread::sleep(Duration::from_millis(sleep_duration));
                                    continue;
                                }
                                #[cfg(target_os = "windows")]
                                &ProcessError::OsError { .. } => {
                                    std::thread::sleep(Duration::from_millis(sleep_duration));
                                    continue;
                                }
                                _ => {
                                    std::thread::sleep(Duration::from_millis(sleep_duration));
                                    continue;
                                }
                            }
                        }
                        std::thread::sleep(Duration::from_millis(sleep_duration));
                    }
                }
            }
            Err(_) => {
                std::thread::sleep(Duration::from_millis(sleep_duration));
            }
        }
    }
}


// Exemple of playing loop
// pub(crate) fn playing(p: &Process, state: &mut State) -> bool {
//     let mode_list = get_mods(p, state);
//     while (GameState::from(get_status(p, state)) == GameState::Playing) {
//         cur_time = reader_gameplay::get_ig_time(p, state);
//         if (cur_time - last_time < 20 && cur_time > 0 && last_time > 0 && last_paused != cur_time) {
//             last_paused = cur_time;
//         }
//         last_time = cur_time;
//         let status = get_status(p, state);
//         let md5 = get_beatmap_md5(p, state);
//         if (last_retries < get_retries(p, state)) {
//             return false;
//         }
//     }
//     true
// }
