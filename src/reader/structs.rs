use std::str::FromStr;
use rosu_mem::{
    process::{Process, ProcessTraits},
    signature::Signature,
};


#[derive(Default, Clone)]
pub struct StaticAddresses {
    pub base: i32,
    pub status: i32,
    pub menu_mods: i32,
    pub rulesets: i32,
    pub playtime: i32,
    pub skin: i32,
    pub chat_checker: i32,
    pub audio_time_base: i32,
    pub ig_time_base : i32,
    pub settings : i32,
}

#[derive(Debug, Default)]
pub struct Hit{
    pub _geki:i16,
    pub _300:i16,
    pub _katu:i16,
    pub _100:i16,
    pub _50:i16,
    pub _miss:i16,
}

#[derive(Debug, Clone)]
pub(crate) struct SignatureBase {
    base_sig: &'static str,
    status_sig: &'static str,
    menu_mods_sig: &'static str,
    rulesets_sig: &'static str,
    playtime_sig: &'static str,
    skin_sig: &'static str,
    chat_checker_sig: &'static str,
    audio_time_base_sig: &'static str,
    ig_time_base_sig: &'static str,
    settings_sig: &'static str,
}

pub(crate) const SIGNATURES: SignatureBase = SignatureBase {
    base_sig: "F8 01 74 04 83 65",
    status_sig: "48 83 F8 04 73 1E",
    menu_mods_sig: "C8 FF ?? ?? ?? ?? ?? 81 0D ?? ?? ?? ?? 00 08 00 00",
    rulesets_sig: "7D 15 A1 ?? ?? ?? ?? 85 C0",
    playtime_sig: "5E 5F 5D C3 A1 ?? ?? ?? ?? 89 ?? 04",
    skin_sig: "75 21 8B 1D",
    chat_checker_sig: "0A D7 23 3C 00 00 ?? 01",
    audio_time_base_sig: "DB 5C 24 34 8B 44 24 34",
    ig_time_base_sig: "EB 0A A1 ?? ?? ?? ?? A3",
    settings_sig: "83 E0 20 85 C0 7E 2F",
};




impl StaticAddresses {
    pub fn new(p: &Process) -> Result<Self,Box<dyn std::error::Error>> {
        println!("Reading base signature...");
        let base_sign = Signature::from_str(&SIGNATURES.base_sig)?;
        let base = p.read_signature(&base_sign)?;
        println!("Base signature found at: {:#x}", base);

        println!("Reading status signature...");
        let status_sign = Signature::from_str(&SIGNATURES.status_sig)?;
        let status = p.read_signature(&status_sign)?;
        println!("Status signature found at: {:#x}", status);

        println!("Reading menu mods signature...");
        let menu_mods_sign = Signature::from_str(&SIGNATURES.menu_mods_sig)?;
        let menu_mods = p.read_signature(&menu_mods_sign)?;
        println!("Menu mods signature found at: {:#x}", menu_mods);

        println!("Reading rulesets signature...");
        let rulesets_sign = Signature::from_str(&SIGNATURES.rulesets_sig)?;
        let rulesets = p.read_signature(&rulesets_sign)?;
        println!("Rulesets signature found at: {:#x}", rulesets);

        println!("Reading playtime signature...");
        let playtime_sign = Signature::from_str(&SIGNATURES.playtime_sig)?;
        let playtime = p.read_signature(&playtime_sign)?;
        println!("Playtime signature found at: {:#x}", playtime);

        println!("Reading skin signature...");
        let skin_sign = Signature::from_str(&SIGNATURES.skin_sig)?;
        let skin = p.read_signature(&skin_sign)?;
        println!("Skin signature found at: {:#x}", skin);

        println!("Reading chat checker signature...");
        let chat_checker_sign = Signature::from_str(&SIGNATURES.chat_checker_sig)?;
        let chat_checker = p.read_signature(&chat_checker_sign)?;
        println!("Chat checker signature found at: {:#x}", chat_checker);

        println!("Reading audio time base signature...");
        let audio_time_base_sign = Signature::from_str(&SIGNATURES.audio_time_base_sig)?;
        let audio_time_base = p.read_signature(&audio_time_base_sign)?;
        println!("Audio time base signature found at: {:#x}", audio_time_base);

        println!("Reading in-game time base signature...");
        let ig_time_base_sign = Signature::from_str(&SIGNATURES.ig_time_base_sig)?;
        let ig_time_base = p.read_signature(&ig_time_base_sign)?;
        println!("In-game time base signature found at: {:#x}", ig_time_base);

        println!("Reading settings signature...");
        let settings_sign = Signature::from_str(&SIGNATURES.settings_sig)?;
        let settings = p.read_signature(&settings_sign)?;
        println!("Settings signature found at: {:#x}", settings);

        Ok(Self {
            base,
            status,
            menu_mods,
            rulesets,
            playtime,
            skin,
            chat_checker,
            audio_time_base,
            ig_time_base,
            settings,
        })
    }
}

/* for pp counter for later
#[derive(Default)]
pub struct InnerValues {
    pub gradual_performance_current: Option<GradualPerformance<'static>>,
    pub current_beatmap_perf: Option<PerformanceAttributes>,
}


impl InnerValues {
    pub fn reset(&mut self) {
        self.current_beatmap_perf = None;
        self.gradual_performance_current = None;
    }
}
*/
#[derive(Default,Clone)]
pub struct State {
    pub addresses: StaticAddresses,
}





