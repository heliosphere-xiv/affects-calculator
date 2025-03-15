use nom::{
    Parser,
    branch::alt,
    bytes::complete::{tag, take_till},
    combinator::{map, map_res},
    sequence::{delimited, preceded, terminated},
};

use crate::parser::{
    EquipSlot, GamePath, IResult, ModelInfo, check_repeat_id, equip_slot, model_info::model_info,
    n_digit_id, path_id,
};

#[derive(Debug, PartialEq, Eq)]
pub enum EquipmentPath {
    Imc(u16),
    Mtrl {
        primary_id: u16,
        variant_id: u16,
        model_info: ModelInfo,
        slot: EquipSlot,
    },
    Mdl {
        id: u16,
        info: ModelInfo,
        slot: EquipSlot,
    },
    Tex {
        primary_id: u16,
        variant_id: u8,
        model_info: ModelInfo,
        slot: EquipSlot,
    },
    Avfx {
        primary_id: u16,
        effect_id: u16,
    },
}

// util

fn file_repeat(input: &str) -> IResult<&str, (ModelInfo, u16)> {
    (preceded(tag("c"), model_info), path_id("e")).parse(input)
}

// chara/equipment

pub(crate) fn chara_equipment_path(input: &str) -> IResult<&str, GamePath> {
    // equipment/e0863/material/v0006/mt_c0101e0863_sho_a.mtrl
    let (left, primary_id) = delimited(tag("equipment/"), path_id("e"), tag("/")).parse(input)?;

    map(
        alt((
            imc_path(primary_id),
            mtrl_path(primary_id),
            mdl_path(primary_id),
            tex_path(primary_id),
            avfx_path(primary_id),
        )),
        GamePath::Equipment,
    )
    .parse(left)
}

// chara/equipment imc

fn imc_path(primary_id: u16) -> impl Fn(&str) -> IResult<&str, EquipmentPath> {
    move |input: &str| {
        map_res(terminated(path_id("e"), tag(".imc")), |repeat_id| {
            check_repeat_id(primary_id, repeat_id)?;
            anyhow::Ok(EquipmentPath::Imc(primary_id))
        })
        .parse(input)
    }
}

// chara/equipment/model

fn mdl_path(primary_id: u16) -> impl Fn(&str) -> IResult<&str, EquipmentPath> {
    move |input: &str| {
        map_res(
            (
                delimited(tag("model/"), file_repeat, tag("_")),
                terminated(equip_slot, tag(".mdl")),
            ),
            |((info, repeat_id), slot)| {
                check_repeat_id(primary_id, repeat_id)?;
                anyhow::Ok(EquipmentPath::Mdl {
                    id: primary_id,
                    info,
                    slot,
                })
            },
        )
        .parse(input)
    }
}

// chara/equipment/material

fn mtrl_path(primary_id: u16) -> impl Fn(&str) -> IResult<&str, EquipmentPath> {
    move |input: &str| {
        map_res(
            (terminated(mtrl_variant, tag("/")), mtrl_simple_file_name),
            |(variant_id, (info, e_id, slot))| {
                check_repeat_id(primary_id, e_id)?;
                anyhow::Ok(EquipmentPath::Mtrl {
                    primary_id,
                    variant_id,
                    model_info: info,
                    slot,
                })
            },
        )
        .parse(input)
    }
}

fn mtrl_variant(input: &str) -> IResult<&str, u16> {
    // material/v0006

    preceded(tag("material/"), path_id("v")).parse(input)
}

fn mtrl_simple_file_name(input: &str) -> IResult<&str, (ModelInfo, u16, EquipSlot)> {
    // mt_c0101e0863_sho_a.mtrl

    let ids = preceded(tag("mt_"), file_repeat);
    let slot = delimited(tag("_"), equip_slot, tag("_"));
    let last_bit = terminated(take_till(|c| c == '.'), tag(".mtrl"));

    map((ids, slot, last_bit), |((info, e_id), slot, _last)| {
        (info, e_id, slot)
    })
    .parse(input)
}

// chara/equipment/texture

fn tex_path(primary_id: u16) -> impl Fn(&str) -> IResult<&str, EquipmentPath> {
    move |input: &str| {
        map_res(
            (
                delimited(tag("texture/v"), n_digit_id::<u8>(2), tag("_")),
                terminated(file_repeat, tag("_")),
                terminated(terminated(equip_slot, take_till(|c| c == '.')), tag(".tex")),
            ),
            |(variant_id, (info, repeat_id), slot)| {
                check_repeat_id(primary_id, repeat_id)?;
                anyhow::Ok(EquipmentPath::Tex {
                    primary_id,
                    variant_id,
                    model_info: info,
                    slot,
                })
            },
        )
        .parse(input)
    }
}

// chara/equipment/vfx

fn avfx_path(primary_id: u16) -> impl Fn(&str) -> IResult<&str, EquipmentPath> {
    move |input: &str| {
        map(
            terminated(path_id("vfx/eff/ve"), tag(".avfx")),
            |variant_id| EquipmentPath::Avfx {
                primary_id,
                effect_id: variant_id,
            },
        )
        .parse(input)
    }
}

#[cfg(test)]
mod test {
    use crate::parser::{
        EquipSlot, GamePath,
        model_info::{ModelInfo, ModelKind},
        path::chara::EquipmentPath,
        race_gender::{Gender, Race},
        test::test_path,
    };

    #[test]
    pub fn mtrl() {
        const PATH: &str = "chara/equipment/e0863/material/v0006/mt_c0101e0863_sho_a.mtrl";
        test_path(
            PATH,
            GamePath::Equipment(EquipmentPath::Mtrl {
                primary_id: 863,
                variant_id: 6,
                model_info: ModelInfo {
                    race: Some(Race::Midlander),
                    gender: Gender::Male,
                    kind: ModelKind::Adult,
                },
                slot: EquipSlot::Feet,
            }),
        );
    }

    #[test]
    pub fn imc() {
        const PATH: &str = "chara/equipment/e6194/e6194.imc";
        test_path(PATH, GamePath::Equipment(EquipmentPath::Imc(6194)));
    }

    #[test]
    pub fn mdl() {
        const PATH: &str = "chara/equipment/e0864/model/c0101e0864_met.mdl";
        test_path(
            PATH,
            GamePath::Equipment(EquipmentPath::Mdl {
                id: 864,
                info: ModelInfo {
                    race: Some(Race::Midlander),
                    gender: Gender::Male,
                    kind: ModelKind::Adult,
                },
                slot: EquipSlot::Head,
            }),
        );
    }

    #[test]
    pub fn tex() {
        const PATH: &str = "chara/equipment/e0862/texture/v01_c0101e0862_top_mask.tex";
        test_path(
            PATH,
            GamePath::Equipment(EquipmentPath::Tex {
                primary_id: 862,
                variant_id: 1,
                model_info: ModelInfo {
                    race: Some(Race::Midlander),
                    gender: Gender::Male,
                    kind: ModelKind::Adult,
                },
                slot: EquipSlot::Body,
            }),
        );
    }

    #[test]
    fn avfx() {
        const PATH: &str = "chara/equipment/e9173/vfx/eff/ve0006.avfx";
        test_path(
            PATH,
            GamePath::Equipment(EquipmentPath::Avfx {
                primary_id: 9173,
                effect_id: 6,
            }),
        );
    }
}
