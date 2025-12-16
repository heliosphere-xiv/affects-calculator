use std::{
    borrow::Cow,
    collections::{BTreeMap, BTreeSet},
};

pub use affects_common::{Affects, EquipSlot, ItemKind};
use path_parser::{
    GamePath,
    path::chara::{
        AccessoryPath, BodyType, BodyTypeSlot, CharacterPath, DemihumanPath, EquipmentPath,
        MonsterPath, WeaponPath,
    },
    types::SkeletonSlot,
};

pub trait CalculatesAffects {
    fn calculate_affected(&self, path: &str) -> BTreeMap<ItemKind, BTreeSet<Cow<'_, str>>>;

    fn calculate_affected_cloned(&self, path: &str) -> BTreeMap<ItemKind, BTreeSet<String>> {
        self.calculate_affected(path)
            .into_iter()
            .map(|(kind, names)| {
                (
                    kind,
                    names
                        .into_iter()
                        .map(|name| name.into_owned())
                        .collect::<BTreeSet<_>>(),
                )
            })
            .collect()
    }
}

impl CalculatesAffects for Affects {
    fn calculate_affected(&self, path: &str) -> BTreeMap<ItemKind, BTreeSet<Cow<'_, str>>> {
        let convert_names = |names: &BTreeSet<(ItemKind, u16)>| {
            names
                .iter()
                .flat_map(|&(kind, index)| {
                    self.names
                        .get(index as usize)
                        .map(|name| (kind, Cow::from(name.as_str())))
                })
                .collect::<BTreeSet<_>>()
        };

        let res = GamePath::parse(path);

