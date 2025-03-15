pub mod file_or_part;
pub mod model_info;
pub mod race_gender;
pub mod skeleton_slot;

use serde_repr::{Deserialize_repr, Serialize_repr};

pub use self::{
    file_or_part::{FileOrPart, file_or_part},
    model_info::{ModelInfo, ModelKind, model_info, model_info_with_raw},
    race_gender::{Gender, Race},
    skeleton_slot::{SkeletonSlot, skeleton_slot},
};

enum_str! {
    pub enum PathRoot {
        Bg => "bg",
        BgCommon => "bgcommon",
        Chara => "chara",
        Common => "common",
        Cut => "cut",
        Exd => "exd",
        GameScript => "game_script",
        Music => "music",
        Shader => "shader",
        Sound => "sound",
        Ui => "ui",
        Vfx => "vfx",
    }

    pub enum BgPart {
        Ex1 => "ex1",
        Ex2 => "ex2",
        Ex3 => "ex3",
        Ex4 => "ex4",
        Ex5 => "ex5",
        Ffxiv => "ffxiv",
    }

    pub enum BgCommonPart {
        Collision => "collision",
        Env => "env",
        Hou => "hou",
        Mji => "mji",
        Nature => "nature",
        Sound => "sound",
        Texture => "texture",
        Tmplate => "_tmplate",
        Vfx => "vfx",
        World => "world",
    }

    pub enum CharaPart {
        Accessory => "accessory",
        Action => "action",
        BaseMaterial => "base_material",
        Common => "common",
        Demihuman => "demihuman",
        Equipment => "equipment",
        Human => "human",
        Monster => "monster",
        Trial => "_trial",
        Weapon => "weapon",
        Xls => "xls",
    }

    pub enum CommonPart {
        Bench => "bench",
        Camerashake => "camerashake",
        Font => "font",
        Graphics => "graphics",
        Hardwarecursor => "hardwarecursor",
        Savedata => "savedata",
        Softwarecursor => "softwarecursor",
    }

    pub enum CutPart {
        Ex1 => "ex1",
        Ex2 => "ex2",
        Ex3 => "ex3",
        Ex4 => "ex4",
        Ex5 => "ex5",
        Ffxiv => "ffxiv",
    }

    pub enum GameScriptPart {
        Content => "content",
        Custom => "custom",
        Fate => "fate",
        GuildOrder => "guild_order",
        Leve => "leve",
        MassivePcContent => "massive_pc_content",
        Opening => "opening",
        PartyContent => "party_content",
        PublicContent => "public_content",
        Quest => "quest",
        Raid => "raid",
        Shop => "shop",
        Story => "story",
        System => "system",
        Test => "test",
        Transport => "transport",
        TreasureHunt => "treasure_hunt",
        Warp => "warp",
    }

    pub enum MusicPart {
        Ex1 => "ex1",
        Ex2 => "ex2",
        Ex3 => "ex3",
        Ex4 => "ex4",
        Ex5 => "ex5",
        Ffxiv => "ffxiv",
    }

    pub enum ShaderPart {
        PostEffect => "posteffect",
        Shcd => "shcd",
        Shpk => "shpk",
        Sm5 => "sm5",
    }

    pub enum SoundPart {
        Battle => "battle",
        BgObj => "bg_obj",
        Cut => "cut",
        Debug => "_debug",
        Event => "event",
        Foot => "foot",
        Instruments => "instruments",
        Nc => "nc",
        Score => "score",
        Stream => "stream",
        Strm => "strm",
        System => "system",
        Vfx => "vfx",
        Vibration => "vibration",
        Voice => "voice",
        Zingle => "zingle",
    }

    pub enum UiPart {
        Common => "common",
        Crest => "crest",
        Icon => "icon",
        LoadingImage => "loadingimage",
        Map => "map",
        Uld => "uld",
    }

    pub enum VfxPart {
        Action => "action",
        Aoz => "aoz",
        Aoz2 => "aoz2",
        Aoz3 => "aoz3",
        Benchmark => "benchmark",
        Benchmark50 => "benchmark50",
        Benchmark60 => "benchmark60",
        Benchmark70 => "benchmark70",
        Bgb => "bgb",
        Bkc => "bkc",
        Camera => "camera",
        Channeling => "channeling",
        Common => "common",
        CrMon => "cr_mon",
        Cut => "cut",
        Debug => "_debug",
        EmoteSp => "emote_sp",
        Equipment => "equipment",
        Eureka => "eureka",
        Event => "event",
        Fishing => "fishing",
        FlyMount => "fly_mount",
        General => "general",
        Gff => "gff",
        Grouppose => "grouppose",
        GsRoulette => "gs_roulette",
        Guild => "guild",
        Harpoon => "harpoon",
        Item => "item",
        Lcut => "lcut",
        Limitbreak => "limitbreak",
        Live => "live",
        Lockon => "lockon",
        Lovm => "lovm",
        Mks => "mks",
        Monster => "monster",
        MountSp => "mount_sp",
        Nomal => "nomal",
        Okamuray => "okamuray",
        Omen => "omen",
        PcCommon => "pc_common",
        PcContentsaction => "pc_contentsaction",
        Pop => "pop",
        Pvp => "pvp",
        Rdb => "rdb",
        Rpm => "rpm",
        Rrp => "rrp",
        Sxt => "sxt",
        Temporary => "temporary",
        Test => "_test",
        Ui => "ui",
        Weapon => "weapon",
        Ws => "ws",
    }

    #[derive(Serialize_repr, Deserialize_repr, Hash, PartialOrd, Ord)]
    #[repr(u8)]
    pub enum EquipSlot {
        Head => "met",
        Hands => "glv",
        Legs => "dwn",
        Feet => "sho",
        Body => "top",
        Ears => "ear",
        Neck => "nek",
        RFinger => "rir",
        LFinger => "ril",
        Wrists => "wrs",
    }

    pub enum Language {
        English => "en",
        Japanese => "ja",
        German => "de",
        French => "fr",
    }
}

impl EquipSlot {
    pub fn to_id(self) -> u64 {
        match self {
            Self::Head => 3,
            Self::Hands => 5,
            Self::Legs => 7,
            Self::Feet => 8,
            Self::Body => 4,
            Self::Ears => 9,
            Self::Neck => 10,
            Self::RFinger => 12,
            Self::LFinger => 14,
            Self::Wrists => 11,
        }
    }

    pub fn to_imc_part_idx(self) -> Option<usize> {
        let idx = match self {
            Self::Head => 0,
            Self::Body => 1,
            Self::Hands => 2,
            Self::Legs => 3,
            Self::Feet => 4,

            Self::Ears => 0,
            Self::Neck => 1,
            Self::Wrists => 2,
            Self::RFinger => 3,
            Self::LFinger => 4,
        };

        Some(idx)
    }

    pub fn is_accessory(self) -> bool {
        matches!(
            self,
            Self::Ears | Self::Neck | Self::Wrists | Self::RFinger | Self::LFinger
        )
    }

    pub fn abbreviation(&self) -> &str {
        match self {
            Self::Head => "m",
            Self::Hands => "g",
            Self::Legs => "d",
            Self::Feet => "s",
            Self::Body => "t",
            _ => "",
        }
    }
}
