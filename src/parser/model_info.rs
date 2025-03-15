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
                kind: ModelKind::Adult,
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
                kind: ModelKind::Child,
            },
            0201 => ModelInfo {
                race: Some(Race::Midlander),
                gender: Gender::Female,
                kind: ModelKind::Adult,
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
                kind: ModelKind::Child,
            },
            0301 => ModelInfo {
                race: Some(Race::Highlander),
                gender: Gender::Male,
                kind: ModelKind::Adult,
            },
            0304 => ModelInfo {
                race: Some(Race::Highlander),
                gender: Gender::Male,
                kind: ModelKind::Child,
            },
            0401 => ModelInfo {
                race: Some(Race::Highlander),
                gender: Gender::Female,
                kind: ModelKind::Adult,
            },
            0404 => ModelInfo {
                race: Some(Race::Highlander),
                gender: Gender::Female,
                kind: ModelKind::Child,
            },
            0501 => ModelInfo {
                race: Some(Race::Elezen),
                gender: Gender::Male,
                kind: ModelKind::Adult,
            },
            0504 => ModelInfo {
                race: Some(Race::Elezen),
                gender: Gender::Male,
                kind: ModelKind::Child,
            },
            0601 => ModelInfo {
                race: Some(Race::Elezen),
                gender: Gender::Female,
                kind: ModelKind::Adult,
            },
            0604 => ModelInfo {
                race: Some(Race::Elezen),
                gender: Gender::Female,
                kind: ModelKind::Child,
            },
            0701 => ModelInfo {
                race: Some(Race::Miqote),
                gender: Gender::Male,
                kind: ModelKind::Adult,
            },
            0704 => ModelInfo {
                race: Some(Race::Miqote),
                gender: Gender::Male,
                kind: ModelKind::Child,
            },
            0801 => ModelInfo {
                race: Some(Race::Miqote),
                gender: Gender::Female,
                kind: ModelKind::Adult,
            },
            0804 => ModelInfo {
                race: Some(Race::Miqote),
                gender: Gender::Female,
                kind: ModelKind::Child,
            },
            0901 => ModelInfo {
                race: Some(Race::Roegadyn),
                gender: Gender::Male,
                kind: ModelKind::Adult,
            },
            0904 => ModelInfo {
                race: Some(Race::Roegadyn),
                gender: Gender::Male,
                kind: ModelKind::Child,
            },
            1001 => ModelInfo {
                race: Some(Race::Roegadyn),
                gender: Gender::Female,
                kind: ModelKind::Adult,
            },
            1004 => ModelInfo {
                race: Some(Race::Roegadyn),
                gender: Gender::Female,
                kind: ModelKind::Child,
            },
            1101 => ModelInfo {
                race: Some(Race::Lalafell),
                gender: Gender::Male,
                kind: ModelKind::Adult,
            },
            1104 => ModelInfo {
                race: Some(Race::Lalafell),
                gender: Gender::Male,
                kind: ModelKind::Child,
            },
            1201 => ModelInfo {
                race: Some(Race::Lalafell),
                gender: Gender::Female,
                kind: ModelKind::Adult,
            },
            1204 => ModelInfo {
                race: Some(Race::Lalafell),
                gender: Gender::Female,
                kind: ModelKind::Child,
            },
            1301 => ModelInfo {
                race: Some(Race::AuRa),
                gender: Gender::Male,
                kind: ModelKind::Adult,
            },
            1304 => ModelInfo {
                race: Some(Race::AuRa),
                gender: Gender::Male,
                kind: ModelKind::Child,
            },
            1401 => ModelInfo {
                race: Some(Race::AuRa),
                gender: Gender::Female,
                kind: ModelKind::Adult,
            },
            1404 => ModelInfo {
                race: Some(Race::AuRa),
                gender: Gender::Female,
                kind: ModelKind::Child,
            },
            1501 => ModelInfo {
                race: Some(Race::Hrothgar),
                gender: Gender::Male,
                kind: ModelKind::Adult,
            },
            1504 => ModelInfo {
                race: Some(Race::Hrothgar),
                gender: Gender::Male,
                kind: ModelKind::Child,
            },
            1601 => ModelInfo {
                race: Some(Race::Hrothgar),
                gender: Gender::Female,
                kind: ModelKind::Adult,
            },
            1604 => ModelInfo {
                race: Some(Race::Hrothgar),
                gender: Gender::Female,
                kind: ModelKind::Child,
            },
            1701 => ModelInfo {
                race: Some(Race::Viera),
                gender: Gender::Male,
                kind: ModelKind::Adult,
            },
            1704 => ModelInfo {
                race: Some(Race::Viera),
                gender: Gender::Male,
                kind: ModelKind::Child,
            },
            1801 => ModelInfo {
                race: Some(Race::Viera),
                gender: Gender::Female,
                kind: ModelKind::Adult,
            },
            1804 => ModelInfo {
                race: Some(Race::Viera),
                gender: Gender::Female,
                kind: ModelKind::Child,
            },
            9104 => ModelInfo {
                race: None,
                gender: Gender::Male,
                kind: ModelKind::Child,
            },
            9204 => ModelInfo {
                race: None,
                gender: Gender::Female,
                kind: ModelKind::Child,
            },
            _ => return Err(anyhow::format_err!("unknown character code")),
        };

        Ok(info)
    }
}

impl ModelInfo {
    pub fn to_path_id(self) -> Option<u64> {
        let race = self.race?;
        let id = match (race, self.gender) {
            (Race::Midlander, Gender::Male) => 1,
            (Race::Midlander, Gender::Female) => 2,
            (Race::Highlander, Gender::Male) => 3,
            (Race::Highlander, Gender::Female) => 4,
            (Race::Elezen, Gender::Male) => 5,
            (Race::Elezen, Gender::Female) => 6,
            (Race::Miqote, Gender::Male) => 7,
            (Race::Miqote, Gender::Female) => 8,
            (Race::Roegadyn, Gender::Male) => 9,
            (Race::Roegadyn, Gender::Female) => 10,
            (Race::Lalafell, Gender::Male) => 11,
            (Race::Lalafell, Gender::Female) => 12,
            (Race::AuRa, Gender::Male) => 13,
            (Race::AuRa, Gender::Female) => 14,
            (Race::Hrothgar, Gender::Male) => 15,
            (Race::Hrothgar, Gender::Female) => 16,
            (Race::Viera, Gender::Male) => 17,
            (Race::Viera, Gender::Female) => 18,
        };

        Some(id)
    }
}

impl std::fmt::Display for ModelInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.race, self.gender, self.kind) {
            (Some(race), gender, kind) => write!(f, "{gender} {race}{}", kind.to_pretty_suffix()),
            (None, gender, kind) => write!(f, "{gender} Unknown{}", kind.to_pretty_suffix()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ModelKind {
    Adult,
    Child,
    Unknown,
}

impl ModelKind {
    pub fn to_pretty_suffix(self) -> &'static str {
        match self {
            Self::Adult => "",
            Self::Child => " (Child)",
            Self::Unknown => " (Unknown)",
        }
    }
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