        let names = match res {
            // monster
            Ok(GamePath::Monster(
                MonsterPath::Imc {
                    primary_id,
                    secondary_id,
                }
                | MonsterPath::Mdl {
                    primary_id,
                    secondary_id,
                }
                | MonsterPath::Skeleton {
                    primary_id,
                    secondary_id,
                },
            )) => self
                .monsters
                .get(&primary_id)
                .and_then(|secondaries| secondaries.get(&(secondary_id as u8)))
                .map(|variants| {
                    variants.values().fold(
                        BTreeSet::new(),
                        |mut acc: BTreeSet<(ItemKind, u16)>, x| {
                            acc.extend(x);
                            acc
                        },
                    )
                })
                .map(|names| convert_names(&names)),
            Ok(GamePath::Monster(MonsterPath::Mtrl {
                primary_id,
                secondary_id,
                variant_id,
            })) => self
                .monsters
                .get(&primary_id)
                .and_then(|secondaries| secondaries.get(&(secondary_id as u8)))
                .and_then(|variants| variants.get(&(variant_id as u8)))
                .map(convert_names),
            Ok(GamePath::Monster(MonsterPath::Tex {
                primary_id,
                secondary_id,
                variant_id,
            })) => self
                .monsters
                .get(&primary_id)
                .and_then(|secondaries| secondaries.get(&(secondary_id as u8)))
                .and_then(|variants| variants.get(&variant_id))
                .map(convert_names),
            Ok(GamePath::Monster(MonsterPath::Avfx {
                primary_id,
                secondary_id,
                effect_id,
            })) => self
                .vfx
                .monsters
                .get(&primary_id)
                .and_then(|secondaries| secondaries.get(&(secondary_id as u8)))
                .and_then(|effects| effects.get(&(effect_id as u8)))
                .and_then(|variant_ids| {
                    self.monsters
                        .get(&primary_id)
                        .and_then(|secondaries| secondaries.get(&(secondary_id as u8)))
                        .map(|variants| {
                            variant_ids.iter().flat_map(|id| variants.get(id)).fold(
                                BTreeSet::new(),
                                |mut acc: BTreeSet<(ItemKind, u16)>, x| {
                                    acc.extend(x);
                                    acc
                                },
                            )
                        })
                        .map(|names| convert_names(&names))
                }),

            // weapon
            Ok(GamePath::Weapon(
                WeaponPath::Imc {
                    primary_id,
                    secondary_id,
                }
                | WeaponPath::Mdl {
                    primary_id,
                    secondary_id,
                }
                | WeaponPath::Skeleton {
                    primary_id,
                    secondary_id,
                },
            )) => self
                .weapons
                .get(&primary_id)
                .and_then(|secondaries| secondaries.get(&secondary_id))
                .map(|variants| {
                    variants.values().fold(
                        BTreeSet::new(),
                        |mut acc: BTreeSet<(ItemKind, u16)>, x| {
                            acc.extend(x);
                            acc
                        },
                    )
                })
                .map(|names| convert_names(&names)),
            Ok(GamePath::Weapon(WeaponPath::Mtrl {
                primary_id,
                secondary_id,
                variant_id,
            })) => self
                .weapons
                .get(&primary_id)
                .and_then(|secondaries| secondaries.get(&secondary_id))
                .and_then(|variants| variants.get(&(variant_id as u8)))
                .map(convert_names),
            Ok(GamePath::Weapon(WeaponPath::Tex {
                primary_id,
                secondary_id,
                variant_id,
            })) => self
                .weapons
                .get(&primary_id)
                .and_then(|secondaries| secondaries.get(&secondary_id))
                .and_then(|variants| variants.get(&variant_id))
                .map(convert_names),
            Ok(GamePath::Weapon(WeaponPath::Avfx {
                primary_id,
                secondary_id,
                effect_id,
            })) => self
                .vfx
                .weapons
                .get(&primary_id)
                .and_then(|secondaries| secondaries.get(&(secondary_id as u8)))
                .and_then(|effects| effects.get(&(effect_id as u8)))
                .and_then(|variant_ids| {
                    self.monsters
                        .get(&primary_id)
                        .and_then(|secondaries| secondaries.get(&(secondary_id as u8)))
                        .map(|variants| {
                            variant_ids.iter().flat_map(|id| variants.get(id)).fold(
                                BTreeSet::new(),
                                |mut acc: BTreeSet<(ItemKind, u16)>, x| {
                                    acc.extend(x);
                                    acc
                                },
                            )
                        })
                        .map(|names| convert_names(&names))
                }),

            // demihuman
            Ok(GamePath::Demihuman(
                DemihumanPath::Imc {
                    primary_id,
                    secondary_id,
                }
                | DemihumanPath::Skeleton {
                    primary_id,
                    secondary_id,
                }
                | DemihumanPath::Mdl {
                    primary_id,
                    secondary_id,
                    ..
                },
            )) => self
                .demihumans
                .get(&primary_id)
                .and_then(|secondaries| secondaries.get(&(secondary_id as u8)))
                .map(|variants| {
                    variants.values().fold(
                        BTreeSet::new(),
                        |mut acc: BTreeSet<(ItemKind, u16)>, x| {
                            acc.extend(x);
                            acc
                        },
                    )
                })
                .map(|names| convert_names(&names)),
            Ok(GamePath::Demihuman(DemihumanPath::Mtrl {
                primary_id,
                secondary_id,
                variant_id,
                ..
            })) => self
                .demihumans
                .get(&primary_id)
                .and_then(|secondaries| secondaries.get(&(secondary_id as u8)))
                .and_then(|variants| variants.get(&(variant_id as u8)))
                .map(convert_names),
            Ok(GamePath::Demihuman(DemihumanPath::Tex {
                primary_id,
                secondary_id,
                variant_id,
                ..
            })) => self
                .demihumans
                .get(&primary_id)
                .and_then(|secondaries| secondaries.get(&(secondary_id as u8)))
                .and_then(|variants| variants.get(&variant_id))
                .map(convert_names),
            Ok(GamePath::Demihuman(DemihumanPath::Avfx {
                primary_id,
                secondary_id,
                effect_id,
            })) => self
                .vfx
                .demihumans
                .get(&primary_id)
                .and_then(|secondaries| secondaries.get(&(secondary_id as u8)))
                .and_then(|effects| effects.get(&(effect_id as u8)))
                .and_then(|variant_ids| {
                    self.demihumans
                        .get(&primary_id)
                        .and_then(|secondaries| secondaries.get(&(secondary_id as u8)))
                        .map(|variants| {
                            variant_ids.iter().flat_map(|id| variants.get(id)).fold(
                                BTreeSet::new(),
                                |mut acc: BTreeSet<(ItemKind, u16)>, x| {
                                    acc.extend(x);
                                    acc
                                },
                            )
                        })
                        .map(|names| convert_names(&names))
                }),

            // equipment/accessory
            Ok(
                GamePath::Equipment(EquipmentPath::Imc(primary_id))
                | GamePath::Accessory(AccessoryPath::Imc(primary_id)),
            ) => Some(convert_names(
                &self
                    .equipment
                    .values()
                    .flat_map(|models| models.get(&primary_id))
                    .flat_map(|variants| variants.values())
                    .flatten()
                    .copied()
                    .collect::<BTreeSet<_>>(),
            )),

            // smallclothes special case
            Ok(GamePath::Equipment(
                EquipmentPath::Mdl { id, info, slot }
                | EquipmentPath::Mtrl {
                    primary_id: id,
                    model_info: info,
                    slot,
                    ..
                }
                | EquipmentPath::Tex {
                    primary_id: id,
                    model_info: info,
                    slot,
                    ..
                },
            )) if id == 0 => {
                let slot = match slot {
                    EquipSlot::Head => "Head",
                    EquipSlot::Hands => "Hands",
                    EquipSlot::Legs => "Legs",
                    EquipSlot::Feet => "Feet",
                    EquipSlot::Body => "Body",
                    EquipSlot::Ears => "Ears",
                    EquipSlot::Neck => "Neck",
                    EquipSlot::RFinger | EquipSlot::LFinger => "Finger",
                    EquipSlot::Wrists => "Wrists",
                };

                single_name(ItemKind::Gear, format!("{info} Smallclothes {slot}"))
            }

            Ok(
                GamePath::Equipment(EquipmentPath::Mdl {
                    id: primary_id,
                    slot,
                    ..
                })
                | GamePath::Accessory(AccessoryPath::Mdl {
                    primary_id, slot, ..
                }),
            ) => self
                .equipment
                .get(&slot)
                .and_then(|models| models.get(&primary_id))
                .map(|variants| {
                    variants.values().fold(
                        BTreeSet::new(),
                        |mut acc: BTreeSet<(ItemKind, u16)>, x| {
                            acc.extend(x);
                            acc
                        },
                    )
                })
                .map(|names| convert_names(&names)),
            Ok(
                GamePath::Equipment(EquipmentPath::Mtrl {
                    primary_id,
                    variant_id,
                    slot,
                    ..
                })
                | GamePath::Accessory(AccessoryPath::Mtrl {
                    primary_id,
                    variant_id,
                    slot,
                    ..
                }),
            ) => self
                .equipment
                .get(&slot)
                .and_then(|models| models.get(&primary_id))
                .and_then(|variants| variants.get(&(variant_id as u8)))
                .map(convert_names),
            Ok(
                GamePath::Equipment(EquipmentPath::Tex {
                    primary_id,
                    variant_id,
                    slot,
                    ..
                })
                | GamePath::Accessory(AccessoryPath::Tex {
                    primary_id,
                    variant_id,
                    slot,
                    ..
                }),
            ) => self
                .equipment
                .get(&slot)
                .and_then(|models| models.get(&primary_id))
                .and_then(|variants| variants.get(&variant_id))
                .map(convert_names),
            Ok(GamePath::Equipment(EquipmentPath::Avfx {
                primary_id,
                effect_id,
            })) => self
                .vfx
                .equipment
                .get(&primary_id)
                .and_then(|effects| effects.get(&(effect_id as u8)))
                .map(|variant_ids| {
                    variant_ids
                        .iter()
                        .flat_map(|(slot, variant_id)| {
                            self.equipment
                                .get(slot)
                                .and_then(|primaries| primaries.get(&primary_id))
                                .and_then(|variants| variants.get(variant_id))
                        })
                        .fold(BTreeSet::new(), |mut acc: BTreeSet<(ItemKind, u16)>, x| {
                            acc.extend(x);
                            acc
                        })
                })
                .map(|names| convert_names(&names)),

            // character
            Ok(GamePath::Character(CharacterPath::Mdl {
                primary_id,
                model_info,
                body_type,
                slot,
            })) => single_name(
                ItemKind::Customisation,
                format!(
                    "{model_info} {} {primary_id}",
                    customisation_type(body_type, slot),
                ),
            ),
            Ok(GamePath::Character(CharacterPath::Mtrl {
                primary_id,
                model_info,
                body_type,
                slot,
                ..
            })) => single_name(
                ItemKind::Customisation,
                format!(
                    "{model_info} {} {primary_id}",
                    customisation_type(body_type, slot),
                ),
            ),
            Ok(GamePath::Character(CharacterPath::Tex {
                primary_id,
                model_info,
                body_type,
                slot,
                ..
            })) => single_name(
                ItemKind::Customisation,
                if slot.is_none() {
                    format!("{model_info} Skin Textures")
                } else {
                    format!(
                        "{model_info} {} {primary_id}",
                        customisation_type(body_type, slot),
                    )
                },
            ),
            Ok(GamePath::Character(CharacterPath::Catchlight(catchlight))) => {
                single_name(ItemKind::Customisation, format!("Catchlight {catchlight}"))
            }
            Ok(GamePath::Character(CharacterPath::Eye { id, kind })) => {
                use const_format::formatcp;

                const EYES: &str = "Eyes";
                const FEMALE: &str = "Female";
                const MALE: &str = "Male";
                const MIDLANDER: &str = "Midlander";
                const HIGHLANDER: &str = "Highlander";
                const DUSKWIGHT: &str = "Duskwight";
                const WILDWOOD: &str = "Wildwood";
                const PLAINSFOLK: &str = "Plainsfolk";
                const DUNESFOLK: &str = "Dunesfolk";
                const SEEKER: &str = "Seeker of the Sun";
                const KEEPER: &str = "Keeper of the Moon";
                const SEA_WOLF: &str = "Sea Wolf";
                const HELLSGUARD: &str = "Hellsguard";
                const RAEN: &str = "Raen";
                const XAELA: &str = "Xaela";
                const HELIONS: &str = "Helions";
                const THE_LOST: &str = "The Lost";
                const RAVA: &str = "Rava";
                const VEENA: &str = "Veena";

                let races_and_tribes = match id {
                    1 => match kind {
                        "norm" => Some(
                            &[
                                formatcp!("{FEMALE} {MIDLANDER} {EYES}"),
                                formatcp!("{MALE} {MIDLANDER} {EYES}"),
                                formatcp!("{FEMALE} {HIGHLANDER} {EYES}"),
                                formatcp!("{MALE} {HIGHLANDER} {EYES}"),
                                formatcp!("{FEMALE} {DUSKWIGHT} {EYES}"),
                                formatcp!("{MALE} {DUSKWIGHT} {EYES}"),
                                formatcp!("{FEMALE} {WILDWOOD} {EYES}"),
                                formatcp!("{MALE} {WILDWOOD} {EYES}"),
                                formatcp!("{FEMALE} {PLAINSFOLK} {EYES}"),
                                formatcp!("{MALE} {PLAINSFOLK} {EYES}"),
                                formatcp!("{FEMALE} {SEEKER} {EYES}"),
                                formatcp!("{MALE} {SEEKER} {EYES}"),
                                formatcp!("{FEMALE} {KEEPER} {EYES}"),
                                formatcp!("{MALE} {KEEPER} {EYES}"),
                                formatcp!("{FEMALE} {SEA_WOLF} {EYES}"),
                                formatcp!("{MALE} {SEA_WOLF} {EYES}"),
                                formatcp!("{FEMALE} {HELLSGUARD} {EYES}"),
                                formatcp!("{MALE} {HELLSGUARD} {EYES}"),
                                formatcp!("{FEMALE} {RAEN} {EYES}"),
                                formatcp!("{FEMALE} {XAELA} {EYES}"),
                                formatcp!("{FEMALE} {RAVA} {EYES}"),
                                formatcp!("{MALE} {RAVA} {EYES}"),
                                formatcp!("{FEMALE} {VEENA} {EYES}"),
                                formatcp!("{MALE} {VEENA} {EYES}"),
                            ][..],
                        ),
                        "mask" => Some(
                            &[
                                formatcp!("{FEMALE} {MIDLANDER} {EYES}"),
                                formatcp!("{MALE} {MIDLANDER} {EYES}"),
                                formatcp!("{FEMALE} {HIGHLANDER} {EYES}"),
                                formatcp!("{MALE} {HIGHLANDER} {EYES}"),
                                formatcp!("{FEMALE} {DUSKWIGHT} {EYES}"),
                                formatcp!("{MALE} {DUSKWIGHT} {EYES}"),
                                formatcp!("{FEMALE} {WILDWOOD} {EYES}"),
                                formatcp!("{MALE} {WILDWOOD} {EYES}"),
                                formatcp!("{FEMALE} {PLAINSFOLK} {EYES}"),
                                formatcp!("{MALE} {PLAINSFOLK} {EYES}"),
                                formatcp!("{FEMALE} {DUNESFOLK} {EYES}"),
                                formatcp!("{MALE} {DUNESFOLK} {EYES}"),
                                formatcp!("{FEMALE} {SEEKER} {EYES}"),
                                formatcp!("{MALE} {SEEKER} {EYES}"),
                                formatcp!("{FEMALE} {KEEPER} {EYES}"),
                                formatcp!("{MALE} {KEEPER} {EYES}"),
                                formatcp!("{FEMALE} {SEA_WOLF} {EYES}"),
                                formatcp!("{MALE} {SEA_WOLF} {EYES}"),
                                formatcp!("{FEMALE} {HELLSGUARD} {EYES}"),
                                formatcp!("{MALE} {HELLSGUARD} {EYES}"),
                                formatcp!("{FEMALE} {RAEN} {EYES}"),
                                formatcp!("{MALE} {RAEN} {EYES}"),
                                formatcp!("{FEMALE} {XAELA} {EYES}"),
                                formatcp!("{MALE} {XAELA} {EYES}"),
                                formatcp!("{FEMALE} {RAVA} {EYES}"),
                                formatcp!("{MALE} {RAVA} {EYES}"),
                                formatcp!("{FEMALE} {VEENA} {EYES}"),
                                formatcp!("{MALE} {VEENA} {EYES}"),
                            ][..],
                        ),
                        "base" => Some(
                            &[
                                formatcp!("{FEMALE} {MIDLANDER} {EYES}"),
                                formatcp!("{FEMALE} {RAVA} {EYES}"),
                                formatcp!("{MALE} {VEENA} {EYES}"),
                            ][..],
                        ),
                        _ => None,
                    },
                    2 if kind == "base" => Some(
                        &[
                            formatcp!("{FEMALE} {SEEKER} {EYES}"),
                            formatcp!("{MALE} {SEEKER} {EYES}"),
                            formatcp!("{FEMALE} {VEENA} {EYES}"),
                        ][..],
                    ),
                    3 if kind == "base" => Some(
                        &[
                            formatcp!("{FEMALE} {KEEPER} {EYES}"),
                            formatcp!("{MALE} {KEEPER} {EYES}"),
                        ][..],
                    ),
                    4 => match kind {
                        "norm" => Some(
                            &[
                                formatcp!("{FEMALE} {DUNESFOLK} {EYES}"),
                                formatcp!("{MALE} {DUNESFOLK} {EYES}"),
                                formatcp!("{MALE} {RAEN} {EYES}"),
                                formatcp!("{MALE} {XAELA} {EYES}"),
                            ][..],
                        ),
                        "base" => Some(
                            &[
                                formatcp!("{FEMALE} {PLAINSFOLK} {EYES}"),
                                formatcp!("{MALE} {PLAINSFOLK} {EYES}"),
                            ][..],
                        ),
                        _ => None,
                    },
                    5 if kind == "base" => Some(
                        &[
                            formatcp!("{FEMALE} {DUNESFOLK} {EYES}"),
                            formatcp!("{MALE} {DUNESFOLK} {EYES}"),
                        ][..],
                    ),
                    6 => match kind {
                        "norm" | "mask" => Some(
                            &[
                                formatcp!("{FEMALE} {HELIONS} {EYES}"),
                                formatcp!("{MALE} {HELIONS} {EYES}"),
                                formatcp!("{FEMALE} {THE_LOST} {EYES}"),
                                formatcp!("{MALE} {THE_LOST} {EYES}"),
                            ][..],
                        ),
                        "base" => Some(
                            &[
                                formatcp!("{FEMALE} {HELIONS} {EYES}"),
                                formatcp!("{MALE} {HELIONS} {EYES}"),
                            ][..],
                        ),
                        _ => None,
                    },
                    7 if kind == "base" => Some(
                        &[
                            formatcp!("{FEMALE} {THE_LOST} {EYES}"),
                            formatcp!("{MALE} {THE_LOST} {EYES}"),
                        ][..],
                    ),
                    9 if kind == "base" => Some(
                        &[
                            formatcp!("{FEMALE} {DUSKWIGHT} {EYES}"),
                            formatcp!("{MALE} {DUSKWIGHT} {EYES}"),
                            formatcp!("{FEMALE} {WILDWOOD} {EYES}"),
                            formatcp!("{MALE} {WILDWOOD} {EYES}"),
                            formatcp!("{MALE} {SEA_WOLF} {EYES}"), // faces 1 & 3
                            formatcp!("{MALE} {HELLSGUARD} {EYES}"), // faces 1 & 3
                        ][..],
                    ),
                    10 if kind == "base" => Some(
                        &[
                            formatcp!("{FEMALE} {SEA_WOLF} {EYES}"),
                            formatcp!("{FEMALE} {HELLSGUARD} {EYES}"),
                            formatcp!("{FEMALE} {RAEN} {EYES}"),
                            formatcp!("{FEMALE} {XAELA} {EYES}"),
                        ][..],
                    ),
                    11 if kind == "base" => Some(
                        &[
                            formatcp!("{MALE} {MIDLANDER} {EYES}"), // faces 1-3 & 5-7
                            formatcp!("{MALE} {HIGHLANDER} {EYES}"),
                            formatcp!("{FEMALE} {HIGHLANDER} {EYES}"), // faces 1-3
                            formatcp!("{MALE} {SEA_WOLF} {EYES}"),     // face 4
                            formatcp!("{MALE} {HELLSGUARD} {EYES}"),   // face 4
                            formatcp!("{MALE} {RAVA} {EYES}"),
                        ][..],
                    ),
                    12 if kind == "base" => Some(
                        &[
                            formatcp!("{MALE} {MIDLANDER} {EYES}"), // face 4
                        ][..],
                    ),
                    13 if kind == "base" => Some(
                        &[
                            formatcp!("{FEMALE} {HIGHLANDER} {EYES}"), // face 4
                        ][..],
                    ),
                    14 if kind == "base" => Some(
                        &[
                            formatcp!("{MALE} {RAEN} {EYES}"),
                            formatcp!("{MALE} {XAELA} {EYES}"),
                        ][..],
                    ),
                    _ => None,
                };

                races_and_tribes.map(|list| {
                    list.iter()
                        .map(|&s| (ItemKind::Customisation, Cow::from(s)))
                        .collect::<BTreeSet<_>>()
                })
            }
            Ok(GamePath::Character(CharacterPath::Skin(skin))) => {
                single_name(ItemKind::Customisation, format!("Skin {skin}"))
            }
            Ok(GamePath::Character(CharacterPath::Decal { kind, primary_id })) => single_name(
                ItemKind::Customisation,
                format!("{kind} Decal {primary_id}"),
            ),
            Ok(GamePath::Character(CharacterPath::Skeleton {
                primary_id,
                model_info,
                slot,
            })) => single_name(
                ItemKind::Customisation,
                if slot == SkeletonSlot::Base {
                    format!("{model_info} Skeleton {primary_id}")
                } else {
                    format!("{model_info} {slot} Skeleton {primary_id}")
                },
            ),
            Ok(GamePath::Character(CharacterPath::Tmb(anim_key))) => {
                let names = check_basic_animations(self, anim_key);
                if names.is_empty() { None } else { Some(names) }
            }
            Ok(GamePath::Character(CharacterPath::Pap {
                model_info,
                category,
                key: anim_key,
                ..
            })) => {
                let mut names = check_basic_animations(self, anim_key);

                let kind = match anim_key {
                    "resident/idle" => Some("idle"),
                    "resident/move_a" => Some("movement"),
                    "resident/move_b" => Some("movement"),
                    "emote/b_pose01_loop" => Some("/cpose"),
                    "emote/b_pose01_start" => Some("/cpose"),
                    _ => None,
                };

                if let Some(kind) = kind {
                    let job = match category {
                        Some("common") => Some(""),
                        Some("2ax_emp") => Some(" WAR"),
                        Some("2bk_emp") => Some(" SCH/SMN"),
                        Some("2bw_emp") => Some(" BRD"),
                        Some("2ff_emp") => Some(" SGE"),
                        Some("2gb_emp") => Some(" GNB"),
                        Some("2gl_emp") => Some(" AST"),
                        Some("2gn_emp") => Some(" MCH"),
                        Some("2km_emp") => Some(" RPR"),
                        Some("2kt_emp") => Some(" SAM"),
                        Some("2rp_emp") => Some(" RDM"),
                        Some("2sp_emp") => Some(" DRG"),
                        Some("2sw_emp") => Some(" DRK"),
                        Some("bld_bld") => Some(" VPR"),
                        Some("brs_plt") => Some(" PCT"),
                        Some("chk_chk") => Some(" DNC"),
                        Some("clw_clw") => Some(" MNK"),
                        Some("dgr_dgr") => Some(" NIN"),
                        Some("rod_emp") => Some(" BLU"),
                        Some("swd_sld") => Some(" PLD"),

                        // weird special cases
                        Some("stf_sld") if kind == "idle" => Some(" WHM"),
                        Some("stf_sld") if kind == "movement" => Some(" WHM/BLM"),

                        Some("2st_emp") => Some(" WHM"),
                        Some("jst_sld") => Some(" BLM"),

                        _ => None,
                    };

                    if let Some(job) = job {
                        names.insert((
                            ItemKind::Animation,
                            Cow::from(format!("{model_info}{job} {kind}")),
                        ));
                    }
                }

                if names.is_empty() { None } else { Some(names) }
            }
            Ok(GamePath::Character(CharacterPath::Atch(model_info))) => single_name(
                ItemKind::Customisation,
                format!("{model_info} attachment offsets"),
            ),

            // icon
            Ok(GamePath::Icon { primary_id, .. }) => {
                single_name(ItemKind::Icon, format!("#{primary_id}"))
            }

            // map
            Ok(GamePath::Map {
                primary_id,
                variant,
                ..
            }) => self
                .maps
                .get(&format!("{primary_id}/{variant:<02}"))
                .map(convert_names),

            // font
            Ok(GamePath::FontFile { family, size }) => {
                single_name(ItemKind::Font, format!("{family} {size}px"))
            }

            Ok(GamePath::FontTexture(font)) => {
                single_name(ItemKind::Font, format!("{font} (texture)"))
            }

            Err(_) => {
                let (kind, affects) = match path {
                    "chara/common/texture/decal_equip/_stigma.tex" => {
                        (ItemKind::Customisation, "Archon Mark")
                    }
                    _ => {
                        let mut iter = path.split('/');
                        let first = iter.next();
                        let last = iter.next_back();
                        match (first, last) {
                            (_, Some(x)) if x.ends_with(".scd") => {
                                (ItemKind::Miscellaneous, "Sound")
                            }
                            (Some("bg" | "bgcommon"), _) => (ItemKind::Miscellaneous, "World"),
                            (Some("vfx"), _) => (ItemKind::Miscellaneous, "VFX"),
                            (Some("ui"), _) => (ItemKind::Miscellaneous, "Interface"),
                            (Some("shader"), _) => (ItemKind::Miscellaneous, "Shader"),

                            _ => return Default::default(),
                        }
                    }
                };

                single_name_ref(kind, affects)
            }
        };

