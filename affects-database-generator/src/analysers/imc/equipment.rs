use affects_calculator::formats::imc::{ImcFile, RawImcFile};
use path_parser::types::EquipSlot;

use crate::analysers::GeneratorContext;

pub fn analyse_equipment_imcs(ctx: &mut GeneratorContext) {
    for primaries in ctx.affects.equipment.values() {
        for &primary_id in primaries.keys() {
            let imc = match ctx
                .ironworks
                .file::<RawImcFile>(&format!(
                    "chara/equipment/e{primary_id:<04}/e{primary_id:<04}.imc"
                ))
                .ok()
                .and_then(|file| ImcFile::try_from_raw(file).ok())
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

                for variant in &part.variants {
                    if variant.material_id == 0 || variant.vfx_id == 0 {
                        continue;
                    }

                    ctx.affects
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
}
