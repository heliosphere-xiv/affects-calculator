use crate::{
    analysers::GeneratorContext,
    formats::imc::{ImcFile, RawImcFile},
};

pub fn analyse_weapon_imcs(ctx: &mut GeneratorContext) {
    for (&model_id, weapons) in &ctx.affects.weapons {
        for &weapon_id in weapons.keys() {
            let imc = match ctx
                .ironworks
                .file::<RawImcFile>(&format!(
                    "chara/weapon/w{model_id:<04}/obj/body/b{weapon_id:<04}/b{weapon_id:<04}.imc"
                ))
                .ok()
                .and_then(ImcFile::try_from_raw)
            {
                Some(imc) => imc,
                None => continue,
            };

            for part in imc.parts {
                for variant in part.variants {
                    if variant.material_id == 0 || variant.vfx_id == 0 {
                        continue;
                    }

                    ctx.affects
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
}
