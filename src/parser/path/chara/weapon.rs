use nom::{
    Parser,
    branch::alt,
    bytes::complete::{tag, take_till},
    combinator::{map, map_res},
    sequence::{delimited, preceded, terminated},
};

use crate::parser::{GamePath, IResult, check_repeat_id, n_digit_id, path_id};

#[derive(Debug, PartialEq, Eq)]
pub enum WeaponPath {
    Imc {
        primary_id: u16,
        secondary_id: u16,
    },
    Mdl {
        primary_id: u16,
        secondary_id: u16,
    },
    Mtrl {
        primary_id: u16,
        secondary_id: u16,
        variant_id: u16,
    },
    Tex {
        primary_id: u16,
        secondary_id: u16,
        variant_id: u8,
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
    (path_id("w"), path_id("b")).parse(input)
}

// chara/weapon

pub(crate) fn chara_weapon_path(input: &str) -> IResult<&str, GamePath> {
    alt((chara_weapon_path_simple, chara_weapon_path_skeleton)).parse(input)
}

fn chara_weapon_path_simple(input: &str) -> IResult<&str, GamePath> {
    let (left, (primary_id, secondary_id)) = (
        delimited(tag("weapon/"), path_id("w"), tag("/")),
        delimited(tag("obj/body/"), path_id("b"), tag("/")),
    )
        .parse(input)?;

    map(
        alt((
            imc_path(primary_id, secondary_id),
            mdl_path(primary_id, secondary_id),
            mtrl_path(primary_id, secondary_id),
            tex_path(primary_id, secondary_id),
            avfx_path(primary_id, secondary_id),
        )),
        GamePath::Weapon,
    )
    .parse(left)
}

// chara/weapon imc

fn imc_path(primary_id: u16, secondary_id: u16) -> impl Fn(&str) -> IResult<&str, WeaponPath> {
    move |input: &str| {
        map_res(
            terminated(path_id("b"), tag(".imc")),
            |repeat_secondary_id| {
                check_repeat_id(secondary_id, repeat_secondary_id)?;
                anyhow::Ok(WeaponPath::Imc {
                    primary_id,
                    secondary_id,
                })
            },
        )
        .parse(input)
    }
}

// chara/weapon/model

fn mdl_path(primary_id: u16, secondary_id: u16) -> impl Fn(&str) -> IResult<&str, WeaponPath> {
    move |input: &str| {
        map_res(
            delimited(tag("model/"), file_repeat, tag(".mdl")),
            |(repeat_primary_id, repeat_secondary_id)| {
                check_repeat_id(primary_id, repeat_primary_id)?;
                check_repeat_id(secondary_id, repeat_secondary_id)?;
                anyhow::Ok(WeaponPath::Mdl {
                    primary_id,
                    secondary_id,
                })
            },
        )
        .parse(input)
    }
}

// chara/weapon/material

fn mtrl_path(primary_id: u16, secondary_id: u16) -> impl Fn(&str) -> IResult<&str, WeaponPath> {
    move |input: &str| {
        map_res(
            (
                delimited(tag("material/"), path_id("v"), tag("/")),
                delimited(
                    tag("mt_"),
                    file_repeat,
                    (tag("_"), take_till(|c| c == '.'), tag(".mtrl")),
                ),
            ),
            |(variant_id, (repeat_primary_id, repeat_secondary_id))| {
                check_repeat_id(primary_id, repeat_primary_id)?;
                check_repeat_id(secondary_id, repeat_secondary_id)?;
                anyhow::Ok(WeaponPath::Mtrl {
                    primary_id,
                    secondary_id,
                    variant_id,
                })
            },
        )
        .parse(input)
    }
}

// chara/weapon/texture

fn tex_path(primary_id: u16, secondary_id: u16) -> impl Fn(&str) -> IResult<&str, WeaponPath> {
    move |input: &str| {
        map_res(
            (
                preceded(tag("texture/v"), n_digit_id::<u8>(2)),
                delimited(
                    tag("_"),
                    file_repeat,
                    (take_till(|c| c == '.'), tag(".tex")),
                ),
            ),
            |(variant_id, (repeat_primary_id, repeat_secondary_id))| {
                check_repeat_id(primary_id, repeat_primary_id)?;
                check_repeat_id(secondary_id, repeat_secondary_id)?;
                anyhow::Ok(WeaponPath::Tex {
                    primary_id,
                    secondary_id,
                    variant_id,
                })
            },
        )
        .parse(input)
    }
}

// chara/weapon/.../skeleton

fn chara_weapon_path_skeleton(input: &str) -> IResult<&str, GamePath> {
    map_res(
        (
            delimited(tag("weapon/"), path_id("w"), tag("/")),
            delimited(tag("skeleton/base/"), path_id("b"), tag("/")),
            delimited(
                alt((tag("eid_"), tag("skl_"), tag("phy_"))),
                file_repeat,
                alt((tag(".eid"), tag(".sklb"), tag(".phyb"), tag(".skp"))),
            ),
        ),
        |(primary_id, secondary_id, (repeat_primary_id, repeat_secondary_id))| {
            check_repeat_id(primary_id, repeat_primary_id)?;
            check_repeat_id(secondary_id, repeat_secondary_id)?;
            anyhow::Ok(GamePath::Weapon(WeaponPath::Skeleton {
                primary_id,
                secondary_id,
            }))
        },
    )
    .parse(input)
}

// chara/weapon/.../vfx

fn avfx_path(primary_id: u16, secondary_id: u16) -> impl Fn(&str) -> IResult<&str, WeaponPath> {
    move |input: &str| {
        map(
            delimited(tag("vfx/eff/"), path_id("vw"), tag(".avfx")),
            |effect_id| WeaponPath::Avfx {
                primary_id,
                secondary_id,
                effect_id,
            },
        )
        .parse(input)
    }
}

// chara/weapon/w0520/obj/body/b0001/vfx/eff/vw0002.avfx

#[cfg(test)]
mod test {
    use crate::parser::{
        GamePath,
        path::chara::{AccessoryPath, WeaponPath},
        test::test_path,
    };

