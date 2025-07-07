pub mod stable;
pub enum OsuType{
    Stable,
    Lazer
}



#[derive( Debug, Default, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
pub enum GameState {
    PreSongSelect = 0,
    Playing = 2,
    SongSelect = 5,
    EditorSongSelect = 4,
    ResultScreen = 7,
    MultiplayerLobbySelect = 11,
    MultiplayerLobby = 12,
    MultiplayerResultScreen = 14,

    #[default]
    Unknown,
}


impl From<u32> for GameState {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::PreSongSelect,
            2 => Self::Playing,
            4 => Self::EditorSongSelect,
            5 => Self::SongSelect,
            7 => Self::ResultScreen,
            11 => Self::MultiplayerLobbySelect,
            12 => Self::MultiplayerLobby,
            14 => Self::MultiplayerResultScreen,
            _ => Self::Unknown,
        }
    }
}


pub enum GameMode{
    Osu,
    Taiko,
    Catch,
    Mania,
}