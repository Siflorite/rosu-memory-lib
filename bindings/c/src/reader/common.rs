use libc::{c_int};
use rosu_memory_lib::common::GameMode;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum RosuGameMode {
    Osu = 0,
    Taiko = 1,
    Catch = 2,
    Mania = 3,
}

impl From<GameMode> for RosuGameMode {
    fn from(mode: GameMode) -> Self {
        match mode {
            GameMode::Osu => RosuGameMode::Osu,
            GameMode::Taiko => RosuGameMode::Taiko,
            GameMode::Catch => RosuGameMode::Catch,
            GameMode::Mania => RosuGameMode::Mania,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum RosuBeatmapStatus {
    Unknown = -2,
    NotSubmitted = -1,
    Pending = 0,
    UpdateAvailable = 1,
    Ranked = 2,
    Approved = 3,
    Qualified = 4,
    Loved = 5,
} 