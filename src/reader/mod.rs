mod beatmap;
mod common;


use eyre::{Report, Result};

use serde_json::Value;

use std::path::Path;
use std::thread;
use std::time::Duration;
use tokio::sync::Mutex;
use rayon::prelude::*;

mod reader_beatmap;
mod reader_common;
mod reader_gameplay;
mod reader_resultscreen;
mod structs;


pub fn waiting_for_play(p: &Process, state: &mut State, weak: Weak<LoginPage>) -> eyre::Result<()> {
    let mut last_map: MapData = MapData {
        song: "".into(),
        author: "".into(),
        creator: "".into(),
        cover: slint::Image::default(),
        link: "".into(),
        difficulties: "".into(),
        download_progress: 0.0,
        md5: "".into(),
    };

    loop {
        if GameState::from(get_status(p, state)) == GameState::Playing {
            return Ok(());
        }

        if SharedString::from(get_beatmap_md5(p, state)?) != last_map.md5 {
            println!("New map");
            let map = get_beatmap(p, state)?;
            let map_to_move = map.clone();
            last_map = map.clone();
            let handle = weak.clone();
            let path = get_beatmap_path(
                p,
                state
            )?;
            println!("{}", path);
            let img = get_cover_path(p, state)?;
            let audio = get_audio_path(p,state)?;

            std::thread::spawn(move || {
                let song = map_to_move.song.clone();
                let author = map_to_move.author.clone();
                let creator = map_to_move.creator.clone();
                let link = map_to_move.link.clone();
                let difficulties = map_to_move.difficulties.clone();
                let progress = map_to_move.download_progress;
                let md5 = map_to_move.md5.clone();
                let (calc_pp, (b, patterns)) = rayon::join(
                    || calc_pp(&path),
                    || rayon::join(
                        || get_nps(&path, 1.0).unwrap(),
                        || {
                            let patterns = get_patterns(&path).unwrap();
                            analyze_patterns(&patterns)
                        }
                    )
                );


                let values : Vec<f32> = b.iter().map(|kv| kv.value as f32).collect();
                let (avg, max) = {
                    let values: Vec<f32> = b.par_iter()
                        .map(|kv| kv.value as f32)
                        .collect();

                    let (sum, max) = values.par_iter()
                        .fold(
                            || (0.0f32, f32::NEG_INFINITY),
                            |(sum, max), &value| (sum + value, max.max(value))
                        )
                        .reduce(
                            || (0.0f32, f32::NEG_INFINITY),
                            |(sum1, max1), (sum2, max2)| (sum1 + sum2, max1.max(max2))
                        );

                    (sum / values.len() as f32, max)
                };

                handle.upgrade_in_event_loop(move |handle| {
                    let img = Image::load_from_path(Path::new(&img)).unwrap_or_else(|_| {
                        Image::default()
                    });
                    let map_data = MapData {
                        song,
                        author,
                        creator,
                        cover: img,
                        link,
                        difficulties,
                        download_progress: progress,
                        md5,
                    };
                });
            });
        }
    }
}


pub fn controlla() -> Result<()> {
    let interval: Duration = Duration::from_millis(300);
    let mut state = State {
        addresses: StaticAddresses::default(),
    };

    if interval != Duration::from_millis(300) {
        println!("Using non default interval: {}", interval.as_millis());
    }

    'init_loop: loop {
        let p = match Process::initialize("osu!.exe") {
            Ok(p) => p,
            Err(e) => {
                continue 'init_loop;
            }
        };

        println!("Reading static signatures...");
        match StaticAddresses::new(&p) {
            Ok(v) => state.addresses = v,
            Err(e) => match e.downcast_ref::<ProcessError>() {
                Some(&ProcessError::ProcessNotFound) => {
                    continue 'init_loop;
                }
                #[cfg(target_os = "windows")]
                Some(&ProcessError::OsError { .. }) => {
                    println!("{:?}", e);
                    continue 'init_loop;
                }
                Some(_) | None => {
                    println!("{:?}", e);
                    continue 'init_loop;
                }
            },
        };

        println!("Starting reading loop");
        'main_loop: loop {
            std::thread::sleep(Duration::from_millis(1000));
            println!("Waiting For Play");
            let weak_clone = weak.clone();
            if let Err(e) = waiting_for_play(&p, &mut state, weak_clone) {
                match e.downcast_ref::<ProcessError>() {
                    Some(&ProcessError::ProcessNotFound) => {
                        continue 'init_loop;
                    }
                    #[cfg(target_os = "windows")]
                    Some(&ProcessError::OsError { .. }) => {
                        println!("{:?}", e);
                        continue 'init_loop;
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
