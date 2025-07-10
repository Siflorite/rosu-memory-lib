pub mod stable;
pub mod common;
use rosu_mem::process::{Process};
use crate::reader::structs::State;
use crate::reader::common::OsuType;
use crate::reader::structs::Hit;
use crate::reader::resultscreen::common::ResultScreenInfo;
use crate::reader::common::GameMode;

pub struct ResultScreenReader<'a> { pub process : &'a Process, pub state : &'a mut State, pub osu_type : OsuType}

impl<'a> ResultScreenReader<'a> {
    pub fn new(p: &'a Process, state: &'a mut State, osu_type: OsuType) -> Self {
        Self { process: p, state: state, osu_type: osu_type }
    }

    pub fn get_result_screen_info(&mut self) -> eyre::Result<ResultScreenInfo> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_result_screen_info(self.process, &mut self.state),
            _ => Err(eyre::eyre!("Unsupported osu type for now")),
        }
    }

    pub fn get_result_username(&mut self) -> eyre::Result<String> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_result_username(self.process, &mut self.state),
            _ => Err(eyre::eyre!("Unsupported osu type for now")),
        }
    }

    pub fn get_result_score(&mut self) -> eyre::Result<i32> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_result_score(self.process, &mut self.state),
            _ => Err(eyre::eyre!("Unsupported osu type for now")),
        }
    }

    pub fn get_result_mode(&mut self) -> eyre::Result<GameMode> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_result_mode(self.process, &mut self.state),
            _ => Err(eyre::eyre!("Unsupported osu type for now")),
        }
    }

    pub fn get_result_hit_300(&mut self) -> eyre::Result<i16> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_result_hit_300(self.process, &mut self.state),
            _ => Err(eyre::eyre!("Unsupported osu type for now")),
        }
    }

    pub fn get_result_hit_100(&mut self) -> eyre::Result<i16> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_result_hit_100(self.process, &mut self.state),
            _ => Err(eyre::eyre!("Unsupported osu type for now")),
        }
    }

    pub fn get_result_hit_50(&mut self) -> eyre::Result<i16> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_result_hit_50(self.process, &mut self.state),
            _ => Err(eyre::eyre!("Unsupported osu type for now")),
        }
    }


    pub fn get_result_hit_miss(&mut self) -> eyre::Result<i16> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_result_hit_miss(self.process, &mut self.state),
            _ => Err(eyre::eyre!("Unsupported osu type for now")),
        }
    }

    pub fn get_result_hit_geki(&mut self) -> eyre::Result<i16> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_result_hit_geki(self.process, &mut self.state),
            _ => Err(eyre::eyre!("Unsupported osu type for now")),
        }
    }

    pub fn get_result_hit_katu(&mut self) -> eyre::Result<i16> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_result_hit_katu(self.process, &mut self.state),
            _ => Err(eyre::eyre!("Unsupported osu type for now")),
        }
    }
    

    pub fn get_result_hits(&mut self) -> eyre::Result<Hit> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_result_hits(self.process, &mut self.state),
            _ => Err(eyre::eyre!("Unsupported osu type for now")),
        }
    }
    
    pub fn get_result_accuracy(&mut self) -> eyre::Result<f64> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_result_accuracy(self.process, &mut self.state),
            _ => Err(eyre::eyre!("Unsupported osu type for now")),
        }
    }

    pub fn get_result_max_combo(&mut self) -> eyre::Result<i16> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_result_max_combo(self.process, &mut self.state),
            _ => Err(eyre::eyre!("Unsupported osu type for now")),
        }
    }
    
    

}