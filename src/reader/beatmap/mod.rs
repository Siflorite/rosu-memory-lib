pub mod common;
pub mod stable;

use rosu_mem::process::{Process};
use crate::reader::beatmap::common::BeatmapInfo;
use crate::reader::structs::State;
use crate::reader::common::OsuType;
use crate::reader::common::GameMode;
use crate::reader::beatmap::common::BeatmapStatus;
use crate::reader::beatmap::common::BeatmapStarRating;
use crate::reader::beatmap::common::BeatmapStats;


pub fn get_beatmap_info(p: &Process, state: &mut State, osu_type: OsuType) -> eyre::Result<BeatmapInfo>
{
    match osu_type {
        OsuType::Stable => stable::memory::get_beatmap_info(p, state),
        _ => Err(eyre::eyre!("Unsupported osu type for now")),
    }
}


pub struct BeatmapReader<'a> { pub process : &'a Process, pub state : &'a mut State, pub osu_type : OsuType}

impl<'a> BeatmapReader<'a> {
    pub fn new(p: &'a Process, state: &'a mut State, osu_type: OsuType) -> eyre::Result<Self> {
        Ok(Self { process: p, state, osu_type })
    }

    pub fn get_beatmap_info(&mut self) -> eyre::Result<BeatmapInfo> {
        get_beatmap_info(self.process, &mut self.state, self.osu_type)
    }


    pub fn get_beatmap_path(&mut self) -> eyre::Result<String> {
        match self.osu_type {
            OsuType::Stable => stable::file::get_beatmap_path(self.process, &mut self.state),
            _ => Err(eyre::eyre!("Unsupported osu type for now")),
        }
    }

    pub fn get_audio_path(&mut self) -> eyre::Result<String> {
        match self.osu_type {
            OsuType::Stable => stable::file::get_audio_path(self.process, &mut self.state),
            _ => Err(eyre::eyre!("Unsupported osu type for now")),
        }
    }

    pub fn get_beatmap_md5(&mut self) -> eyre::Result<String> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_beatmap_md5(self.process, &mut self.state),
            _ => Err(eyre::eyre!("Unsupported osu type for now")),
        }
    }

    pub fn get_beatmap_id(&mut self) -> eyre::Result<i32> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_beatmap_id(self.process, &mut self.state),
            _ => Err(eyre::eyre!("Unsupported osu type for now")),
        }
    }

    pub fn get_beatmap_set_id(&mut self) -> eyre::Result<i32> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_beatmap_set_id(self.process, &mut self.state),
            _ => Err(eyre::eyre!("Unsupported osu type for now")),
        }
    }

    pub fn get_beatmap_mode(&mut self) -> eyre::Result<GameMode> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_beatmap_mode(self.process, &mut self.state),
            _ => Err(eyre::eyre!("Unsupported osu type for now")),
        }
    }

    pub fn get_beatmap_tags(&mut self) -> eyre::Result<String> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_beatmap_tags(self.process, &mut self.state),
            _ => Err(eyre::eyre!("Unsupported osu type for now")),
        }
    }

    pub fn get_beatmap_length(&mut self) -> eyre::Result<i32> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_beatmap_length(self.process, &mut self.state),
            _ => Err(eyre::eyre!("Unsupported osu type for now")),
        }
    }

    pub fn get_beatmap_drain_time(&mut self) -> eyre::Result<i32> {
            match self.osu_type {
                OsuType::Stable => stable::memory::get_beatmap_drain_time(self.process, &mut self.state),
                _ => Err(eyre::eyre!("Unsupported osu type for now")),
            }
        }

        pub fn get_beatmap_status(&mut self) -> eyre::Result<BeatmapStatus> {
            match self.osu_type {
                OsuType::Stable => stable::memory::get_beatmap_status(self.process, &mut self.state),
                _ => Err(eyre::eyre!("Unsupported osu type for now")),
            }
        }

        pub fn get_author(&mut self) -> eyre::Result<String> {
            match self.osu_type {
                OsuType::Stable => stable::memory::get_author(self.process, &mut self.state),
                _ => Err(eyre::eyre!("Unsupported osu type for now")),
            }
        }

        pub fn get_creator(&mut self) -> eyre::Result<String> {
            match self.osu_type {
                OsuType::Stable => stable::memory::get_creator(self.process, &mut self.state),
                _ => Err(eyre::eyre!("Unsupported osu type for now")),
            }
        }

        pub fn get_title_romanized(&mut self) -> eyre::Result<String> {
            match self.osu_type {
                OsuType::Stable => stable::memory::get_title_romanized(self.process, &mut self.state),
                _ => Err(eyre::eyre!("Unsupported osu type for now")),
            }
        }

        pub fn get_title_original(&mut self) -> eyre::Result<String> {
            match self.osu_type {
                OsuType::Stable => stable::memory::get_title_original(self.process, &mut self.state),
                _ => Err(eyre::eyre!("Unsupported osu type for now")),
            }
        }

        pub fn get_difficulty(&mut self) -> eyre::Result<String> {
            match self.osu_type {
                OsuType::Stable => stable::memory::get_difficulty(self.process, &mut self.state),
                _ => Err(eyre::eyre!("Unsupported osu type for now")),
            }
        }

        pub fn get_beatmap_od(&mut self) -> eyre::Result<f32> {
            match self.osu_type {
                OsuType::Stable => stable::memory::get_beatmap_od(self.process, &mut self.state),
                _ => Err(eyre::eyre!("Unsupported osu type for now")),
            }
        }

        pub fn get_beatmap_ar(&mut self) -> eyre::Result<f32> { 
            match self.osu_type {
                OsuType::Stable => stable::memory::get_beatmap_ar(self.process, &mut self.state),
                _ => Err(eyre::eyre!("Unsupported osu type for now")),
            }
        }

        pub fn get_beatmap_cs(&mut self) -> eyre::Result<f32> {
            match self.osu_type {
                OsuType::Stable => stable::memory::get_beatmap_cs(self.process, &mut self.state),   
                _ => Err(eyre::eyre!("Unsupported osu type for now")),
            }
        }

        pub fn get_beatmap_hp(&mut self) -> eyre::Result<f32> {
            match self.osu_type {
                OsuType::Stable => stable::memory::get_beatmap_hp(self.process, &mut self.state),
                _ => Err(eyre::eyre!("Unsupported osu type for now")),
            }
        }
        

        pub fn get_beatmap_object_count(&mut self) -> eyre::Result<u32> {
            match self.osu_type {
                OsuType::Stable => stable::memory::get_beatmap_object_count(self.process, &mut self.state),
                _ => Err(eyre::eyre!("Unsupported osu type for now")),
            }
        }

        pub fn get_beatmap_slider_count(&mut self) -> eyre::Result<i32> {
            match self.osu_type {
                OsuType::Stable => stable::memory::get_beatmap_slider_count(self.process, &mut self.state),
                _ => Err(eyre::eyre!("Unsupported osu type for now")),
            }
        }
        


        pub fn get_beatmap_star_rating(&mut self) -> eyre::Result<BeatmapStarRating> {
            match self.osu_type {
                OsuType::Stable => stable::file::get_beatmap_star_rating(self.process, &mut self.state),
                _ => Err(eyre::eyre!("Unsupported osu type for now")),
            }
        }

        pub fn get_beatmap_stats(&mut self) -> eyre::Result<BeatmapStats> {
            match self.osu_type {
                OsuType::Stable => stable::memory::get_beatmap_stats(self.process, &mut self.state),
                _ => Err(eyre::eyre!("Unsupported osu type for now")),
            }
        }
        
}