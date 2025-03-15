use std::{
    borrow::Cow,
    collections::{BTreeMap, BTreeSet},
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    sync::Arc,
    time::Instant,
};

use clap::Parser;
use ironworks::{
    Ironworks,
    excel::{Excel, Language},
    sqpack::{Install, SqPack},
};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{
    cli::CliArguments,
    formats::imc::{ImcFile, RawImcFile},
    parser::{
        EquipSlot, GamePath,
        path::chara::{
            AccessoryPath, CharacterPath, DemihumanPath, EquipmentPath, MonsterPath, WeaponPath,
        },
    },
    schema::{
        Action, ActionCastTimeline, ActionTimeline, BNpcBase, BNpcName, Companion, ENpcBase,
        ENpcResident, Emote, EquipSlotCategory, Item, Map, MetadataProvider, ModelChara,
        ModelCharaKind, Mount, NpcEquip, Ornament, PlaceName, TextCommand,
    },
};

#[macro_use]
mod macros;

mod cli;
mod formats;
mod parser;
mod schema;

#[derive(Debug, Default, Serialize, Deserialize)]
struct Affects {
    names: Vec<String>,
    /// slot => model => variant => set of name indices
    equipment: BTreeMap<EquipSlot, BTreeMap<u16, BTreeMap<u8, BTreeSet<(ItemKind, u16)>>>>,
    /// model => secondary => variant => set of name indices
    weapons: BTreeMap<u16, BTreeMap<u16, BTreeMap<u8, BTreeSet<(ItemKind, u16)>>>>,
    /// timeline key => set of name indices (name, command)
    emotes: BTreeMap<String, BTreeSet<(ItemKind, u16, Option<u16>)>>,
    /// model => base => variant => set of name indices
    monsters: BTreeMap<u16, BTreeMap<u8, BTreeMap<u8, BTreeSet<(ItemKind, u16)>>>>,
    /// model => base => variant => set of name indices
    demihumans: BTreeMap<u16, BTreeMap<u8, BTreeMap<u8, BTreeSet<(ItemKind, u16)>>>>,
    /// animation id => set of name indices
    actions: BTreeMap<String, BTreeSet<(ItemKind, u16)>>,
    /// map id => set of name indices
    maps: BTreeMap<String, BTreeSet<(ItemKind, u16)>>,

    vfx: VfxMaps,

