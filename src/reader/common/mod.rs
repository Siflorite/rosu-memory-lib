pub mod stable;
use serde::{Deserialize, Serialize};
use crate::reader::structs::{State};
use rosu_mem::process::{Process};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum OsuType{
    #[default]
    Stable,
    Lazer
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default, Serialize, Deserialize)]
pub enum GameMode{
    Osu,
    Taiko,
    Catch,
    Mania,
    #[default]
    Unknown,
}
impl From<u32> for GameMode {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Osu,
            1 => Self::Taiko,
            2 => Self::Catch,
            3 => Self::Mania,
            _ => Self::Unknown,
        }
    }
}

impl From<i32> for GameMode {
    fn from(value: i32) -> Self {
        Self::from(value as u32)
    }
}
impl GameMode {
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        match self {
            GameMode::Osu => "std".to_string(),
            GameMode::Taiko => "taiko".to_string(),
            GameMode::Catch => "catch".to_string(),
            GameMode::Mania => "mania".to_string(),
            GameMode::Unknown => "unknown".to_string(),
        }
    }
}

#[derive( Debug, Default, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
pub enum GameState {
    MainMenu = 0,
    Editor = 1,
    Playing = 2,
    Exit = 3,
    EditorSongSelect = 4,
    SongSelect = 5,
    SelectDrawing = 6,
    ResultScreen = 7,
    Update = 8,
    Busy = 9,
    MultiplayerLobbySelect = 11,
    MultiplayerLobby = 12,
    MultiplayerSongSelect = 13,
    MultiplayerResultScreen = 14,
    OffsetWizard = 16,
    MultiplayerResultScreenTagCoop = 17,
    MultiplayerResultScreenTeamVs = 18,
    SongImport = 19,
    #[default]
    Unknown,
}


impl From<u32> for GameState {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::MainMenu,
            1 => Self::Editor,
            2 => Self::Playing,
            3 => Self::Exit, // Very not useful :)
            4 => Self::EditorSongSelect,
            5 => Self::SongSelect,
            6 => Self::SelectDrawing, // Idk wwhat this shit is but tosu said its that lol
            7 => Self::ResultScreen,
            8 => Self::Update, // Idk what this shit is but tosu said its that lol
            9 => Self::Busy, // Idk what this shit is but tosu said its that lol
            10 => Self::Unknown, // if tosu doesnt know i will not too
            11 => Self::MultiplayerLobbySelect,
            12 => Self::MultiplayerLobby,
            13 => Self::MultiplayerSongSelect,
            14 => Self::MultiplayerResultScreen,
            16 => Self::OffsetWizard,
            17 => Self::MultiplayerResultScreenTagCoop,
            18 => Self::MultiplayerResultScreenTeamVs,
            19 => Self::SongImport,
            _ => Self::Unknown,
        }
    }
}


#[derive(Debug, Clone)]
pub enum Error {
    NotAvailable(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NotAvailable(msg) => write!(f, "{msg}"),
        }
    }
}


pub struct CommonReader<'a> { pub process : &'a Process, pub state : &'a mut State, pub osu_type : OsuType}

impl<'a> CommonReader<'a> {
    pub fn new(p: &'a Process, state: &'a mut State, osu_type: OsuType) -> Self {
        Self { process: p, state: state, osu_type: osu_type }
    }

    pub fn get_game_state(&mut self) -> eyre::Result<GameState> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_game_state(self.process, &mut self.state),
            _ => Err(eyre::eyre!("Unsupported osu type for now")),
        }
    }

    pub fn get_menu_mods(&mut self) -> eyre::Result<i32> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_menu_mods(self.process, &mut self.state),
            _ => Err(eyre::eyre!("Unsupported osu type for now")),
        }
    }
    
    pub fn get_path_folder(&mut self) -> eyre::Result<String> {
        match self.osu_type {
            OsuType::Stable => stable::memory::get_path_folder(self.process, &mut self.state),
            _ => Err(eyre::eyre!("Unsupported osu type for now")),
        }
    }
    
    pub fn check_game_state(&mut self, g_state: GameState) -> eyre::Result<bool> {
        match self.osu_type {
            OsuType::Stable => stable::memory::check_game_state(self.process, &mut self.state, g_state),
            _ => Err(eyre::eyre!("Unsupported osu type for now")),
        }
    }
}

