use std::collections::{BTreeMap, BTreeSet};

use affects_common::ItemKind;

use crate::{
    analysers::GeneratorContext,
    formats::imc::{ImcFile, RawImcFile},
    schema::{
        BNpcBase, BNpcName, Companion, MetadataProvider, ModelChara, ModelCharaKind, NpcEquip,
    },
};

pub fn analyse_bnpcs(ctx: &mut GeneratorContext) {
    let bnpc_bases = ctx
        .excel
        .sheet(MetadataProvider::<BNpcBase>::for_sheet())
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    let bnpc_names = ctx
        .excel
        .sheet(MetadataProvider::<BNpcName>::for_sheet())
        .unwrap()
        .into_iter()
        .map(|name| (name.row_id, name))
        .collect::<BTreeMap<_, _>>();
    let model_charas = ctx
        .excel
        .sheet(MetadataProvider::<ModelChara>::for_sheet())
        .unwrap()
        .into_iter()
        .map(|mc| (mc.row_id, mc))
        .collect::<BTreeMap<_, _>>();
    let npc_equips = ctx
        .excel
        .sheet(MetadataProvider::<NpcEquip>::for_sheet())
        .unwrap()
        .into_iter()
        .map(|equip| (equip.row_id, equip))
        .collect::<BTreeMap<_, _>>();
    let minions = ctx
        .excel
        .sheet(MetadataProvider::<Companion>::for_sheet())
        .unwrap()
        .into_iter()
        .map(|minion| minion.model)
        .collect::<BTreeSet<_>>();

    for bnpc in bnpc_bases {
        if minions.contains(&bnpc.model_chara) {
            // we don't need the battle npc minions
            continue;
        }

        let model_chara = match model_charas.get(&(bnpc.model_chara as u32)) {
            Some(mc) if !mc.kind.is_other() => mc,
            _ => continue,
        };

        let names = ctx
            .bnpcs
            .bnpc
            .iter()
            .filter(|entry| entry.bnpc_base == bnpc.row_id)
            .flat_map(|info| {
                bnpc_names
                    .get(&(info.bnpc_name))
                    .and_then(|name| name.singular.format().ok())
                    .filter(|name| !name.is_empty())
            })
            .map(|name| (ItemKind::BattleNpc, ctx.get_name_idx(name)))
            .collect::<Vec<_>>();

        if names.is_empty() {
            continue;
        }

        let mut variant_id = model_chara.variant;

        // need to use the imc file to map this variant_id to the
        // correct variant_id used in the game path
        if model_chara.kind == ModelCharaKind::Monster {
            let imc = ctx
                .ironworks
                .file::<RawImcFile>(&format!(
                    "chara/monster/m{model:<04}/obj/body/b{base:<04}/b{base:<04}.imc",
                    model = model_chara.model,
                    base = model_chara.base,
                ))
                .ok()
                .and_then(ImcFile::try_from_raw);
            if let Some(imc) = imc {
                let imc_variant = &imc.parts[0].variants[variant_id as usize - 1];
                variant_id = imc_variant.material_id;
            }
        } else if model_chara.kind == ModelCharaKind::Demihuman {
            // look up all the equipment models and their imc, then store the
            // vfx in the vfx map

            let npc_equip = match npc_equips.get(&(bnpc.npc_equip as u32)) {
                Some(ne) => ne,
                _ => continue,
            };

            for (slot_idx, (gear_model_id, gear_variant_id)) in
                npc_equip.gear_models().into_iter().enumerate()
            {
                let imc_path = format!(
                    "chara/demihuman/d{model_chara:<04}/obj/equipment/e{gear_model:<04}/e{gear_model:<04}.imc",
                    model_chara = model_chara.model,
                    gear_model = gear_model_id,
                );

                let imc = match ctx
                    .ironworks
                    .file::<RawImcFile>(&imc_path)
                    .ok()
                    .and_then(ImcFile::try_from_raw)
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

                    ctx.affects
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
            ModelCharaKind::Demihuman => &mut ctx.affects.demihumans,
            ModelCharaKind::Monster => &mut ctx.affects.monsters,
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
}
