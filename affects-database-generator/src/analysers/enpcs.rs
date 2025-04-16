use std::collections::BTreeMap;

use affects_common::ItemKind;

use crate::{
    analysers::GeneratorContext,
    formats::imc::{ImcFile, RawImcFile},
    schema::{ENpcBase, ENpcResident, MetadataProvider, ModelChara, ModelCharaKind},
};

pub fn analyse_enpcs(ctx: &mut GeneratorContext) {
    let enpc_bases = ctx
        .excel
        .sheet(MetadataProvider::<ENpcBase>::for_sheet())
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    let enpc_residents = ctx
        .excel
        .sheet(MetadataProvider::<ENpcResident>::for_sheet())
        .unwrap()
        .into_iter()
        .map(|res| res.map(|res| (res.row_id, res)))
        .collect::<Result<BTreeMap<_, _>, _>>()
        .unwrap();
    let model_charas = ctx
        .excel
        .sheet(MetadataProvider::<ModelChara>::for_sheet())
        .unwrap()
        .into_iter()
        .map(|mc| mc.map(|mc| (mc.row_id, mc)))
        .collect::<Result<BTreeMap<_, _>, _>>()
        .unwrap();

    for enpc in enpc_bases {
        let enpc = enpc.unwrap();

        let model_chara = match model_charas.get(&(enpc.model_chara as u32)) {
            Some(mc) if !mc.kind.is_other() => mc,
            _ => continue,
        };

        let resident = match enpc_residents.get(&{ enpc.row_id }) {
            Some(resident) => resident,
            _ => continue,
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

            for (slot_idx, (gear_model_id, gear_variant_id)) in
                enpc.gear_models().into_iter().enumerate()
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

        let name_idx = ctx.get_name_idx(ItemKind::EventNpc, name);
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
            .insert((ItemKind::EventNpc, name_idx));
    }
}