    /// name => index
    #[serde(skip)]
    name_map: BTreeMap<String, u16>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct VfxMaps {
    /// model => vfx => (slot, variant)s
    equipment: BTreeMap<u16, BTreeMap<u8, BTreeSet<(EquipSlot, u8)>>>,
    /// model => base => vfx => variants
    monsters: BTreeMap<u16, BTreeMap<u8, BTreeMap<u8, BTreeSet<u8>>>>,
    /// model => base => vfx => variants
    demihumans: BTreeMap<u16, BTreeMap<u8, BTreeMap<u8, BTreeSet<u8>>>>,
    /// model => weapon => vfx => variants
    weapons: BTreeMap<u16, BTreeMap<u8, BTreeMap<u8, BTreeSet<u8>>>>,
}

#[derive(Debug, Deserialize_repr, Serialize_repr, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
#[repr(u8)]
pub enum ItemKind {
    Gear,
    Weapon,
    Emote,
    BattleNpc,
    EventNpc,
    Minion,
    Mount,
    FashionAccessory,
    Customisation,
    Action,
    Map,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct GraphqlContainer<T> {
    data: T,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct BNpcContainer {
    bnpc: Vec<BNpcMapEntry>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct BNpcMapEntry {
    bnpc_base: u32,
    bnpc_name: u32,
}

fn main() {
    let args = CliArguments::parse();

    // handle bnpcs
    let bnpcs = if args.bnpc_path == "download" {
        println!("downloading bnpc data...");
        let bnpcs = ureq::post("https://gubal.ffxivteamcraft.com/graphql")
            .send_json(serde_json::json!({
                "query": "query { bnpc { bnpcBase, bnpcName } }",
            }))
            .unwrap()
            .body_mut()
            .read_json::<GraphqlContainer<BNpcContainer>>()
            .unwrap()
            .data;

        let mut bnpcs_file = BufWriter::new(File::create("bnpcs.json").unwrap());
        serde_json::to_writer_pretty(&mut bnpcs_file, &bnpcs).unwrap();

        bnpcs
    } else {
        let bnpcs_file = BufReader::new(File::open(&args.bnpc_path).unwrap());
        serde_json::from_reader::<_, BNpcContainer>(bnpcs_file).unwrap()
    };

    // initialise ironworks
    let ironworks =
        Arc::new(Ironworks::new().with_resource(SqPack::new(Install::at(&args.game_path))));
    let excel = Excel::new(Arc::clone(&ironworks)).with_default_language(Language::English);

    // main object
    let mut affects = Affects::default();

    let mut get_name_idx = |name: String, append: Option<&str>| {
        let name = match append {
            Some(append) => format!("{name}{append}"),
            None => name,
        };

        if let Some(&idx) = affects.name_map.get(&name) {
            return idx;
        }

        affects.names.push(name.clone());
        let name_idx = affects.names.len() - 1;
        let name_idx = u16::try_from(name_idx).expect("name idx exceeded 16 bits");

        affects.name_map.insert(name, name_idx);

        name_idx
    };

    let db_time = Instant::now();

    // items
    let items = excel.sheet(MetadataProvider::<Item>::for_sheet()).unwrap();
    let equip_slot_categories = excel
        .sheet(MetadataProvider::<EquipSlotCategory>::for_sheet())
        .unwrap();

    for item in items {
        let name = match item.name.format() {
            Ok(name) if !name.is_empty() => name,
            _ => continue,
        };

        if item.row_id != 17_557 && name.starts_with("Dated ") {
            continue;
        }

        let esc = if item.equip_slot_category == 0 {
            continue;
        } else {
            equip_slot_categories
                .row(item.equip_slot_category as u32)
                .unwrap()
        };

        match EquipSlot::try_from(&esc) {
            Ok(slot) => {
                // equipment
                let model_id = (item.model_main & 0xFFFF) as u16;
                let mut variant_id = ((item.model_main >> 16) & 0xFF) as u8;

                // need to use the imc file to map this variant_id to the
                // correct variant_id used in the game path
                let imc_path = if slot.is_accessory() {
                    format!("chara/accessory/a{model_id:<04}/a{model_id:<04}.imc")
                } else {
                    format!("chara/equipment/e{model_id:<04}/e{model_id:<04}.imc")
                };
                let imc = ironworks
                    .file::<RawImcFile>(&imc_path)
                    .ok()
                    .and_then(|file| ImcFile::try_from(file).ok());
                if let Some(imc) = imc {
                    if let Some(part_idx) = slot.to_imc_part_idx() {
                        let imc_variant = &imc.parts[part_idx].variants[variant_id as usize - 1];
                        variant_id = imc_variant.material_id as u8;
                    }
                }

                affects
                    .equipment
                    .entry(slot)
                    .or_default()
                    .entry(model_id)
                    .or_default()
                    .entry(variant_id)
                    .or_default()
                    .insert((ItemKind::Gear, get_name_idx(name.clone(), None)));

                let other_ring = match slot {
                    EquipSlot::LFinger => Some(EquipSlot::RFinger),
                    EquipSlot::RFinger => Some(EquipSlot::LFinger),
                    _ => None,
                };

                if let Some(slot) = other_ring {
                    affects
                        .equipment
                        .entry(slot)
                        .or_default()
                        .entry(model_id)
                        .or_default()
                        .entry(variant_id)
                        .or_default()
                        .insert((ItemKind::Gear, get_name_idx(name, None)));
                }
            }
            Err(()) => {
                // weapon
                if esc.main_hand != 1 && esc.off_hand != 1 {
                    continue;
                }

                let model_id = (item.model_main & 0xFFFF) as u16;
                let weapon_id = ((item.model_main >> 16) & 0xFFFF) as u16;
                let mut variant_id = ((item.model_main >> 32) & 0xFF) as u8;

                // need to use the imc file to map this variant_id to the
                // correct variant_id used in the game path
                let imc = ironworks
                    .file::<RawImcFile>(&format!(
                        "chara/weapon/w{model_id:<04}/obj/body/b{weapon_id:<04}/b{weapon_id:<04}.imc"
                    ))
                    .ok()
                    .and_then(|file| ImcFile::try_from(file).ok());
                if let Some(imc) = imc {
                    let imc_variant = &imc.parts[0].variants[variant_id as usize - 1];
                    variant_id = imc_variant.material_id as u8;
                }

                affects
                    .weapons
                    .entry(model_id)
                    .or_default()
                    .entry(weapon_id)
                    .or_default()
                    .entry(variant_id)
                    .or_default()
                    .insert((ItemKind::Weapon, get_name_idx(name.clone(), None)));

                // https://github.com/xivapi/ffxiv-datamining/blob/master/csv/ItemUICategory.csv
                let append = match item.item_ui_category {
                    // MNK
                    1 => {
                        // FIXME
                        /*
                        // The game uses a hack for fist weapons where they have a tertiary gauntlet model in the secondary slot,
                        // and compute the actual secondary model from the primary.
                        if (type is FullEquipType.Fists && item.ModelSub < 0x100000000)
                        {
                            tmp[(int)FullEquipType.Hands].Add(new EquipItem(mh.Name + " (Gauntlets)", mh.Id, mh.IconId, (PrimaryId)item.ModelSub, 0,
                                (byte)(item.ModelSub >> 16), FullEquipType.Hands, mh.Flags, mh.Level, mh.JobRestrictions));

                            tmp[(int)FullEquipType.FistsOff].Add(new EquipItem(mh.Name + FullEquipType.FistsOff.OffhandTypeSuffix(), mh.Id,
                                mh.IconId, (PrimaryId)(mh.PrimaryId.Id + 50), mh.SecondaryId, mh.Variant, FullEquipType.FistsOff, mh.Flags, mh.Level,
                                mh.JobRestrictions));
                        }
                         */
                        " (Offhand)"
                    }

                    // BRD
                    4 => " (Quiver)",
                    // NIN
                    84 => " (Offhand)",
                    // MCH
                    88 => " (Aetherotransformer)",
                    // AST
                    89 => " (Orrery)",
                    // SAM
                    96 => " (Sheathe)",
                    // RDM
                    97 => " (Focus)",
                    // DNC
                    107 => " (Offhand)",
                    // VPR
                    110 => " (Offhand)",
                    // PCT
                    111 => " (Palette)",

                    // shields
                    11 => "",

                    // tools
                    12..=33 => "",

                    _ => continue,
                };

                let model = (item.model_sub & 0xFFFF) as u16;
                let weapon = ((item.model_sub >> 16) & 0xFFFF) as u16;
                let variant = ((item.model_sub >> 32) & 0xFF) as u8;

                affects
                    .weapons
                    .entry(model)
                    .or_default()
                    .entry(weapon)
                    .or_default()
                    .entry(variant)
                    .or_default()
                    .insert((ItemKind::Weapon, get_name_idx(name, Some(append))));
            }
        }
    }

    // emotes
    let emotes = excel.sheet(MetadataProvider::<Emote>::for_sheet()).unwrap();
    let action_timelines = excel
        .sheet(MetadataProvider::<ActionTimeline>::for_sheet())
        .unwrap();
    let text_commands = excel
        .sheet(MetadataProvider::<TextCommand>::for_sheet())
        .unwrap();

    for emote in emotes {
        let name = emote.name.format().unwrap();
        if name.is_empty() {
            continue;
        }

        let command = if emote.text_command == 0 {
            None
        } else {
            text_commands
                .row(emote.text_command as u32)
                .ok()
                .and_then(|tc| tc.command.format().ok())
        };

        let key = emote
            .action_timelines
            .iter()
            .find(|&&id| id != 0)
            .and_then(|&id| action_timelines.row(id as u32).ok())
            .and_then(|tl| tl.key.format().ok());

        let key = match key.and_then(|key| key.split('/').last().map(ToString::to_string)) {
            Some(key) => key,
            None => continue,
        };

        affects.emotes.entry(key).or_default().insert((
            ItemKind::Emote,
            get_name_idx(name, None),
            command.map(|command| get_name_idx(command, None)),
        ));
    }

    // bnpcs
    let bnpc_bases = excel
        .sheet(MetadataProvider::<BNpcBase>::for_sheet())
        .unwrap();
    let bnpc_names = excel
        .sheet(MetadataProvider::<BNpcName>::for_sheet())
        .unwrap();
    let model_charas = excel
        .sheet(MetadataProvider::<ModelChara>::for_sheet())
        .unwrap();
    let npc_equips = excel
        .sheet(MetadataProvider::<NpcEquip>::for_sheet())
        .unwrap();

    for bnpc in bnpc_bases {
        let model_chara = match model_charas.row(bnpc.model_chara as u32) {
            Ok(mc) if !mc.kind.is_other() => mc,
            _ => continue,
        };

        let names = bnpcs
            .bnpc
            .iter()
            .filter(|entry| entry.bnpc_base == bnpc.row_id)
            .flat_map(|info| {
                bnpc_names
                    .row(info.bnpc_name)
                    .ok()
                    .and_then(|name| name.singular.format().ok())
                    .filter(|name| !name.is_empty())
            })
            .map(|name| (ItemKind::BattleNpc, get_name_idx(name, None)))
            .collect::<Vec<_>>();

        if names.is_empty() {
            continue;
        }

        let mut variant_id = model_chara.variant;

        // need to use the imc file to map this variant_id to the
        // correct variant_id used in the game path
        if model_chara.kind == ModelCharaKind::Monster {
            let imc = ironworks
                .file::<RawImcFile>(&format!(
                    "chara/monster/m{model:<04}/obj/body/b{base:<04}/b{base:<04}.imc",
                    model = model_chara.model,
                    base = model_chara.base,
                ))
                .ok()
                .and_then(|file| ImcFile::try_from(file).ok());
            if let Some(imc) = imc {
                let imc_variant = &imc.parts[0].variants[variant_id as usize - 1];
                variant_id = imc_variant.material_id as u8;
            }
        } else if model_chara.kind == ModelCharaKind::Demihuman {
            // look up all the equipment models and their imc, then store the
            // vfx in the vfx map

            let npc_equip = match npc_equips.row(bnpc.npc_equip as u32) {
                Ok(ne) => ne,
                Err(_) => continue,
            };

            for (slot_idx, (gear_model_id, gear_variant_id)) in
                npc_equip.gear_models().into_iter().enumerate()
            {
                let imc_path = format!(
                    "chara/demihuman/d{model_chara:<04}/obj/equipment/e{gear_model:<04}/e{gear_model:<04}.imc",
                    model_chara = model_chara.model,
                    gear_model = gear_model_id,
                );

                let imc = match ironworks
                    .file::<RawImcFile>(&imc_path)
                    .ok()
                    .and_then(|file| ImcFile::try_from(file).ok())
                {
                    Some(imc) => imc,
                    None => continue,
                };

                let slot_idx = if slot_idx >= 5 {
                    slot_idx - 5
                } else {
                    slot_idx
                };

                let imc_slot = &imc.parts[slot_idx];
                let imc_variant = if gear_variant_id == 0 {
                    &imc_slot.default_variant
                } else {
                    &imc_slot.variants[gear_variant_id as usize - 1]
                };
                // update the variant id
                variant_id = imc_variant.material_id;

                // store the vfx => materials mapping
                for variant in &imc_slot.variants {
                    if variant.material_id == 0 || variant.vfx_id == 0 {
                        continue;
                    }

                    affects
                        .vfx
                        .demihumans
                        .entry(model_chara.model)
                        .or_default()
                        .entry(gear_model_id as u8)
                        .or_default()
                        .entry(variant.vfx_id)
                        .or_default()
                        .insert(variant.material_id);
                }
            }
        }

        let map = match model_chara.kind {
            ModelCharaKind::Demihuman => &mut affects.demihumans,
            ModelCharaKind::Monster => &mut affects.monsters,
            _ => continue,
        };

        map.entry(model_chara.model)
            .or_default()
            .entry(model_chara.base)
            .or_default()
            .entry(variant_id)
            .or_default()
            .extend(names);
    }

    // enpcs
    let enpc_bases = excel
        .sheet(MetadataProvider::<ENpcBase>::for_sheet())
        .unwrap();
    let enpc_residents = excel
        .sheet(MetadataProvider::<ENpcResident>::for_sheet())
        .unwrap();

    for enpc in enpc_bases {
        let model_chara = match model_charas.row(enpc.model_chara as u32) {
            Ok(mc) if !mc.kind.is_other() => mc,
            _ => continue,
        };

        let resident = match enpc_residents.row(enpc.row_id) {
            Ok(resident) => resident,
            Err(_) => continue,
        };

        let name = match resident.singular.format() {
            Ok(name) if !name.is_empty() => name,
            _ => continue,
            // _ => format!("Event NPC #{}", enpc.row_id),
        };

        match resident.plural.format() {
            Ok(name) if name.is_empty() => continue,
            Err(_) => continue,
            _ => {}
        }

        let mut variant_id = model_chara.variant;

        if model_chara.kind == ModelCharaKind::Monster {
            // need to use the imc file to map this variant_id to the
            // correct variant_id used in the game path
            let imc = ironworks
                .file::<RawImcFile>(&format!(
                    "chara/monster/m{model:<04}/obj/body/b{base:<04}/b{base:<04}.imc",
                    model = model_chara.model,
                    base = model_chara.base,
                ))
                .ok()
                .and_then(|file| ImcFile::try_from(file).ok());
            if let Some(imc) = imc {
                let imc_variant = &imc.parts[0].variants[variant_id as usize - 1];
                variant_id = imc_variant.material_id as u8;
            }
        } else if model_chara.kind == ModelCharaKind::Demihuman {
            // look up all the equipment models and their imc, then store the
            // vfx in the vfx map

            for (slot_idx, (gear_model_id, gear_variant_id)) in
                enpc.gear_models().into_iter().enumerate()
            {
                let imc_path = format!(
                    "chara/demihuman/d{model_chara:<04}/obj/equipment/e{gear_model:<04}/e{gear_model:<04}.imc",
                    model_chara = model_chara.model,
                    gear_model = gear_model_id,
                );

                let imc = match ironworks
                    .file::<RawImcFile>(&imc_path)
                    .ok()
                    .and_then(|file| ImcFile::try_from(file).ok())
                {
                    Some(imc) => imc,
                    None => continue,
                };

                let slot_idx = if slot_idx >= 5 {
                    slot_idx - 5
                } else {
                    slot_idx
                };

                let imc_slot = &imc.parts[slot_idx];
                let imc_variant = if gear_variant_id == 0 {
                    &imc_slot.default_variant
                } else {
                    &imc_slot.variants[gear_variant_id as usize - 1]
                };
                // update the variant id
                variant_id = imc_variant.material_id;

                // store the vfx => materials mapping
                for variant in &imc_slot.variants {
                    if variant.material_id == 0 || variant.vfx_id == 0 {
                        continue;
                    }

                    affects
                        .vfx
                        .demihumans
                        .entry(model_chara.model)
                        .or_default()
                        .entry(gear_model_id as u8)
                        .or_default()
                        .entry(variant.vfx_id)
                        .or_default()
                        .insert(variant.material_id);
                }
            }
        }

        let map = match model_chara.kind {
            ModelCharaKind::Demihuman => &mut affects.demihumans,
            ModelCharaKind::Monster => &mut affects.monsters,
            _ => continue,
        };

        map.entry(model_chara.model)
            .or_default()
            .entry(model_chara.base)
            .or_default()
            .entry(variant_id)
            .or_default()
            .insert((ItemKind::EventNpc, get_name_idx(name, None)));
    }

    // minions
    let minions = excel
        .sheet(MetadataProvider::<Companion>::for_sheet())
        .unwrap();

    for minion in minions {
        let model_chara = match model_charas.row(minion.model as u32) {
            Ok(mc) if !mc.kind.is_other() => mc,
            _ => continue,
        };

        let name = match minion.singular.format() {
            Ok(name) if !name.is_empty() => name,
            _ => continue,
        };

        let mut variant_id = model_chara.variant;

        // need to use the imc file to map this variant_id to the
        // correct variant_id used in the game path
        if model_chara.kind == ModelCharaKind::Monster {
            let imc = ironworks
                .file::<RawImcFile>(&format!(
                    "chara/monster/m{model:<04}/obj/body/b{base:<04}/b{base:<04}.imc",
                    model = model_chara.model,
                    base = model_chara.base,
                ))
                .ok()
                .and_then(|file| ImcFile::try_from(file).ok());
            if let Some(imc) = imc {
                let imc_variant = &imc.parts[0].variants[variant_id as usize - 1];
                variant_id = imc_variant.material_id as u8;
            }
        }

        let map = match model_chara.kind {
            ModelCharaKind::Demihuman => &mut affects.demihumans,
            ModelCharaKind::Monster => &mut affects.monsters,
            _ => continue,
        };

        map.entry(model_chara.model)
            .or_default()
            .entry(model_chara.base)
            .or_default()
            .entry(variant_id)
            .or_default()
            .insert((ItemKind::Minion, get_name_idx(name, None)));
    }

    // mounts
    let mounts = excel.sheet(MetadataProvider::<Mount>::for_sheet()).unwrap();

    for mount in mounts {
        let model_chara = match model_charas.row(mount.model_chara as u32) {
            Ok(mc) if !mc.kind.is_other() => mc,
            _ => continue,
        };

        let name = match mount.singular.format() {
            Ok(name) if !name.is_empty() => name,
            _ => continue,
        };

        let mut variant_id = model_chara.variant;

        // need to use the imc file to map this variant_id to the
        // correct variant_id used in the game path
        if model_chara.kind == ModelCharaKind::Monster {
            let imc = ironworks
                .file::<RawImcFile>(&format!(
                    "chara/monster/m{model:<04}/obj/body/b{base:<04}/b{base:<04}.imc",
                    model = model_chara.model,
                    base = model_chara.base,
                ))
                .ok()
                .and_then(|file| ImcFile::try_from(file).ok());
            if let Some(imc) = imc {
                let imc_variant = &imc.parts[0].variants[variant_id as usize - 1];
                variant_id = imc_variant.material_id as u8;
            }
        }

        let map = match model_chara.kind {
            ModelCharaKind::Demihuman => &mut affects.demihumans,
            ModelCharaKind::Monster => &mut affects.monsters,
            _ => continue,
        };

        map.entry(model_chara.model)
            .or_default()
            .entry(model_chara.base)
            .or_default()
            .entry(variant_id)
            .or_default()
            .insert((ItemKind::Mount, get_name_idx(name, None)));
    }

    // ornaments
    let ornaments = excel
        .sheet(MetadataProvider::<Ornament>::for_sheet())
        .unwrap();

    for ornament in ornaments {
        let model_chara = match model_charas.row(ornament.model as u32) {
            Ok(mc) if !mc.kind.is_other() => mc,
            _ => continue,
        };

        let name = match ornament.singular.format() {
            Ok(name) if !name.is_empty() => name,
            _ => continue,
        };

        let mut variant_id = model_chara.variant;

        // need to use the imc file to map this variant_id to the
        // correct variant_id used in the game path
        if model_chara.kind == ModelCharaKind::Monster {
            let imc = ironworks
                .file::<RawImcFile>(&format!(
                    "chara/monster/m{model:<04}/obj/body/b{base:<04}/b{base:<04}.imc",
                    model = model_chara.model,
                    base = model_chara.base,
                ))
                .ok()
                .and_then(|file| ImcFile::try_from(file).ok());
            if let Some(imc) = imc {
                let imc_variant = &imc.parts[0].variants[variant_id as usize - 1];
                variant_id = imc_variant.material_id as u8;
            }
        }

        let map = match model_chara.kind {
            ModelCharaKind::Demihuman => &mut affects.demihumans,
            ModelCharaKind::Monster => &mut affects.monsters,
            _ => continue,
        };

        map.entry(model_chara.model)
            .or_default()
            .entry(model_chara.base)
            .or_default()
            .entry(variant_id)
            .or_default()
            .insert((ItemKind::FashionAccessory, get_name_idx(name, None)));
    }

    // actions
    let actions = excel
        .sheet(MetadataProvider::<Action>::for_sheet())
        .unwrap();
    let action_cast_timelines = excel
        .sheet(MetadataProvider::<ActionCastTimeline>::for_sheet())
        .unwrap();

    for action in actions {
        let name = match action.name.format() {
            Ok(name) if !name.is_empty() => name,
            _ => continue,
        };

        // if name.starts_with("_rsv_") {
        //     // FIXME: unencrypted available somewhere?
        //     continue;
        // }

        let start_key = action_cast_timelines
            .row(action.animation_start as u32)
            .ok()
            .and_then(|tl| action_timelines.row(tl.action_timeline as u32).ok())
            .and_then(|tl| tl.key.format().ok());
        let end_key = action_timelines
            .row(action.animation_end as u32)
            .ok()
            .and_then(|tl| tl.key.format().ok());
        let hit_key = action_timelines
            .row(action.animation_hit as u32)
            .ok()
            .and_then(|tl| tl.key.format().ok());

        let mut add_action = |key: Option<String>, name: &str| {
            let key = match key {
                Some(key) if !key.is_empty() => key,
                _ => return,
            };

            affects
                .actions
                .entry(key)
                .or_default()
                .insert((ItemKind::Action, get_name_idx(name.to_string(), None)));
        };

        add_action(start_key, &name);
        add_action(end_key, &name);
        add_action(hit_key, &name);
    }

    // maps
    let maps = excel.sheet(MetadataProvider::<Map>::for_sheet()).unwrap();
    let place_names = excel
        .sheet(MetadataProvider::<PlaceName>::for_sheet())
        .unwrap();

    for map in maps {
        let id = match map.id.format() {
            Ok(id) if !id.is_empty() => id,
            _ => continue,
        };

        let place_name_region = place_names
            .row(map.place_name_region as u32)
            .ok()
            .and_then(|pn| pn.name.format().ok())
            .and_then(|name| if name.is_empty() { None } else { Some(name) });
        let place_name = place_names
            .row(map.place_name as u32)
            .ok()
            .and_then(|pn| pn.name.format().ok())
            .and_then(|name| if name.is_empty() { None } else { Some(name) });
        let place_name_sub = place_names
            .row(map.place_name_sub as u32)
            .ok()
            .and_then(|pn| pn.name.format().ok())
            .and_then(|name| if name.is_empty() { None } else { Some(name) });

        let mut name = String::new();
        if let Some(region) = &place_name_region {
            name.push_str(region);
        }

        if let Some(pn) = &place_name {
            if !name.is_empty() {
                name.push_str(" - ");
            }

            name.push_str(pn);
        }

        if let Some(sub) = &place_name_sub {
            if place_name_sub != place_name {
                let empty = name.is_empty();
                if !empty {
                    name.push_str(" (");
                }

                name.push_str(sub);

                if !empty {
                    name.push(')');
                }
            }
        }

        affects
            .maps
            .entry(id)
            .or_default()
            .insert((ItemKind::Map, get_name_idx(name, None)));
    }

    // equipment imcs
    for primaries in affects.equipment.values() {
        for &primary_id in primaries.keys() {
            let imc = match ironworks
                .file::<RawImcFile>(&format!(
                    "chara/equipment/e{primary_id:<04}/e{primary_id:<04}.imc"
                ))
                .ok()
                .and_then(|file| ImcFile::try_from(file).ok())
            {
                Some(imc) => imc,
                None => {
                    continue;
                }
            };

            for (part_idx, part) in imc.parts.iter().enumerate() {
                let slot = match part_idx {
                    0 => EquipSlot::Head,
                    1 => EquipSlot::Body,
                    2 => EquipSlot::Hands,
                    3 => EquipSlot::Legs,
                    4 => EquipSlot::Feet,
                    _ => panic!("too many parts"),
                };

                for (variant_idx, variant) in part.variants.iter().enumerate() {
                    if variant.material_id == 0 || variant.vfx_id == 0 {
                        continue;
                    }

                    affects
                        .vfx
                        .equipment
                        .entry(primary_id)
                        .or_default()
                        .entry(variant.vfx_id)
                        .or_default()
                        .insert((slot, variant.material_id));
                }
            }
        }
    }

    // weapon imcs
    for (&model_id, weapons) in &affects.weapons {
        for &weapon_id in weapons.keys() {
            let imc = match ironworks
                .file::<RawImcFile>(&format!(
                    "chara/weapon/w{model_id:<04}/obj/body/b{weapon_id:<04}/b{weapon_id:<04}.imc"
                ))
                .ok()
                .and_then(|file| ImcFile::try_from(file).ok())
            {
                Some(imc) => imc,
                None => continue,
            };

            for part in imc.parts {
                for variant in part.variants {
                    if variant.material_id == 0 || variant.vfx_id == 0 {
                        continue;
                    }

                    affects
                        .vfx
                        .weapons
                        .entry(model_id)
                        .or_default()
                        .entry(weapon_id as u8)
                        .or_default()
                        .entry(variant.vfx_id)
                        .or_default()
                        .insert(variant.material_id);
                }
            }
        }
    }

    // monster imcs
    for (&model_id, bases) in &affects.monsters {
        for (&base_id, variants) in bases {
            let imc = match ironworks
                .file::<RawImcFile>(&format!(
                    "chara/monster/m{model:<04}/obj/body/b{base:<04}/b{base:<04}.imc",
                    model = model_id,
                    base = base_id,
                ))
                .ok()
                .and_then(|file| ImcFile::try_from(file).ok())
            {
                Some(imc) => imc,
                None => continue,
            };

            for part in imc.parts {
                for variant in part.variants {
                    if variant.material_id == 0 || variant.vfx_id == 0 {
                        continue;
                    }

                    affects
                        .vfx
                        .monsters
                        .entry(model_id)
                        .or_default()
                        .entry(base_id)
                        .or_default()
                        .entry(variant.vfx_id)
                        .or_default()
                        .insert(variant.material_id);
                }
            }
        }
    }

    println!(
        "{} names in {}ms",
        affects.names.len(),
        db_time.elapsed().as_millis(),
    );

    let mut affects_file = BufWriter::new(File::create("affects.json").unwrap());
    let ser_time = Instant::now();
    if args.pretty {
        serde_json::to_writer_pretty(&mut affects_file, &affects).unwrap();
    } else {
        serde_json::to_writer(&mut affects_file, &affects).unwrap();
    }
    println!("serialised json db in {}ms", ser_time.elapsed().as_millis(),);

    let f = File::open("paths.csv").unwrap();
    let reader = BufReader::new(f);

    let mut missing = BufWriter::new(File::create("missing.txt").unwrap());

    let parse_time = Instant::now();

    let mut old_style = BTreeMap::new();

    let mut success = 0;
    let mut total = 0;
    const BAD_EXTS: &[&str] = &[".luab", ".exd", ".exh", ".lgb", ".sgb"];
    for line in reader.lines() {
        let line = line.unwrap();
        if line.starts_with(|c: char| c.is_ascii_uppercase() || c.is_numeric())
            || BAD_EXTS.iter().any(|ext| line.ends_with(ext))
        {
            continue;
        }

        total += 1;
        let res = GamePath::parse(&line);
        if res.is_ok() {
            success += 1;
        } else {
            missing.write_all(line.as_bytes()).unwrap();
            missing.write_all(b"\n").unwrap();
        }

        let convert_names = |names: &BTreeSet<(ItemKind, u16)>| {
            let mut names: Vec<String> = names
                .iter()
                .flat_map(|&(kind, idx)| {
                    affects
                        .names
                        .get(idx as usize)
                        .map(|name| format!("{kind:?}: {name}"))
                })
                .collect();
            names.sort_unstable();
            names
        };

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
            )) => affects
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
            })) => affects
                .monsters
                .get(&primary_id)
                .and_then(|secondaries| secondaries.get(&(secondary_id as u8)))
                .and_then(|variants| variants.get(&(variant_id as u8)))
                .map(convert_names),
            Ok(GamePath::Monster(MonsterPath::Tex {
                primary_id,
                secondary_id,
                variant_id,
            })) => affects
                .monsters
                .get(&primary_id)
                .and_then(|secondaries| secondaries.get(&(secondary_id as u8)))
                .and_then(|variants| variants.get(&variant_id))
                .map(convert_names),
            Ok(GamePath::Monster(MonsterPath::Avfx {
                primary_id,
                secondary_id,
                effect_id,
            })) => affects
                .vfx
                .monsters
                .get(&primary_id)
                .and_then(|secondaries| secondaries.get(&(secondary_id as u8)))
                .and_then(|effects| effects.get(&(effect_id as u8)))
                .and_then(|variant_ids| {
                    affects
                        .monsters
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
            )) => affects
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
            })) => affects
                .weapons
                .get(&primary_id)
                .and_then(|secondaries| secondaries.get(&secondary_id))
                .and_then(|variants| variants.get(&(variant_id as u8)))
                .map(convert_names),
            Ok(GamePath::Weapon(WeaponPath::Tex {
                primary_id,
                secondary_id,
                variant_id,
            })) => affects
                .weapons
                .get(&primary_id)
                .and_then(|secondaries| secondaries.get(&secondary_id))
                .and_then(|variants| variants.get(&variant_id))
                .map(convert_names),
            Ok(GamePath::Weapon(WeaponPath::Avfx {
                primary_id,
                secondary_id,
                effect_id,
            })) => affects
                .vfx
                .weapons
                .get(&primary_id)
                .and_then(|secondaries| secondaries.get(&(secondary_id as u8)))
                .and_then(|effects| effects.get(&(effect_id as u8)))
                .and_then(|variant_ids| {
                    affects
                        .monsters
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
            )) => affects
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
            })) => affects
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
            })) => affects
                .demihumans
                .get(&primary_id)
                .and_then(|secondaries| secondaries.get(&(secondary_id as u8)))
                .and_then(|variants| variants.get(&variant_id))
                .map(convert_names),
            Ok(GamePath::Demihuman(DemihumanPath::Avfx {
                primary_id,
                secondary_id,
                effect_id,
            })) => affects
                .vfx
                .demihumans
                .get(&primary_id)
                .and_then(|secondaries| secondaries.get(&(secondary_id as u8)))
                .and_then(|effects| effects.get(&(effect_id as u8)))
                .and_then(|variant_ids| {
                    affects
                        .demihumans
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
                &affects
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
                    info,
                    slot,
                })
                | GamePath::Accessory(AccessoryPath::Mdl {
                    primary_id,
                    info,
                    slot,
                }),
            ) => affects
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
                    model_info,
                    slot,
                })
                | GamePath::Accessory(AccessoryPath::Mtrl {
                    primary_id,
                    variant_id,
                    model_info,
                    slot,
                }),
            ) => affects
                .equipment
                .get(&slot)
                .and_then(|models| models.get(&primary_id))
                .and_then(|variants| variants.get(&(variant_id as u8)))
                .map(convert_names),
            Ok(
                GamePath::Equipment(EquipmentPath::Tex {
                    primary_id,
                    variant_id,
                    model_info,
                    slot,
                })
                | GamePath::Accessory(AccessoryPath::Tex {
                    primary_id,
                    variant_id,
                    model_info,
                    slot,
                }),
            ) => affects
                .equipment
                .get(&slot)
                .and_then(|models| models.get(&primary_id))
                .and_then(|variants| variants.get(&variant_id))
                .map(convert_names),
            Ok(GamePath::Equipment(EquipmentPath::Avfx {
                primary_id,
                effect_id,
            })) => affects
                .vfx
                .equipment
                .get(&primary_id)
                .and_then(|effects| effects.get(&(effect_id as u8)))
                .map(|variant_ids| {
                    variant_ids
                        .iter()
                        .flat_map(|(slot, variant_id)| {
                            affects
                                .equipment
                                .get(&slot)
                                .and_then(|primaries| primaries.get(&primary_id))
                                .and_then(|variants| variants.get(&variant_id))
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
            })) => Some(vec![format!(
                "Customisation: {model_info} {body_type:?} {slot:?} {primary_id}",
            )]),
            Ok(GamePath::Character(CharacterPath::Mtrl {
                primary_id,
                variant_id,
                model_info,
                body_type,
                slot,
            })) => Some(vec![format!(
                "Customisation: {model_info} {body_type:?} {slot:?} {primary_id}",
            )]),
            Ok(GamePath::Character(CharacterPath::Tex {
                primary_id,
                variant_id,
                model_info,
                body_type,
                slot,
            })) => Some(vec![format!(
                "Customisation: {model_info} {body_type:?} {slot:?} {primary_id}",
            )]),
            Ok(GamePath::Character(CharacterPath::Catchlight(catchlight))) => {
                Some(vec![format!("Customisation: Catchlight {catchlight}")])
            }
            Ok(GamePath::Character(CharacterPath::Skin(skin))) => {
                Some(vec![format!("Customisation: Skin {skin}")])
            }
            Ok(GamePath::Character(CharacterPath::Decal { kind, primary_id })) => {
                Some(vec![format!("Customisation: {kind:?} Decal {primary_id}")])
            }
            Ok(GamePath::Character(CharacterPath::Skeleton {
                primary_id,
                model_info,
                slot,
            })) => Some(vec![format!(
                "Customisation: {model_info} {slot:?} Skeleton {primary_id}"
            )]),
            Ok(GamePath::Character(
                CharacterPath::Tmb(anim_key) | CharacterPath::Pap(anim_key),
            )) => {
                let mut names = anim_key
                    .split('/')
                    .last()
                    .and_then(|key| affects.emotes.get(key))
                    .map(|names| {
                        names
                            .iter()
                            .flat_map(|(kind, name, command)| {
                                let name = affects.names.get(*name as usize);
                                let command =
                                    command.and_then(|command| affects.names.get(command as usize));
                                match (name, command) {
                                    (None, _) => None,
                                    (Some(name), None) => Some(format!("{kind:?}: {name}")),
                                    (Some(name), Some(command)) => {
                                        Some(format!("{kind:?}: {name} ({command})"))
                                    }
                                }
                            })
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default();

                if let Some(actions) = affects.actions.get(anim_key) {
                    for &(kind, idx) in actions {
                        if let Some(name) = affects.names.get(idx as usize) {
                            names.push(format!("{kind:?}: {name}"));
                        }
                    }
                }

                if names.is_empty() {
                    None
                } else {
                    names.sort_unstable();
                    Some(names)
                }
            }
            Ok(GamePath::Character(CharacterPath::Atch(model_info))) => Some(vec![format!(
                "Customisation: {model_info} attachment offsets",
            )]),

            // icon
            Ok(GamePath::Icon {
                group,
                primary_id,
                language,
                hq,
                hires,
            }) => Some(vec![format!("Icon: #{primary_id}")]),

            // map
            Ok(GamePath::Map {
                primary_id,
                variant,
                suffix,
                extra,
            }) => affects
                .maps
                .get(&format!("{primary_id}/{variant:<02}"))
                .map(convert_names),

            // font
            Ok(GamePath::FontFile { family, size }) => {
                Some(vec![format!("Font: {family} {size}px")])
            }

            Ok(GamePath::FontTexture(font)) => Some(vec![format!("Font: {font} (texture)")]),

            Err(_) => {
                let affects = match &line {
                    "chara/common/texture/decal_equip/_stigma.tex" => "Customisation: Archon Mark decal",
                    _ => {
                        let mut iter = line.split('/');
                        let first = iter.next();
                        let last = iter.last();
                        match (first, last) {
                            (_, Some(x)) if x.ends_with(".scd") => "Sound",
                            (Some("bg" | "bgcommon"), _) => "World",
                            (Some("vfx"), _) => "Vfx",
                            (Some("ui"), _) => "Interface",
                            (Some("shader"), _) => "Shader",

                            _ => continue,
                        }
                    }
                };

                Some(vec![affects.to_string()])
            }
        };

        let names = match names {
            Some(names) if !names.is_empty() => names,
            _ => continue,
        };

        old_style.insert(line, names);
        // println!("{line}: {names:?}");
    }

    let mut old_file = BufWriter::new(File::create("affects_old.json").unwrap());
    serde_json::to_writer_pretty(&mut old_file, &old_style).unwrap();

    let pct = success as f32 / total as f32 * 100.0;
    println!(
        "{success}/{total} ({pct}%) in {}ms",
        parse_time.elapsed().as_millis(),
    );
}
