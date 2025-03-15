mod file_or_part;
pub mod model_info;
pub mod path;
pub mod race_gender;
pub mod skeleton_slot;
#[cfg(test)]
mod test;

use std::str::FromStr;

use anyhow::Context;
use nom::{
    Parser,
    branch::alt,
    bytes::complete::{tag, take, take_till},
    combinator::map_res,
    sequence::preceded,
};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{
    parser::{
        model_info::ModelInfo,
        path::chara::{
            AccessoryPath, CharacterPath, DemihumanPath, EquipmentPath, MonsterPath, WeaponPath,
        },
    },
    schema::EquipSlotCategory,
};

#[cfg(test)]
type IResult<I, O, E = nom_language::error::VerboseError<I>> = nom::IResult<I, O, E>;

#[cfg(not(test))]
type IResult<I, O, E = nom::error::Error<I>> = nom::IResult<I, O, E>;

#[derive(Debug, PartialEq, Eq)]
pub enum GamePath<'a> {
    Monster(MonsterPath),
    Weapon(WeaponPath),
    Demihuman(DemihumanPath),
    Equipment(EquipmentPath),
    Accessory(AccessoryPath),
    Character(CharacterPath<'a>),
    Icon {
        group: u64,
        primary_id: u64,
        language: Option<Language>,
        hq: bool,
        hires: bool,
    },
    Map {
        primary_id: &'a str,
        variant: u8,
        suffix: Option<char>,
        extra: Option<char>,
    },
    FontTexture(&'a str),
    FontFile {
        family: &'a str,
        size: u8,
    },
}

impl<'a> GamePath<'a> {
    pub fn parse(input: &'a str) -> anyhow::Result<Self> {
        let (left, path) = game_path(input)
            // FIXME: shouldn't need to clone here
            .map_err(|e| {
                #[cfg(not(test))]
                {
                    e.to_owned()
                }

                #[cfg(test)]
                {
                    panic!("do not use GamePath::parse in tests")
                }
            })
            .context("could not parse path")?;
        if !left.is_empty() {
            anyhow::bail!("did not parse entire path");
        }

        Ok(path)
    }
}

// util

fn raw_part(input: &str) -> IResult<&str, &str> {
    take_till(|c| c == '/')(input)
}

fn simple_part_enum<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(raw_part, |part| T::from_str(part)).parse(input)
}

fn path_id<'a, 'b>(before: &'a str) -> impl Fn(&'b str) -> IResult<&'b str, u16> {
    move |input: &'b str| preceded(tag(before), n_digit_id::<u16>(4)).parse(input)
}

fn n_digit_id<T: FromStr>(n: usize) -> impl Fn(&str) -> IResult<&str, T> {
    move |input: &str| map_res(take(n), |id: &str| id.parse::<T>()).parse(input)
}

fn check_repeat_id(expected_id: u16, repeat_id: u16) -> anyhow::Result<()> {
    if expected_id != repeat_id {
        return Err(anyhow::format_err!(
            "id did not match file name: {expected_id} != {repeat_id}",
        ));
    }

    Ok(())
}

fn equip_slot(input: &str) -> IResult<&str, EquipSlot> {
    map_res(take(3_usize), EquipSlot::from_str).parse(input)
}

// main parser

pub fn game_path(input: &str) -> IResult<&str, GamePath> {
    alt((path::common_path, path::chara_path, path::ui_path)).parse(input)
}

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

impl<'a> TryFrom<&'a EquipSlotCategory> for EquipSlot {
    type Error = ();

    fn try_from(value: &'a EquipSlotCategory) -> Result<Self, Self::Error> {
        if value.head == 1 {
            return Ok(Self::Head);
        }

        if value.gloves == 1 {
            return Ok(Self::Hands);
        }

        if value.legs == 1 {
            return Ok(Self::Legs);
        }

        if value.feet == 1 {
            return Ok(Self::Feet);
        }

        if value.body == 1 {
            return Ok(Self::Body);
        }

        if value.ears == 1 {
            return Ok(Self::Ears);
        }

        if value.neck == 1 {
            return Ok(Self::Neck);
        }

        if value.finger_r == 1 {
            return Ok(Self::RFinger);
        }

        if value.finger_l == 1 {
            return Ok(Self::LFinger);
        }

        if value.wrists == 1 {
            return Ok(Self::Wrists);
        }

        Err(())
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

            _ => return None,
        };

        Some(idx)
    }

    pub fn is_accessory(self) -> bool {
        match self {
            Self::Ears => true,
            Self::Neck => true,
            Self::Wrists => true,
            Self::RFinger => true,
            Self::LFinger => true,
            _ => false,
        }
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
