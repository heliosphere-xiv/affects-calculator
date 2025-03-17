use affects_common::ItemKind;

use crate::{
    analysers::GeneratorContext,
    formats::imc::{ImcFile, RawImcFile},
    schema::{MetadataProvider, ModelChara, ModelCharaKind, Ornament},
};

pub fn analyse_ornaments(ctx: &mut GeneratorContext) {
    let ornaments = ctx
        .excel
        .sheet(MetadataProvider::<Ornament>::for_sheet())
        .unwrap();
    let model_charas = ctx
        .excel
        .sheet(MetadataProvider::<ModelChara>::for_sheet())
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
        }

        let name_idx = ctx.get_name_idx(ItemKind::FashionAccessory, name);
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
            .insert((ItemKind::FashionAccessory, name_idx));
    }
}
