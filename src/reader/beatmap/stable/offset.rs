pub(crate) struct BeatmapOffset {
    pub ptr: i32,
    pub md5: i32,
    pub author: i32,
    pub creator: i32,
    pub title: i32,
    pub difficulty: i32,
    pub folder: i32,
    pub filename: i32,
    pub audio: i32,
    pub cover: i32,
}

pub(crate) static BEATMAP_OFFSET: BeatmapOffset = BeatmapOffset {
    ptr: 0xC,
    md5: 0x6c,
    author: 0x18,
    creator: 0x7C,
    title: 0x24,
    difficulty: 0xAC,
    folder: 0x78,
    filename: 0x90,
    audio: 0x64,
    cover: 0x68,
};