        let mut grouped: BTreeMap<ItemKind, BTreeSet<Cow<str>>> = Default::default();
        let names = names.unwrap_or_default();
        for (kind, name) in names {
            grouped.entry(kind).or_default().insert(name);
        }

        grouped
    }
}

fn single_name<'a>(
    kind: ItemKind,
    name: impl Into<String>,
) -> Option<BTreeSet<(ItemKind, Cow<'a, str>)>> {
    let name = name.into();
    let mut set = BTreeSet::new();
    set.insert((kind, Cow::from(name)));
    Some(set)
}

fn single_name_ref(kind: ItemKind, name: &str) -> Option<BTreeSet<(ItemKind, Cow<'_, str>)>> {
    let mut set = BTreeSet::new();
    set.insert((kind, Cow::from(name)));
    Some(set)
}

fn check_basic_animations<'affects>(
    affects: &'affects Affects,
    anim_key: &str,
) -> BTreeSet<(ItemKind, Cow<'affects, str>)> {
    let mut names = anim_key
        .split('/')
        .next_back()
        .and_then(|key| affects.emotes.get(key))
        .map(|names| {
            names
                .iter()
                .flat_map(|(kind, name)| affects.names.get(*name as usize).map(|name| (kind, name)))
                .map(|(kind, name)| (*kind, Cow::from(name)))
                .collect::<BTreeSet<_>>()
        })
        .unwrap_or_default();

    if let Some(actions) = affects.actions.get(anim_key) {
        for &(kind, idx) in actions {
            if let Some(name) = affects.names.get(idx as usize) {
                names.insert((kind, Cow::from(name)));
            }
        }
    }

    names
}

fn customisation_type(kind: BodyType, slot: Option<BodyTypeSlot>) -> String {
    match (kind, slot) {
        (BodyType::Hair, Some(BodyTypeSlot::Hair))
        | (BodyType::Face, Some(BodyTypeSlot::Face))
        | (BodyType::Ear, Some(BodyTypeSlot::Ear))
        | (BodyType::Body, Some(BodyTypeSlot::Body))
        | (BodyType::Tail, Some(BodyTypeSlot::Tail)) => format!("{kind}"),
        (BodyType::Face, Some(BodyTypeSlot::Iris)) => "Eyes".into(),
        (kind, Some(slot)) if slot != BodyTypeSlot::Etc => format!("{kind} ({slot})"),
        (kind, _) => format!("{kind}"),
    }
}
