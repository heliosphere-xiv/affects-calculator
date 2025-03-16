use std::{
    borrow::Cow,
    collections::{BTreeMap, BTreeSet},
};

pub use affects_common::{Affects, EquipSlot, ItemKind};
use path_parser::{
    GamePath,
    path::chara::{
        AccessoryPath, CharacterPath, DemihumanPath, EquipmentPath, MonsterPath, WeaponPath,
    },
};

pub trait CalculatesAffects {
    fn calculate_affected(&self, path: &str) -> BTreeMap<ItemKind, BTreeSet<Cow<str>>>;
}

impl CalculatesAffects for Affects {
    fn calculate_affected(&self, path: &str) -> BTreeMap<ItemKind, BTreeSet<Cow<str>>> {
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
                format!("{model_info} {body_type:?} {slot:?} {primary_id}"),
            ),
            Ok(GamePath::Character(CharacterPath::Mtrl {
                primary_id,
                model_info,
                body_type,
                slot,
                ..
            })) => single_name(
                ItemKind::Customisation,
                format!("{model_info} {body_type:?} {slot:?} {primary_id}",),
            ),
            Ok(GamePath::Character(CharacterPath::Tex {
                primary_id,
                model_info,
                body_type,
                slot,
                ..
            })) => single_name(
                ItemKind::Customisation,
                format!("{model_info} {body_type:?} {slot:?} {primary_id}",),
            ),
            Ok(GamePath::Character(CharacterPath::Catchlight(catchlight))) => {
                single_name(ItemKind::Customisation, format!("Catchlight {catchlight}"))
            }
            Ok(GamePath::Character(CharacterPath::Skin(skin))) => {
                single_name(ItemKind::Customisation, format!("Skin {skin}"))
            }
            Ok(GamePath::Character(CharacterPath::Decal { kind, primary_id })) => single_name(
                ItemKind::Customisation,
                format!("{kind:?} Decal {primary_id}"),
            ),
            Ok(GamePath::Character(CharacterPath::Skeleton {
                primary_id,
                model_info,
                slot,
            })) => single_name(
                ItemKind::Customisation,
                format!("{model_info} {slot:?} Skeleton {primary_id}"),
            ),
            Ok(GamePath::Character(
                CharacterPath::Tmb(anim_key) | CharacterPath::Pap(anim_key),
            )) => {
                let mut names = anim_key
                    .split('/')
                    .last()
                    .and_then(|key| self.emotes.get(key))
                    .map(|names| {
                        names
                            .iter()
                            .flat_map(|(kind, name, command)| {
                                let name = self.names.get(*name as usize);
                                let command =
                                    command.and_then(|command| self.names.get(command as usize));
                                match (name, command) {
                                    (None, _) => None,
                                    (Some(name), None) => Some((*kind, Cow::from(name))),
                                    (Some(name), Some(command)) => {
                                        Some((*kind, Cow::from(format!("{name} ({command})"))))
                                    }
                                }
                            })
                            .collect::<BTreeSet<_>>()
                    })
                    .unwrap_or_default();

                if let Some(actions) = self.actions.get(anim_key) {
                    for &(kind, idx) in actions {
                        if let Some(name) = self.names.get(idx as usize) {
                            names.insert((kind, Cow::from(name)));
                        }
                    }
                }

                if names.is_empty() { None } else { Some(names) }
            }
            Ok(GamePath::Character(CharacterPath::Atch(model_info))) => single_name(
                ItemKind::Customisation,
                format!("{model_info} attachment offsets",),
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
                        (ItemKind::Customisation, "Archon Mark decal")
                    }
                    _ => {
                        let mut iter = path.split('/');
                        let first = iter.next();
                        let last = iter.last();
                        match (first, last) {
                            (_, Some(x)) if x.ends_with(".scd") => {
                                (ItemKind::Miscellaneous, "Sound")
                            }
                            (Some("bg" | "bgcommon"), _) => (ItemKind::Miscellaneous, "World"),
                            (Some("vfx"), _) => (ItemKind::Miscellaneous, "Vfx"),
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

fn single_name_ref(kind: ItemKind, name: &str) -> Option<BTreeSet<(ItemKind, Cow<str>)>> {
    let mut set = BTreeSet::new();
    set.insert((kind, Cow::from(name)));
    Some(set)
}
