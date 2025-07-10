pub mod stable;
pub mod common;

use rosu_mem::process::{Process};
use crate::reader::structs::{State};
use crate::reader::common::OsuType;
use crate::reader::structs::Hit;
use crate::reader::gameplay::common::GameplayInfo;

pub struct GameplayReader<'a> { pub process : &'a Process, pub state : &'a mut State, pub osu_type : OsuType}

impl<'a> GameplayReader<'a> {
    pub fn new(p: &'a Process, state: &'a mut State, osu_type: OsuType) -> Self {
        Self { process: p, state, osu_type }
    }

    pub fn get_score_gameplay(&mut self) -> eyre::Result<i32> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_score_gameplay(self.process, self.state),
            _ => Err(eyre::eyre!("Unsupported osu type for now")),
        }
    }

    pub fn get_mods(&mut self) -> eyre::Result<u32> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_mods(self.process, self.state),
            _ => Err(eyre::eyre!("Unsupported osu type for now")),
        }
    }

    pub fn get_combo(&mut self) -> eyre::Result<i16> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_combo(self.process, self.state),
            _ => Err(eyre::eyre!("Unsupported osu type for now")),
        }
    }

    pub fn get_max_combo(&mut self) -> eyre::Result<i16> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_max_combo(self.process, self.state),
            _ => Err(eyre::eyre!("Unsupported osu type for now")),
        }
    }

    pub fn get_current_hp(&mut self) -> eyre::Result<f64> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_current_hp(self.process, self.state),
            _ => Err(eyre::eyre!("Unsupported osu type for now")),
        }
    }

    pub fn get_username(&mut self) -> eyre::Result<String> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_username(self.process, self.state),
            _ => Err(eyre::eyre!("Unsupported osu type for now")),
        }
    }

    pub fn get_ig_time(&mut self) -> eyre::Result<i32> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_ig_time(self.process, self.state),
            _ => Err(eyre::eyre!("Unsupported osu type for now")),
        }
    }

    pub fn get_retries(&mut self) -> eyre::Result<i32> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_retries(self.process, self.state),
            _ => Err(eyre::eyre!("Unsupported osu type for now")),
        }
    }
    
    pub fn get_hits_300(&mut self) -> eyre::Result<i16> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_hits_300(self.process, self.state),
            _ => Err(eyre::eyre!("Unsupported osu type for now")),
        }
    }
    
    pub fn get_hits_100(&mut self) -> eyre::Result<i16> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_hits_100(self.process, self.state),
            _ => Err(eyre::eyre!("Unsupported osu type for now")),
        }
    }
    
    pub fn get_hits_50(&mut self) -> eyre::Result<i16> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_hits_50(self.process, self.state),
            _ => Err(eyre::eyre!("Unsupported osu type for now")),
        }
    }
    
    pub fn get_hits_miss(&mut self) -> eyre::Result<i16> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_hits_miss(self.process, self.state),
            _ => Err(eyre::eyre!("Unsupported osu type for now")),
        }
    }
    
    pub fn get_hits_geki(&mut self) -> eyre::Result<i16> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_hits_geki(self.process, self.state),
            _ => Err(eyre::eyre!("Unsupported osu type for now")),
        }
    }
    
    pub fn get_hits_katu(&mut self) -> eyre::Result<i16> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_hits_katu(self.process, self.state),
            _ => Err(eyre::eyre!("Unsupported osu type for now")),
        }
    }
    
    pub fn get_hits(&mut self) -> eyre::Result<Hit> {
        match self.osu_type {   
            OsuType::Stable => stable::memory::get_hits(self.process, self.state),
            _ => Err(eyre::eyre!("Unsupported osu type for now")),
        }
    }

    pub fn get_gameplay_info(&mut self) -> eyre::Result<GameplayInfo> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_gameplay_info(self.process, self.state),
            _ => Err(eyre::eyre!("Unsupported osu type for now")),
        }
    }
}
