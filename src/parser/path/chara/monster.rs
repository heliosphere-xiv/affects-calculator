use nom::{
    Parser,
    branch::alt,
    bytes::complete::{tag, take_till},
    combinator::{map, map_res},
    sequence::{delimited, terminated},
};

use crate::parser::{GamePath, IResult, check_repeat_id, n_digit_id, path_id};

#[derive(Debug, PartialEq, Eq)]
pub enum MonsterPath {
    Imc {
        primary_id: u16,
        secondary_id: u16,
    },
    Mdl {
        primary_id: u16,
        secondary_id: u16,
    },
    Skeleton {
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
    Avfx {
        primary_id: u16,
        secondary_id: u16,
        effect_id: u16,
    },
}

// util

fn file_repeat(input: &str) -> IResult<&str, (u16, u16)> {
    (path_id("m"), path_id("b")).parse(input)
}

// chara/monster

pub(crate) fn chara_monster_path(input: &str) -> IResult<&str, GamePath> {
    alt((chara_monster_path_normal, chara_monster_path_skeleton)).parse(input)
}

fn chara_monster_path_normal(input: &str) -> IResult<&str, GamePath> {
    let (left, (primary_id, secondary_id)) = (
        delimited(tag("monster/"), path_id("m"), tag("/")),
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
        GamePath::Monster,
    )
    .parse(left)
}

// chara/monster imc

fn imc_path(primary_id: u16, secondary_id: u16) -> impl Fn(&str) -> IResult<&str, MonsterPath> {
    move |input: &str| {
        map_res(terminated(path_id("b"), tag(".imc")), |repeat_id| {
            if repeat_id != secondary_id {
                return Err(anyhow::format_err!("repeat id didn't match"));
            }
            Ok(MonsterPath::Imc {
                primary_id,
                secondary_id,
            })
        })
        .parse(input)
    }
}

// chara/monster/model

fn mdl_path(primary_id: u16, secondary_id: u16) -> impl Fn(&str) -> IResult<&str, MonsterPath> {
    move |input: &str| {
        map_res(
            delimited(tag("model/"), file_repeat, tag(".mdl")),
            |(repeat_primary_id, repeat_secondary_id)| {
                check_repeat_id(primary_id, repeat_primary_id)?;
                check_repeat_id(secondary_id, repeat_secondary_id)?;

                anyhow::Ok(MonsterPath::Mdl {
                    primary_id,
                    secondary_id,
                })
            },
        )
        .parse(input)
    }
}

// chara/monster/material

fn mtrl_path(primary_id: u16, secondary_id: u16) -> impl Fn(&str) -> IResult<&str, MonsterPath> {
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
                anyhow::Ok(MonsterPath::Mtrl {
                    primary_id,
                    secondary_id,
                    variant_id,
                })
            },
        )
        .parse(input)
    }
}

// chara/monster/texture

fn tex_path(primary_id: u16, secondary_id: u16) -> impl Fn(&str) -> IResult<&str, MonsterPath> {
    move |input: &str| {
        map_res(
            (
                delimited(tag("texture/v"), n_digit_id::<u8>(2), tag("_")),
                terminated(file_repeat, (take_till(|c| c == '.'), tag(".tex"))),
            ),
            |(variant_id, (repeat_primary_id, repeat_secondary_id))| {
                check_repeat_id(primary_id, repeat_primary_id)?;
                check_repeat_id(secondary_id, repeat_secondary_id)?;
                anyhow::Ok(MonsterPath::Tex {
                    primary_id,
                    secondary_id,
                    variant_id,
                })
            },
        )
        .parse(input)
    }
}

// chara/monster/.../skeleton

fn chara_monster_path_skeleton(input: &str) -> IResult<&str, GamePath> {
    map_res(
        (
            delimited(tag("monster/"), path_id("m"), tag("/")),
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
            anyhow::Ok(GamePath::Monster(MonsterPath::Skeleton {
                primary_id,
                secondary_id,
            }))
        },
    )
    .parse(input)
}

// chara/monster/....avfx

fn avfx_path(primary_id: u16, secondary_id: u16) -> impl Fn(&str) -> IResult<&str, MonsterPath> {
    move |input: &str| {
        map(
            delimited(tag("vfx/eff/"), path_id("vm"), tag(".avfx")),
            |effect_id| MonsterPath::Avfx {
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
    use crate::parser::{GamePath, path::chara::MonsterPath, test::test_path};

    #[test]
    fn imc() {
        const PATH: &str = "chara/monster/m0133/obj/body/b0002/b0002.imc";
        test_path(
            PATH,
            GamePath::Monster(MonsterPath::Imc {
                primary_id: 133,
                secondary_id: 2,
            }),
        );
    }

    #[test]
    fn mdl() {
        const PATH: &str = "chara/monster/m8074/obj/body/b0001/model/m8074b0001.mdl";
        test_path(
            PATH,
            GamePath::Monster(MonsterPath::Mdl {
                primary_id: 8074,
                secondary_id: 1,
            }),
        );
    }

    #[test]
    fn mtrl() {
        const PATH: &str = "chara/monster/m0119/obj/body/b0008/material/v0002/mt_m0119b0008_a.mtrl";
        test_path(
            PATH,
            GamePath::Monster(MonsterPath::Mtrl {
                primary_id: 119,
                secondary_id: 8,
                variant_id: 2,
            }),
        );
    }

    #[test]
    fn tex() {
        const PATHS: &[&str] = &[
            "chara/monster/m0161/obj/body/b0002/texture/v01_m0161b0002_s.tex",
            "chara/monster/m0161/obj/body/b0002/texture/v01_m0161b0002_b_n.tex",
        ];

        for path in PATHS {
            test_path(
                path,
                GamePath::Monster(MonsterPath::Tex {
                    primary_id: 161,
                    secondary_id: 2,
                    variant_id: 1,
                }),
            );
        }
    }

    #[test]
    fn skeleton() {
        const PATHS: &[&str] = &[
            "chara/monster/m0926/skeleton/base/b0001/eid_m0926b0001.eid",
            "chara/monster/m0926/skeleton/base/b0001/phy_m0926b0001.phyb",
            "chara/monster/m0926/skeleton/base/b0001/skl_m0926b0001.skp",
            "chara/monster/m0926/skeleton/base/b0001/skl_m0926b0001.sklb",
        ];

        for path in PATHS {
            test_path(
                path,
                GamePath::Monster(MonsterPath::Skeleton {
                    primary_id: 926,
                    secondary_id: 1,
                }),
            );
        }
    }

    #[test]
    fn avfx() {
        const PATH: &str = "chara/monster/m0904/obj/body/b0001/vfx/eff/vm0002.avfx";
        test_path(
            PATH,
            GamePath::Monster(MonsterPath::Avfx {
                primary_id: 904,
                secondary_id: 1,
                effect_id: 2,
            }),
        );
    }
}
