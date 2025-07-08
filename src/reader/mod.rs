mod beatmap;
mod common;
mod resultscreen;

use eyre::{Result};
use std::time::Duration;
use crate::reader::common::stable::get_game_state;
use crate::reader::common::GameState;
use rosu_mem::process::{Process, ProcessTraits};
use crate::reader::structs::State;
use crate::reader::structs::StaticAddresses;
use rosu_mem::process::ProcessError;

mod structs;

static EXCLUDE_WORDS: [&str; 2] = ["umu-run", "waitforexitandrun"];


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

pub fn init_loop(sleep_duration: u64) -> eyre::Result<(State, Process)> {
    let mut state = State {
        addresses: StaticAddresses::default(),
        };
    let p = match Process::initialize("osu!.exe", &EXCLUDE_WORDS) {
        Ok(p) => {
            println!("Found process, pid - {}", p.pid);
            p
        }
        Err(e) => {
            std::thread::sleep(Duration::from_millis(sleep_duration));
            return init_loop(sleep_duration);
        }
    };

    println!("Reading static signatures...");
    match StaticAddresses::new(&p) {
        Ok(v) => state.addresses = v,
        Err(e) => match e.downcast_ref::<ProcessError>() {
            Some(&ProcessError::ProcessNotFound) => {
                std::thread::sleep(Duration::from_millis(sleep_duration));
                return init_loop(sleep_duration);
            }
            #[cfg(target_os = "windows")]
            Some(&ProcessError::OsError { .. }) => {
                std::thread::sleep(Duration::from_millis(sleep_duration));
                return init_loop(sleep_duration);
            }
            Some(_) | None => {
                std::thread::sleep(Duration::from_millis(sleep_duration));
                return init_loop(sleep_duration);
            }
        },
    };

    Ok((state, p))
}

pub fn controlla() -> Result<()> {

    let (state, p) = init_loop(300)?;


        println!("Starting reading loop");
        'main_loop: loop {
            std::thread::sleep(Duration::from_millis(1000));
            println!("Waiting For Play");
            if let Err(e) = waiting_for_play(&p, &mut state, None) {
                match e.downcast_ref::<ProcessError>() {
                    Some(&ProcessError::ProcessNotFound) => {
                        continue 'main_loop;
                    }
                    #[cfg(target_os = "windows")]
                    Some(&ProcessError::OsError { .. }) => {
                        println!("{:?}", e);
                        continue 'main_loop;
                    }
                    Some(_) | None => {
                        println!("{:?}", e);
                        continue 'main_loop;
                    }
                }
            }
            println!("Playing");
            std::thread::sleep(Duration::from_millis(700));
            let a = playing(&p, &mut state);

            // let a = wait_result_screen(&p, &mut state) && a;
    
    }
}

pub(crate) fn playing(p: &Process, state: &mut State) -> bool {
    println!("Playing");
    let mut cur_time = 0;
    let mut last_time = 0;
    let mut last_paused = 0;
    let mut last_retries = 0;
    let mode_list = get_mods(p, state);
    while (GameState::from(get_status(p, state)) == GameState::Playing) {
        cur_time = reader_gameplay::get_ig_time(p, state);
        if (cur_time - last_time < 20 && cur_time > 0 && last_time > 0 && last_paused != cur_time) {
            last_paused = cur_time;
        }
        last_time = cur_time;
        let status = get_status(p, state);
        let md5 = get_beatmap_md5(p, state);
        if (last_retries < get_retries(p, state)) {
            return false;
        }
    }
    true
}
