use nom::{
    Parser,
    branch::alt,
    bytes::complete::{tag, take_till},
    combinator::{map, map_res},
    sequence::{delimited, separated_pair, terminated},
};

use crate::{
    EquipSlot, GamePath, IResult, Result, check_repeat_id, equip_slot, n_digit_id, path_id,
};

#[derive(Debug, PartialEq, Eq)]
pub enum DemihumanPath {
    Imc {
        primary_id: u16,
        secondary_id: u16,
    },
    Mdl {
        primary_id: u16,
        secondary_id: u16,
        slot: EquipSlot,
    },
    Mtrl {
        primary_id: u16,
        secondary_id: u16,
        variant_id: u16,
        slot: EquipSlot,
    },
    Tex {
        primary_id: u16,
        secondary_id: u16,
        variant_id: u8,
        slot: EquipSlot,
    },
    Skeleton {
        primary_id: u16,
        secondary_id: u16,
    },
    Avfx {
        primary_id: u16,
        secondary_id: u16,
        effect_id: u16,
    },
}

// util

fn file_repeat(input: &str) -> IResult<&str, (u16, u16)> {
    (path_id("d"), path_id("e")).parse(input)
}

// chara/demihuman

pub(crate) fn chara_demihuman_path(input: &str) -> IResult<&str, GamePath<'_>> {
    alt((chara_demihuman_path_simple, chara_demihuman_path_skeleton)).parse(input)
}

fn chara_demihuman_path_simple(input: &str) -> IResult<&str, GamePath<'_>> {
    let (left, (primary_id, secondary_id)) = (
        delimited(tag("demihuman/"), path_id("d"), tag("/")),
        delimited(tag("obj/equipment/"), path_id("e"), tag("/")),
    )
        .parse(input)?;

    map(
        alt((
            imc_path(primary_id, secondary_id),
            mtrl_path(primary_id, secondary_id),
            mdl_path(primary_id, secondary_id),
            tex_path(primary_id, secondary_id),
            avfx_path(primary_id, secondary_id),
        )),
        GamePath::Demihuman,
    )
    .parse(left)
}

// chara/demihuman imc

fn imc_path(primary_id: u16, secondary_id: u16) -> impl Fn(&str) -> IResult<&str, DemihumanPath> {
    move |input: &str| {
        map_res(
            terminated(path_id("e"), tag(".imc")),
            |repeat_secondary_id| -> Result<DemihumanPath> {
                check_repeat_id(secondary_id, repeat_secondary_id)?;
                Ok(DemihumanPath::Imc {
                    primary_id,
                    secondary_id,
                })
            },
        )
        .parse(input)
    }
}

// chara/demihuman/model

fn mdl_path(primary_id: u16, secondary_id: u16) -> impl Fn(&str) -> IResult<&str, DemihumanPath> {
    move |input: &str| {
        map_res(
            delimited(
                tag("model/"),
                separated_pair(file_repeat, tag("_"), equip_slot),
                tag(".mdl"),
            ),
            |((repeat_primary_id, repeat_secondary_id), slot)| -> Result<DemihumanPath> {
                check_repeat_id(primary_id, repeat_primary_id)?;
                check_repeat_id(secondary_id, repeat_secondary_id)?;
                Ok(DemihumanPath::Mdl {
                    primary_id,
                    secondary_id,
                    slot,
                })
            },
        )
        .parse(input)
    }
}

// chara/demihuman/material

fn mtrl_path(primary_id: u16, secondary_id: u16) -> impl Fn(&str) -> IResult<&str, DemihumanPath> {
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
            |(variant_id, ((repeat_primary_id, repeat_secondary_id), slot))| -> Result<DemihumanPath> {
                check_repeat_id(primary_id, repeat_primary_id)?;
                check_repeat_id(secondary_id, repeat_secondary_id)?;
                Ok(DemihumanPath::Mtrl {
                    primary_id,
                    secondary_id,
                    variant_id,
                    slot,
                })
            },
        )
        .parse(input)
    }
}

// chara/demihuman/texture

fn tex_path(primary_id: u16, secondary_id: u16) -> impl Fn(&str) -> IResult<&str, DemihumanPath> {
    move |input: &str| {
        map_res(
            (
                delimited(tag("texture/v"), n_digit_id::<u8>(2), tag("_")),
                terminated(
                    separated_pair(file_repeat, tag("_"), equip_slot),
                    (take_till(|c| c == '.'), tag(".tex")),
                ),
            ),
            |(variant_id, ((repeated_primary_id, repeated_secondary_id), slot))| -> Result<DemihumanPath> {
                check_repeat_id(primary_id, repeated_primary_id)?;
                check_repeat_id(secondary_id, repeated_secondary_id)?;
                Ok(DemihumanPath::Tex {
                    primary_id,
                    secondary_id,
                    variant_id,
                    slot,
                })
            },
        )
        .parse(input)
    }
}

