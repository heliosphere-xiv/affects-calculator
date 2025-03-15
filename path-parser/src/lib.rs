#[macro_use]
mod macros;

mod error;
pub mod path;
pub mod types;

pub use self::error::Error;

#[cfg(test)]
mod test;

use std::str::FromStr;

use nom::{
    Parser,
    branch::alt,
    bytes::complete::{tag, take, take_till},
    combinator::map_res,
    sequence::preceded,
};

use crate::{
    path::chara::{
        AccessoryPath, CharacterPath, DemihumanPath, EquipmentPath, MonsterPath, WeaponPath,
    },
    types::{EquipSlot, Language, ModelInfo},
};

type Result<'a, T, E = Error<'a>> = std::result::Result<T, E>;

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
    pub fn parse(input: &'a str) -> Result<'a, Self> {
        let (left, path) = game_path(input)
            // FIXME: shouldn't need to clone here
            .map_err(|e| {
                #[cfg(not(test))]
                {
                    Error::Nom(e)
                }

                #[cfg(test)]
                {
                    panic!("do not use GamePath::parse in tests: {e}")
                }
            })?;

        if !left.is_empty() {
            return Err(Error::IncompleteParse);
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

fn check_repeat_id(expected_id: u16, repeat_id: u16) -> Result<'static, ()> {
    if expected_id != repeat_id {
        return Err(Error::MismatchedPathIds {
            expected: expected_id as u32,
            actual: repeat_id as u32,
        });
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
