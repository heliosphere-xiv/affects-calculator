use nom::{
    Parser,
    branch::alt,
    bytes::complete::{tag, take_till},
    combinator::{map, map_res},
    sequence::{delimited, preceded, separated_pair, terminated},
};

use crate::parser::{
    EquipSlot, GamePath, IResult, check_repeat_id, equip_slot,
    model_info::{ModelInfo, model_info},
    n_digit_id, path_id,
};

#[derive(Debug, PartialEq, Eq)]
pub enum AccessoryPath {
    Imc(u16),
    Mdl {
        primary_id: u16,
        info: ModelInfo,
        slot: EquipSlot,
    },
    Mtrl {
        primary_id: u16,
        variant_id: u16,
        model_info: ModelInfo,
        slot: EquipSlot,
    },
    Tex {
        primary_id: u16,
        variant_id: u8,
        model_info: ModelInfo,
        slot: EquipSlot,
    },
    // NOTE: Avfx do not exist for accessories yet
}

// util

fn file_repeat(input: &str) -> IResult<&str, (ModelInfo, u16)> {
    (preceded(tag("c"), model_info), path_id("a")).parse(input)
}

// chara/accessory

pub(crate) fn chara_accessory_path(input: &str) -> IResult<&str, GamePath> {
    let (left, primary_id) = delimited(tag("accessory/"), path_id("a"), tag("/")).parse(input)?;

    map(
        alt((
            imc_path(primary_id),
            mtrl_path(primary_id),
            mdl_path(primary_id),
            tex_path(primary_id),
            // avfx_path(primary_id),
        )),
        GamePath::Accessory,
    )
    .parse(left)
}

// chara/accessory imc

fn imc_path(primary_id: u16) -> impl Fn(&str) -> IResult<&str, AccessoryPath> {
    move |input: &str| {
        map_res(terminated(path_id("a"), tag(".imc")), |repeat_primary_id| {
            check_repeat_id(primary_id, repeat_primary_id)?;
            anyhow::Ok(AccessoryPath::Imc(primary_id))
        })
        .parse(input)
    }
}

// chara/accessory/model

fn mdl_path(primary_id: u16) -> impl Fn(&str) -> IResult<&str, AccessoryPath> {
    move |input: &str| {
        map_res(
            delimited(
                tag("model/"),
                separated_pair(file_repeat, tag("_"), equip_slot),
                tag(".mdl"),
            ),
            |((info, repeat_primary_id), slot)| {
                check_repeat_id(primary_id, repeat_primary_id)?;
                anyhow::Ok(AccessoryPath::Mdl {
                    primary_id,
                    info,
                    slot,
                })
            },
        )
        .parse(input)
    }
}

// chara/accessory/material

fn mtrl_path(primary_id: u16) -> impl Fn(&str) -> IResult<&str, AccessoryPath> {
    move |input: &str| {
        map_res(
            (
                delimited(tag("material/"), path_id("v"), tag("/")),
                delimited(
                    tag("mt_"),
                    separated_pair(file_repeat, tag("_"), equip_slot),
                    (take_till(|c| c == '.'), tag(".mtrl")),
                ),
            ),
            |(variant_id, ((model_info, repeat_primary_id), slot))| {
                check_repeat_id(primary_id, repeat_primary_id)?;
                anyhow::Ok(AccessoryPath::Mtrl {
                    primary_id,
                    variant_id,
                    model_info,
                    slot,
                })
            },
        )
        .parse(input)
    }
}

// chara/accessory/texture

fn tex_path(primary_id: u16) -> impl Fn(&str) -> IResult<&str, AccessoryPath> {
    move |input: &str| {
        map_res(
            (
                delimited(tag("texture/v"), n_digit_id::<u8>(2), tag("_")),
                terminated(
                    separated_pair(file_repeat, tag("_"), equip_slot),
                    (take_till(|c| c == '.'), tag(".tex")),
                ),
            ),
            |(variant_id, ((model_info, repeat_primary_id), slot))| {
                check_repeat_id(primary_id, repeat_primary_id)?;
                anyhow::Ok(AccessoryPath::Tex {
                    primary_id,
                    model_info,
                    variant_id,
                    slot,
                })
            },
        )
        .parse(input)
    }
}

// chara/accessory/avfx

#[cfg(test)]
mod test {
    use crate::parser::{
        EquipSlot, GamePath,
        model_info::{ModelInfo, ModelKind},
        path::chara::AccessoryPath,
        race_gender::{Gender, Race},
        test::test_path,
    };

    #[test]
    fn imc() {
        const PATH: &str = "chara/accessory/a0041/a0041.imc";
        test_path(PATH, GamePath::Accessory(AccessoryPath::Imc(41)));
    }

    #[test]
    fn mdl() {
        const PATH: &str = "chara/accessory/a0170/model/c0101a0170_ear.mdl";
        test_path(
            PATH,
            GamePath::Accessory(AccessoryPath::Mdl {
                primary_id: 170,
                info: ModelInfo {
                    race: Some(Race::Midlander),
                    gender: Gender::Male,
                    kind: ModelKind::Adult,
                },
                slot: EquipSlot::Ears,
            }),
        );
    }

    #[test]
    fn mdl_2() {
        const PATH: &str = "chara/accessory/a0155/model/c0101a0155_ril.mdl";
        test_path(
            PATH,
            GamePath::Accessory(AccessoryPath::Mdl {
                primary_id: 155,
                info: ModelInfo {
                    race: Some(Race::Midlander),
                    gender: Gender::Male,
                    kind: ModelKind::Adult,
                },
                slot: EquipSlot::LFinger,
            }),
        );
    }

    #[test]
    fn mtrl() {
        const PATH: &str = "chara/accessory/a0170/material/v0001/mt_c0101a0170_ear_a.mtrl";
        test_path(
            PATH,
            GamePath::Accessory(AccessoryPath::Mtrl {
                primary_id: 170,
                model_info: ModelInfo {
                    race: Some(Race::Midlander),
                    gender: Gender::Male,
                    kind: ModelKind::Adult,
                },
                variant_id: 1,
                slot: EquipSlot::Ears,
            }),
        );
    }

    #[test]
    fn tex() {
        const PATH: &str = "chara/accessory/a0112/texture/v02_c0101a0112_wrs_m.tex";

        test_path(
            PATH,
            GamePath::Accessory(AccessoryPath::Tex {
                primary_id: 112,
                model_info: ModelInfo {
                    race: Some(Race::Midlander),
                    gender: Gender::Male,
                    kind: ModelKind::Adult,
                },
                variant_id: 2,
                slot: EquipSlot::Wrists,
            }),
        );
    }
}