// chara/demihuman/.../skeleton

fn chara_demihuman_path_skeleton(input: &str) -> IResult<&str, GamePath<'_>> {
    map_res(
        (
            delimited(tag("demihuman/"), path_id("d"), tag("/")),
            delimited(tag("skeleton/base/"), path_id("b"), tag("/")),
            delimited(
                alt((tag("eid_"), tag("skl_"), tag("phy_"))),
                (path_id("d"), path_id("b")),
                alt((tag(".eid"), tag(".sklb"), tag(".phyb"), tag(".skp"))),
            ),
        ),
        |(primary_id, secondary_id, (repeat_primary_id, repeat_secondary_id))| -> Result<GamePath> {
            check_repeat_id(primary_id, repeat_primary_id)?;
            check_repeat_id(secondary_id, repeat_secondary_id)?;
            Ok(GamePath::Demihuman(DemihumanPath::Skeleton {
                primary_id,
                secondary_id,
            }))
        },
    )
    .parse(input)
}

// chara/demihuman/.../vfx

fn avfx_path(primary_id: u16, secondary_id: u16) -> impl Fn(&str) -> IResult<&str, DemihumanPath> {
    move |input: &str| {
        map(
            delimited(tag("vfx/eff/"), path_id("ve"), tag(".avfx")),
            |effect_id| DemihumanPath::Avfx {
                primary_id,
                secondary_id,
                effect_id,
            },
        )
        .parse(input)
    }
}

#[cfg(test)]
mod test {
    use crate::{EquipSlot, GamePath, path::chara::DemihumanPath, test::test_path};

    #[test]
    fn imc() {
        const PATH: &str = "chara/demihuman/d0001/obj/equipment/e0026/e0026.imc";
        test_path(
            PATH,
            GamePath::Demihuman(DemihumanPath::Imc {
                primary_id: 1,
                secondary_id: 26,
            }),
        );
    }

    #[test]
    fn mdl() {
        const PATH: &str = "chara/demihuman/d0001/obj/equipment/e0027/model/d0001e0027_dwn.mdl";
        test_path(
            PATH,
            GamePath::Demihuman(DemihumanPath::Mdl {
                primary_id: 1,
                secondary_id: 27,
                slot: EquipSlot::Legs,
            }),
        );
    }

    #[test]
    fn mtrl() {
        const PATH: &str =
            "chara/demihuman/d1025/obj/equipment/e0001/material/v0011/mt_d1025e0001_dwn_a.mtrl";
        test_path(
            PATH,
            GamePath::Demihuman(DemihumanPath::Mtrl {
                primary_id: 1025,
                secondary_id: 1,
                variant_id: 11,
                slot: EquipSlot::Legs,
            }),
        );
    }

    #[test]
    fn tex() {
        const PATHS: &[&str] = &[
            "chara/demihuman/d1068/obj/equipment/e0006/texture/v01_d1068e0006_top_b_mask.tex",
            "chara/demihuman/d1068/obj/equipment/e0006/texture/v01_d1068e0006_top_norm.tex",
        ];

        for path in PATHS {
            test_path(
                path,
                GamePath::Demihuman(DemihumanPath::Tex {
                    primary_id: 1068,
                    secondary_id: 6,
                    variant_id: 1,
                    slot: EquipSlot::Body,
                }),
            );
        }
    }

    #[test]
    fn skeleton() {
        const PATHS: &[&str] = &[
            "chara/demihuman/d1026/skeleton/base/b0001/eid_d1026b0001.eid",
            "chara/demihuman/d1026/skeleton/base/b0001/phy_d1026b0001.phyb",
            "chara/demihuman/d1026/skeleton/base/b0001/skl_d1026b0001.skp",
            "chara/demihuman/d1026/skeleton/base/b0001/skl_d1026b0001.sklb",
        ];

        for path in PATHS {
            test_path(
                path,
                GamePath::Demihuman(DemihumanPath::Skeleton {
                    primary_id: 1026,
                    secondary_id: 1,
                }),
            );
        }
    }

    #[test]
    fn avfx() {
        const PATH: &str = "chara/demihuman/d1038/obj/equipment/e0001/vfx/eff/ve0002.avfx";
        test_path(
            PATH,
            GamePath::Demihuman(DemihumanPath::Avfx {
                primary_id: 1038,
                secondary_id: 1,
                effect_id: 2,
            }),
        );
    }
}
