use affects_calculator::schema::{Map, MetadataProvider, PlaceName};

use crate::{analysers::GeneratorContext, containers::ItemKind};

pub fn analyse_maps(ctx: &mut GeneratorContext) {
    let maps = ctx
        .excel
        .sheet(MetadataProvider::<Map>::for_sheet())
        .unwrap();
    let place_names = ctx
        .excel
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

        let name_idx = ctx.get_name_idx(name);
        ctx.affects
            .maps
            .entry(id)
            .or_default()
            .insert((ItemKind::Map, name_idx));
    }
}