    #[test]
    fn imc() {
        const PATH: &str = "chara/weapon/w0323/obj/body/b0037/b0037.imc";
        test_path(
            PATH,
            GamePath::Weapon(WeaponPath::Imc {
                primary_id: 323,
                secondary_id: 37,
            }),
        );
    }

    #[test]
    fn mdl() {
        const PATH: &str = "chara/weapon/w2021/obj/body/b0001/model/w2021b0001.mdl";
        test_path(
            PATH,
            GamePath::Weapon(WeaponPath::Mdl {
                primary_id: 2021,
                secondary_id: 1,
            }),
        );
    }

    #[test]
    fn mtrl() {
        const PATH: &str = "chara/weapon/w7051/obj/body/b0001/material/v0017/mt_w7051b0001_a.mtrl";
        test_path(
            PATH,
            GamePath::Weapon(WeaponPath::Mtrl {
                primary_id: 7051,
                secondary_id: 1,
                variant_id: 17,
            }),
        );
    }

    #[test]
    fn tex() {
        const PATHS: &[&str] = &[
            "chara/weapon/w9001/obj/body/b0130/texture/v01_w9001b0130_m.tex",
            "chara/weapon/w9001/obj/body/b0130/texture/v01_w9001b0130_b_m.tex",
        ];

        for path in PATHS {
            test_path(
                path,
                GamePath::Weapon(WeaponPath::Tex {
                    primary_id: 9001,
                    secondary_id: 130,
                    variant_id: 1,
                }),
            );
        }
    }

    #[test]
    fn skeleton() {
        const PATHS: &[&str] = &[
            "chara/weapon/w3001/skeleton/base/b0001/eid_w3001b0001.eid",
            "chara/weapon/w3001/skeleton/base/b0001/phy_w3001b0001.phyb",
            "chara/weapon/w3001/skeleton/base/b0001/skl_w3001b0001.skp",
            "chara/weapon/w3001/skeleton/base/b0001/skl_w3001b0001.sklb",
        ];

        for path in PATHS {
            test_path(
                path,
                GamePath::Weapon(WeaponPath::Skeleton {
                    primary_id: 3001,
                    secondary_id: 1,
                }),
            );
        }
    }

    #[test]
    fn avfx() {
        const PATH: &str = "chara/weapon/w0520/obj/body/b0001/vfx/eff/vw0002.avfx";
        test_path(
            PATH,
            GamePath::Weapon(WeaponPath::Avfx {
                primary_id: 520,
                secondary_id: 1,
                effect_id: 2,
            }),
        );
    }
}
