use nom::{Parser, combinator::map_res};

use crate::parser::{
    IResult, n_digit_id,
    race_gender::{Gender, Race},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ModelInfo {
    pub race: Option<Race>,
    pub gender: Gender,
    pub kind: ModelKind,
}

impl TryFrom<u16> for ModelInfo {
    type Error = anyhow::Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        #[allow(clippy::zero_prefixed_literal)]
        let info = match value {
            0101 => ModelInfo {
                race: Some(Race::Midlander),
                gender: Gender::Male,
                kind: ModelKind::Player,
            },
            0102 => ModelInfo {
                race: Some(Race::Midlander),
                gender: Gender::Male,
                kind: ModelKind::Unknown,
            },
            0103 => ModelInfo {
                race: Some(Race::Midlander),
                gender: Gender::Male,
                kind: ModelKind::Unknown,
            },
            0104 => ModelInfo {
                race: Some(Race::Midlander),
                gender: Gender::Male,
                kind: ModelKind::Npc,
            },
            0201 => ModelInfo {
                race: Some(Race::Midlander),
                gender: Gender::Female,
                kind: ModelKind::Player,
            },
            0202 => ModelInfo {
                race: Some(Race::Midlander),
                gender: Gender::Female,
                kind: ModelKind::Unknown,
            },
            0203 => ModelInfo {
                race: Some(Race::Midlander),
                gender: Gender::Female,
                kind: ModelKind::Unknown,
            },
            0204 => ModelInfo {
                race: Some(Race::Midlander),
                gender: Gender::Female,
                kind: ModelKind::Npc,
            },
            0301 => ModelInfo {
                race: Some(Race::Highlander),
                gender: Gender::Male,
                kind: ModelKind::Player,
            },
            0304 => ModelInfo {
                race: Some(Race::Highlander),
                gender: Gender::Male,
                kind: ModelKind::Npc,
            },
            0401 => ModelInfo {
                race: Some(Race::Highlander),
                gender: Gender::Female,
                kind: ModelKind::Player,
            },
            0404 => ModelInfo {
                race: Some(Race::Highlander),
                gender: Gender::Female,
                kind: ModelKind::Npc,
            },
            0501 => ModelInfo {
                race: Some(Race::Elezen),
                gender: Gender::Male,
                kind: ModelKind::Player,
            },
            0504 => ModelInfo {
                race: Some(Race::Elezen),
                gender: Gender::Male,
                kind: ModelKind::Npc,
            },
            0601 => ModelInfo {
                race: Some(Race::Elezen),
                gender: Gender::Female,
                kind: ModelKind::Player,
            },
            0604 => ModelInfo {
                race: Some(Race::Elezen),
                gender: Gender::Female,
                kind: ModelKind::Npc,
            },
            0701 => ModelInfo {
                race: Some(Race::Miqote),
                gender: Gender::Male,
                kind: ModelKind::Player,
            },
            0704 => ModelInfo {
                race: Some(Race::Miqote),
                gender: Gender::Male,
                kind: ModelKind::Npc,
            },
            0801 => ModelInfo {
                race: Some(Race::Miqote),
                gender: Gender::Female,
                kind: ModelKind::Player,
            },
            0804 => ModelInfo {
                race: Some(Race::Miqote),
                gender: Gender::Female,
                kind: ModelKind::Npc,
            },
            0901 => ModelInfo {
                race: Some(Race::Roegadyn),
                gender: Gender::Male,
                kind: ModelKind::Player,
            },
            0904 => ModelInfo {
                race: Some(Race::Roegadyn),
                gender: Gender::Male,
                kind: ModelKind::Npc,
            },
            1001 => ModelInfo {
                race: Some(Race::Roegadyn),
                gender: Gender::Female,
                kind: ModelKind::Player,
            },
            1004 => ModelInfo {
                race: Some(Race::Roegadyn),
                gender: Gender::Female,
                kind: ModelKind::Npc,
            },
            1101 => ModelInfo {
                race: Some(Race::Lalafell),
                gender: Gender::Male,
                kind: ModelKind::Player,
            },
            1104 => ModelInfo {
                race: Some(Race::Lalafell),
                gender: Gender::Male,
                kind: ModelKind::Npc,
            },
            1201 => ModelInfo {
                race: Some(Race::Lalafell),
                gender: Gender::Female,
                kind: ModelKind::Player,
            },
            1204 => ModelInfo {
                race: Some(Race::Lalafell),
                gender: Gender::Female,
                kind: ModelKind::Npc,
            },
            1301 => ModelInfo {
                race: Some(Race::AuRa),
                gender: Gender::Male,
                kind: ModelKind::Player,
            },
            1304 => ModelInfo {
                race: Some(Race::AuRa),
                gender: Gender::Male,
                kind: ModelKind::Npc,
            },
            1401 => ModelInfo {
                race: Some(Race::AuRa),
                gender: Gender::Female,
                kind: ModelKind::Player,
            },
            1404 => ModelInfo {
                race: Some(Race::AuRa),
                gender: Gender::Female,
                kind: ModelKind::Npc,
            },
            1501 => ModelInfo {
                race: Some(Race::Hrothgar),
                gender: Gender::Male,
                kind: ModelKind::Player,
            },
            1504 => ModelInfo {
                race: Some(Race::Hrothgar),
                gender: Gender::Male,
                kind: ModelKind::Npc,
            },
            1601 => ModelInfo {
                race: Some(Race::Hrothgar),
                gender: Gender::Female,
                kind: ModelKind::Player,
            },
            1604 => ModelInfo {
                race: Some(Race::Hrothgar),
                gender: Gender::Female,
                kind: ModelKind::Npc,
            },
            1701 => ModelInfo {
                race: Some(Race::Viera),
                gender: Gender::Male,
                kind: ModelKind::Player,
            },
            1704 => ModelInfo {
                race: Some(Race::Viera),
                gender: Gender::Male,
                kind: ModelKind::Npc,
            },
            1801 => ModelInfo {
                race: Some(Race::Viera),
                gender: Gender::Female,
                kind: ModelKind::Player,
            },
            1804 => ModelInfo {
                race: Some(Race::Viera),
                gender: Gender::Female,
                kind: ModelKind::Npc,
            },
            9104 => ModelInfo {
                race: None,
                gender: Gender::Male,
                kind: ModelKind::Npc,
            },
            9204 => ModelInfo {
                race: None,
                gender: Gender::Female,
                kind: ModelKind::Npc,
            },
            _ => return Err(anyhow::format_err!("unknown character code")),
        };

        Ok(info)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ModelKind {
    Player,
    Npc,
    Unknown,
}

pub fn model_info(input: &str) -> IResult<&str, ModelInfo> {
    map_res(raw_model_info, ModelInfo::try_from).parse(input)
}

pub fn model_info_with_raw(input: &str) -> IResult<&str, (u16, ModelInfo)> {
    map_res(raw_model_info, |id| {
        ModelInfo::try_from(id).map(|info| (id, info))
    })
    .parse(input)
}

pub fn raw_model_info(input: &str) -> IResult<&str, u16> {
    n_digit_id::<u16>(4).parse(input)
}
