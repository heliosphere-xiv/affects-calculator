use affects_calculator::schema::{Action, ActionCastTimeline, ActionTimeline, MetadataProvider};

use crate::{analysers::GeneratorContext, containers::ItemKind};

pub fn analyse_actions(ctx: &mut GeneratorContext) {
    let actions = ctx
        .excel
        .sheet(MetadataProvider::<Action>::for_sheet())
        .unwrap();
    let action_timelines = ctx
        .excel
        .sheet(MetadataProvider::<ActionTimeline>::for_sheet())
        .unwrap();
    let action_cast_timelines = ctx
        .excel
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

            let name_idx = ctx.get_name_idx(name);
            ctx.affects
                .actions
                .entry(key)
                .or_default()
                .insert((ItemKind::Action, name_idx));
        };

        add_action(start_key, &name);
        add_action(end_key, &name);
        add_action(hit_key, &name);
    }
}
