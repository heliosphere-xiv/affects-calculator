use std::str::FromStr;

use nom::{
    Parser,
    branch::alt,
    bytes::complete::{tag, take, take_till, take_until},
    character::complete::{digit1, one_of},
    combinator::{map, map_res, opt},
    sequence::{delimited, preceded, separated_pair, terminated},
};

use crate::parser::{
    EquipSlot, GamePath, IResult, equip_slot,
    model_info::{ModelInfo, model_info, model_info_with_raw},
    n_digit_id, path_id, simple_part_enum,
    skeleton_slot::{SkeletonSlot, skeleton_slot},
};

#[derive(Debug, PartialEq, Eq)]
pub enum CharacterPath<'a> {
    Mdl {
        primary_id: u16,
        model_info: ModelInfo,
        body_type: BodyType,
        slot: Option<BodyTypeSlot>,
    },
    Mtrl {
        primary_id: u16,
        variant_id: Option<u16>,
        model_info: ModelInfo,
        body_type: BodyType,
        slot: Option<BodyTypeSlot>,
    },
    Tex {
        primary_id: u16,
        variant_id: Option<u8>,
        model_info: ModelInfo,
        body_type: BodyType,
        slot: Option<BodyTypeSlot>,
    },
    Catchlight(&'a str),
    Skin(&'a str),
    Decal {
        kind: DecalType,
        primary_id: u64,
    },
    Skeleton {
        primary_id: u16,
        model_info: ModelInfo,
        slot: SkeletonSlot,
    },
    Tmb(&'a str),
    Pap(&'a str),
    Atch(ModelInfo),
}

enum_str! {
    pub enum BodyType {
        Body => "body",
        Ear => "zear",
        Face => "face",
        Hair => "hair",
        Tail => "tail",
    }

    pub enum BodyTypeSlot {
        Ear => "zer",
        Face => "fac",
        Hair => "hir",
        Tail => "til",
        Iris => "iri",
        Accessory => "acc",
        Etc => "etc",

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

    pub enum DecalType {
        Face => "face",
        Equip => "equip",
    }
}

impl BodyType {
    pub fn abbreviation(&self) -> &str {
        match self {
            Self::Body => "b",
            Self::Ear => "z",
            Self::Face => "f",
            Self::Hair => "h",
            Self::Tail => "t",
        }
    }
}

// util

fn file_repeat(
    primary_id: u16,
    raw_model_info: u16,
    body_type: BodyType,
) -> impl Fn(&str) -> IResult<&str, (&str, &str, &str)> {
    move |input: &str| {
        (
            tag(&*format!("c{raw_model_info:<04}")),
            tag(body_type.abbreviation()),
            tag(&*format!("{primary_id:<04}")),
        )
            .parse(input)
    }
}

fn body_type_slot(input: &str) -> IResult<&str, BodyTypeSlot> {
    map_res(take(3_usize), BodyTypeSlot::from_str).parse(input)
}

// chara/human

pub(crate) fn chara_character_path(input: &str) -> IResult<&str, GamePath> {
    alt((chara_character_path_simple, chara_path_complex)).parse(input)
}

fn chara_character_path_simple(input: &str) -> IResult<&str, GamePath> {
    let (left, (model_info, body_type)) = (
        delimited(
            tag("human/"),
            preceded(tag("c"), model_info_with_raw),
            tag("/"),
        ),
        delimited(tag("obj/"), simple_part_enum::<BodyType>, tag("/")),
    )
        .parse(input)?;

    let (left, primary_id) = terminated(path_id(body_type.abbreviation()), tag("/")).parse(left)?;

    map(
        alt((
            mdl_path(primary_id, model_info, body_type),
            mtrl_path(primary_id, model_info, body_type),
            tex_path(primary_id, model_info, body_type),
        )),
        GamePath::Character,
    )
    .parse(left)
}

fn chara_path_complex(input: &str) -> IResult<&str, GamePath> {
    map(
        alt((
            catchlight_path,
            skin_path,
            decal_path,
            skeleton_path,
            tmb_path,
            pap_path,
            atch_path,
        )),
        GamePath::Character,
    )
    .parse(input)
}

// chara/human/.../model

fn mdl_path(
    primary_id: u16,
    model_info: (u16, ModelInfo),
    body_type: BodyType,
) -> impl Fn(&str) -> IResult<&str, CharacterPath> {
    move |input: &str| {
        map(
            delimited(
                tag("model/"),
                (
                    file_repeat(primary_id, model_info.0, body_type),
                    opt(preceded(tag("_"), body_type_slot)),
                ),
                tag(".mdl"),
            ),
            |(_repeat, slot)| CharacterPath::Mdl {
                primary_id,
                model_info: model_info.1,
                body_type,
                slot,
            },
        )
        .parse(input)
    }
}

// chara/human/.../material

fn mtrl_path(
    primary_id: u16,
    model_info: (u16, ModelInfo),
    body_type: BodyType,
) -> impl Fn(&str) -> IResult<&str, CharacterPath> {
    move |input: &str| {
        map(
            (
                preceded(tag("material/"), opt(terminated(path_id("v"), tag("/")))),
                preceded(tag("mt_"), file_repeat(primary_id, model_info.0, body_type)),
                terminated(
                    opt(preceded(tag("_"), body_type_slot)),
                    (take_till(|c| c == '.'), tag(".mtrl")),
                ),
            ),
            |(variant_id, _repeat, slot)| CharacterPath::Mtrl {
                primary_id,
                variant_id,
                model_info: model_info.1,
                body_type,
                slot,
            },
        )
        .parse(input)
    }
}

// chara/human/.../texture

fn tex_path(
    primary_id: u16,
    model_info: (u16, ModelInfo),
    body_type: BodyType,
) -> impl Fn(&str) -> IResult<&str, CharacterPath> {
    move |input: &str| {
        map(
            (
                preceded(
                    (tag("texture/"), opt(tag("--"))),
                    opt(delimited(tag("v"), n_digit_id::<u8>(2), tag("_"))),
                ),
                file_repeat(primary_id, model_info.0, body_type),
                terminated(
                    opt(preceded(tag("_"), body_type_slot)),
                    (take_till(|c| c == '.'), tag(".tex")),
                ),
            ),
            |(variant_id, _repeat, slot)| CharacterPath::Tex {
                primary_id,
                variant_id,
                model_info: model_info.1,
                body_type,
                slot,
            },
        )
        .parse(input)
    }
}

// chara/common/texture/catchlight

fn catchlight_path(input: &str) -> IResult<&str, CharacterPath> {
    map(
        delimited(
            tag("common/texture/catchlight"),
            take_until(".tex"),
            tag(".tex"),
        ),
        CharacterPath::Catchlight,
    )
    .parse(input)
}

// chara/common/texture/skin

fn skin_path(input: &str) -> IResult<&str, CharacterPath> {
    map(
        delimited(tag("common/texture/skin"), take_until(".tex"), tag(".tex")),
        CharacterPath::Skin,
    )
    .parse(input)
}

// chara/common/texture/decal

fn decal_path(input: &str) -> IResult<&str, CharacterPath> {
    map(
        (
            delimited(
                tag("common/texture/decal_"),
                simple_part_enum::<DecalType>,
                tag("/"),
            ),
            delimited(
                (opt(one_of("-_")), tag("decal_")),
                map_res(digit1, |id: &str| id.parse::<u64>()),
                tag(".tex"),
            ),
        ),
        |(kind, primary_id)| CharacterPath::Decal { kind, primary_id },
    )
    .parse(input)
}

// chara/human/.../skeleton

fn skeleton_path(input: &str) -> IResult<&str, CharacterPath> {
    let (left, (model_info, slot)) = (
        delimited(tag("human/c"), model_info_with_raw, tag("/")),
        delimited(tag("skeleton/"), skeleton_slot, tag("/")),
    )
        .parse(input)?;
    let (left, primary_id) = terminated(path_id(slot.abbreviation()), tag("/")).parse(left)?;

    map(
        delimited(
            alt((tag("eid_"), tag("skl_"), tag("phy_"), tag("kdi_"))),
            (
                tag(&*format!("c{:<04}", model_info.0)),
                tag(slot.abbreviation()),
                tag(&*format!("{primary_id:<04}")),
            ),
            alt((
                tag(".eid"),
                tag(".sklb"),
                tag(".phyb"),
                tag(".skp"),
                tag(".kdb"),
            )),
        ),
        |_repeat| CharacterPath::Skeleton {
            primary_id,
            model_info: model_info.1,
            slot,
        },
    )
    .parse(left)
}

// chara/action/....tmb

fn tmb_path(input: &str) -> IResult<&str, CharacterPath> {
    map(
        delimited(tag("action/"), take_until(".tmb"), tag(".tmb")),
        CharacterPath::Tmb,
    )
    .parse(input)
}

// chara/human/.../animation

fn pap_path(input: &str) -> IResult<&str, CharacterPath> {
    map(
        (
            delimited(tag("human/"), path_id("c"), tag("/")),
            delimited(tag("animation/"), path_id("a"), tag("/")),
            delimited(
                opt((tag("bt_"), take_till(|c| c == '/'), tag("/"))),
                take_until(".pap"),
                tag(".pap"),
            ),
        ),
        // old way
        // delimited(
        //     tag("human/c0101/animation/a0001/"),
        //     take_until(".pap"),
        //     tag(".pap"),
        // ),
        |(_model_id, _primary_id, anim_key)| CharacterPath::Pap(anim_key),
    )
    .parse(input)
}

// chara/xls/attachOffset

fn atch_path(input: &str) -> IResult<&str, CharacterPath> {
    map(
        delimited(tag("xls/attachOffset/c"), model_info, tag(".atch")),
        CharacterPath::Atch,
    )
    .parse(input)
}

#[cfg(test)]
mod test {
    use crate::parser::{
        EquipSlot, GamePath,
        model_info::{ModelInfo, ModelKind},
        path::chara::{
            CharacterPath,
            character::{BodyType, BodyTypeSlot, DecalType},
        },
        race_gender::{Gender, Race},
        skeleton_slot::SkeletonSlot,
        test::test_path,
    };

    #[test]
    fn mdl() {
        const PATH: &str = "chara/human/c1701/obj/hair/h0173/model/c1701h0173_hir.mdl";
        test_path(
            PATH,
            GamePath::Character(CharacterPath::Mdl {
                primary_id: 173,
                model_info: ModelInfo {
                    race: Some(Race::Viera),
                    gender: Gender::Male,
                    kind: ModelKind::Adult,
                },
                body_type: BodyType::Hair,
                slot: Some(BodyTypeSlot::Hair),
            }),
        );
    }

    fn mdl_2() {
        const PATH: &str = "chara/human/c1001/obj/hair/h0102/model/c1001h0102.mdl";
        test_path(
            PATH,
            GamePath::Character(CharacterPath::Mdl {
                primary_id: 173,
                model_info: ModelInfo {
                    race: Some(Race::Roegadyn),
                    gender: Gender::Female,
                    kind: ModelKind::Adult,
                },
                body_type: BodyType::Hair,
                slot: None,
            }),
        );
    }

    #[test]
    fn mtrl_simple() {
        const PATH: &str = "chara/human/c1701/obj/zear/z0003/material/mt_c1701z0003_a.mtrl";

        test_path(
            PATH,
            GamePath::Character(CharacterPath::Mtrl {
                primary_id: 3,
                variant_id: None,
                model_info: ModelInfo {
                    race: Some(Race::Viera),
                    gender: Gender::Male,
                    kind: ModelKind::Adult,
                },
                body_type: BodyType::Ear,
                slot: None,
            }),
        );
    }

    #[test]
    fn mtrl_slot_no_variant() {
        const PATH: &str = "chara/human/c0501/obj/face/f0103/material/mt_c0501f0103_etc_c.mtrl";

        test_path(
            PATH,
            GamePath::Character(CharacterPath::Mtrl {
                primary_id: 103,
                variant_id: None,
                model_info: ModelInfo {
                    race: Some(Race::Elezen),
                    gender: Gender::Male,
                    kind: ModelKind::Adult,
                },
                body_type: BodyType::Face,
                slot: Some(BodyTypeSlot::Etc),
            }),
        );
    }

    #[test]
    fn mtrl_slot_variant() {
        const PATH: &str =
            "chara/human/c0301/obj/hair/h0053/material/v0001/mt_c0301h0053_acc_b.mtrl";

        test_path(
            PATH,
            GamePath::Character(CharacterPath::Mtrl {
                primary_id: 53,
                variant_id: Some(1),
                model_info: ModelInfo {
                    race: Some(Race::Highlander),
                    gender: Gender::Male,
                    kind: ModelKind::Adult,
                },
                body_type: BodyType::Hair,
                slot: Some(BodyTypeSlot::Accessory),
            }),
        );
    }

    #[test]
    fn tex_simple() {
        const PATH: &str = "chara/human/c0101/obj/hair/h0175/texture/c0101h0175_hir_norm.tex";

        test_path(
            PATH,
            GamePath::Character(CharacterPath::Tex {
                primary_id: 175,
                variant_id: None,
                model_info: ModelInfo {
                    race: Some(Race::Midlander),
                    gender: Gender::Male,
                    kind: ModelKind::Adult,
                },
                body_type: BodyType::Hair,
                slot: Some(BodyTypeSlot::Hair),
            }),
        );
    }

    #[test]
    fn tex_minus() {
        const PATH: &str = "chara/human/c1801/obj/hair/h0110/texture/--c1801h0110_hir_n.tex";

        test_path(
            PATH,
            GamePath::Character(CharacterPath::Tex {
                primary_id: 110,
                variant_id: None,
                model_info: ModelInfo {
                    race: Some(Race::Viera),
                    gender: Gender::Female,
                    kind: ModelKind::Adult,
                },
                body_type: BodyType::Hair,
                slot: Some(BodyTypeSlot::Hair),
            }),
        );
    }

    #[test]
    fn tex_variant() {
        const PATH: &str = "chara/human/c1501/obj/body/b0001/texture/--v02_c1501b0001_f_s.tex";

        test_path(
            PATH,
            GamePath::Character(CharacterPath::Tex {
                primary_id: 1,
                variant_id: Some(2),
                model_info: ModelInfo {
                    race: Some(Race::Hrothgar),
                    gender: Gender::Male,
                    kind: ModelKind::Adult,
                },
                body_type: BodyType::Body,
                slot: None,
            }),
        );
    }

    #[test]
    fn catchlight() {
        const PATH: &str = "chara/common/texture/catchlight_2.tex";

        test_path(PATH, GamePath::Character(CharacterPath::Catchlight("_2")));
    }

    #[test]
    fn skin() {
        const PATH: &str = "chara/common/texture/skin_mask.tex";

        test_path(PATH, GamePath::Character(CharacterPath::Skin("_mask")));
    }

    #[test]
    fn decal_1() {
        const PATH: &str = "chara/common/texture/decal_face/_decal_54.tex";

        test_path(
            PATH,
            GamePath::Character(CharacterPath::Decal {
                kind: DecalType::Face,
                primary_id: 54,
            }),
        );
    }

    #[test]
    fn decal_2() {
        const PATH: &str = "chara/common/texture/decal_equip/-decal_110.tex";

        test_path(
            PATH,
            GamePath::Character(CharacterPath::Decal {
                kind: DecalType::Equip,
                primary_id: 110,
            }),
        );
    }

    #[test]
    fn skeleton() {
        const PATHS: &[&str] = &[
            "chara/human/c1601/skeleton/met/m0875/eid_c1601m0875.eid",
            "chara/human/c1601/skeleton/met/m0875/phy_c1601m0875.phyb",
            "chara/human/c1601/skeleton/met/m0875/skl_c1601m0875.skp",
            "chara/human/c1601/skeleton/met/m0875/skl_c1601m0875.sklb",
        ];

        for path in PATHS {
            test_path(
                path,
                GamePath::Character(CharacterPath::Skeleton {
                    primary_id: 875,
                    model_info: ModelInfo {
                        race: Some(Race::Hrothgar),
                        gender: Gender::Female,
                        kind: ModelKind::Adult,
                    },
                    slot: SkeletonSlot::Head,
                }),
            );
        }
    }

    #[test]
    fn tmb() {
        const PATH: &str = "chara/action/magic/2ff_sage/mgc024.tmb";

        test_path(
            PATH,
            GamePath::Character(CharacterPath::Tmb("magic/2ff_sage/mgc024")),
        );
    }

    #[test]
    fn pap() {
        const PATH: &str =
            "chara/human/c0101/animation/a0001/bt_common/ability/cnj_white/abl025.pap";

        test_path(
            PATH,
            GamePath::Character(CharacterPath::Pap("ability/cnj_white/abl025")),
        );
    }

    #[test]
    fn atch() {
        const PATH: &str = "chara/xls/attachOffset/c0501.atch";

        test_path(
            PATH,
            GamePath::Character(CharacterPath::Atch(ModelInfo {
                race: Some(Race::Elezen),
                gender: Gender::Male,
                kind: ModelKind::Adult,
            })),
        );
    }
}
