use crate::{
    analysers::GeneratorContext,
    formats::imc::{ImcFile, RawImcFile},
};

pub fn analyse_monster_imcs(ctx: &mut GeneratorContext) {
    for (&model_id, bases) in &ctx.affects.monsters {
        for &base_id in bases.keys() {
            let imc = match ctx
                .ironworks
                .file::<RawImcFile>(&format!(
                    "chara/monster/m{model:<04}/obj/body/b{base:<04}/b{base:<04}.imc",
                    model = model_id,
                    base = base_id,
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
}
