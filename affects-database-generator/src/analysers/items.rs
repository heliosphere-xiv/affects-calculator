use std::collections::BTreeMap;

use affects_common::{EquipSlot, ItemKind};

use crate::{
    analysers::GeneratorContext,
    formats::imc::{ImcFile, RawImcFile},
    schema::{EquipSlotCategory, Item, MetadataProvider},
};

pub fn analyse_items(ctx: &mut GeneratorContext) {
    let items = ctx
        .excel
        .sheet(MetadataProvider::<Item>::for_sheet())
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    let equip_slot_categories = ctx
        .excel
        .sheet(MetadataProvider::<EquipSlotCategory>::for_sheet())
        .unwrap()
        .into_iter()
        .map(|esc| (esc.row_id, esc))
        .collect::<BTreeMap<_, _>>();

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
                .get(&(item.equip_slot_category as u32))
                .unwrap()
        };

        match EquipSlot::try_from(esc) {
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
                let imc = ctx
                    .ironworks
                    .file::<RawImcFile>(&imc_path)
                    .ok()
                    .and_then(ImcFile::try_from_raw);
                if let Some(imc) = imc {
                    if let Some(part_idx) = slot.to_imc_part_idx() {
                        let imc_variant = &imc.parts[part_idx].variants[variant_id as usize - 1];
                        variant_id = imc_variant.material_id;
                    }
                }

                let name_idx = ctx.get_name_idx(ItemKind::Gear, &name);
                ctx.affects
                    .equipment
                    .entry(slot)
                    .or_default()
                    .entry(model_id)
                    .or_default()
                    .entry(variant_id)
                    .or_default()
                    .insert((ItemKind::Gear, name_idx));

                let other_ring = match slot {
                    EquipSlot::LFinger => Some(EquipSlot::RFinger),
                    EquipSlot::RFinger => Some(EquipSlot::LFinger),
                    _ => None,
                };

                if let Some(slot) = other_ring {
                    ctx.affects
                        .equipment
                        .entry(slot)
                        .or_default()
                        .entry(model_id)
                        .or_default()
                        .entry(variant_id)
                        .or_default()
                        .insert((ItemKind::Gear, name_idx));
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
                let imc = ctx.ironworks
                    .file::<RawImcFile>(&format!(
                        "chara/weapon/w{model_id:<04}/obj/body/b{weapon_id:<04}/b{weapon_id:<04}.imc"
                    ))
                    .ok()
                    .and_then(ImcFile::try_from_raw);
                if let Some(imc) = imc {
                    let imc_variant = &imc.parts[0].variants[variant_id as usize - 1];
                    variant_id = imc_variant.material_id;
                }

                let name_idx = ctx.get_name_idx(ItemKind::Weapon, &name);
                ctx.affects
                    .weapons
                    .entry(model_id)
                    .or_default()
                    .entry(weapon_id)
                    .or_default()
                    .entry(variant_id)
                    .or_default()
                    .insert((ItemKind::Weapon, name_idx));

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

                let name_idx = ctx.get_name_idx(ItemKind::Weapon, format!("{name}{append}"));
                ctx.affects
                    .weapons
                    .entry(model)
                    .or_default()
                    .entry(weapon)
                    .or_default()
                    .entry(variant)
                    .or_default()
                    .insert((ItemKind::Weapon, name_idx));
            }
        }
    }
}
