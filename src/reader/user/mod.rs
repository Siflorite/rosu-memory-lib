pub mod common;
pub mod stable;
use rosu_mem::process::{Process};
use crate::reader::structs::State;
use crate::reader::common::OsuType;
use crate::reader::user::common::UserInfo;


pub struct UserReader<'a> { pub process : &'a Process, pub state : &'a mut State, pub osu_type : OsuType}


impl <'a> UserReader<'a> {
    pub fn new(p: &'a Process, state: &'a mut State, osu_type: OsuType) -> Self {
        Self { process: p, state, osu_type }
    }

    pub fn get_user_info(&mut self) -> eyre::Result<UserInfo> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_user_info(self.process, &mut self.state),
            _ => Err(eyre::eyre!("Unsupported osu type for now")),
        }
    }

    pub fn get_user_id(&mut self) -> eyre::Result<i32> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_user_id(self.process, &mut self.state),
            _ => Err(eyre::eyre!("Unsupported osu type for now")),
        }
    }

    pub fn get_user_bancho_status(&mut self) -> eyre::Result<i32> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_user_bancho_status(self.process, &mut self.state),
            _ => Err(eyre::eyre!("Unsupported osu type for now")),
        }
    }

    pub fn get_user_country_code(&mut self) -> eyre::Result<i32> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_user_country_code(self.process, &mut self.state),
            _ => Err(eyre::eyre!("Unsupported osu type for now")),
        }
    }

    pub fn get_user_username(&mut self) -> eyre::Result<String> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_user_username(self.process, &mut self.state),
            _ => Err(eyre::eyre!("Unsupported osu type for now")),
        }
    }

    pub fn get_user_pp(&mut self) -> eyre::Result<i32> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_user_pp(self.process, &mut self.state),
            _ => Err(eyre::eyre!("Unsupported osu type for now")),
        }
    }

    pub fn get_user_rankedscore(&mut self) -> eyre::Result<i64> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_user_rankedscore(self.process, &mut self.state),
            _ => Err(eyre::eyre!("Unsupported osu type for now")),
        }
    }

    pub fn get_user_level(&mut self) -> eyre::Result<f32> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_user_level(self.process, &mut self.state),
            _ => Err(eyre::eyre!("Unsupported osu type for now")),
        }
    }

    pub fn get_user_playcount(&mut self) -> eyre::Result<i32> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_user_playcount(self.process, &mut self.state),
            _ => Err(eyre::eyre!("Unsupported osu type for now")),
        }
    }

    pub fn get_user_rank(&mut self) -> eyre::Result<i32> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_user_rank(self.process, &mut self.state),
            _ => Err(eyre::eyre!("Unsupported osu type for now")),
        }
    }

    pub fn get_user_playmode(&mut self) -> eyre::Result<i32> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_user_playmode(self.process, &mut self.state),
            _ => Err(eyre::eyre!("Unsupported osu type for now")),
        }
    }

    pub fn get_user_accuracy(&mut self) -> eyre::Result<f64> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_user_accuracy(self.process, &mut self.state),
            _ => Err(eyre::eyre!("Unsupported osu type for now")),
        }
    }
}