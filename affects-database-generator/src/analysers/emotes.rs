use affects_common::ItemKind;

use crate::{
    analysers::GeneratorContext,
    schema::{ActionTimeline, Emote, MetadataProvider, TextCommand},
};

pub fn analyse_emotes(ctx: &mut GeneratorContext) {
    let emotes = ctx
        .excel
        .sheet(MetadataProvider::<Emote>::for_sheet())
        .unwrap();
    let action_timelines = ctx
        .excel
        .sheet(MetadataProvider::<ActionTimeline>::for_sheet())
        .unwrap();
    let text_commands = ctx
        .excel
        .sheet(MetadataProvider::<TextCommand>::for_sheet())
        .unwrap();

    for emote in emotes {
        let emote = emote.unwrap();

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

        let name = command
            .map(|command| format!("{name} ({command})"))
            .unwrap_or(name);

        let name_idx = ctx.get_name_idx(ItemKind::Emote, name);
        ctx.affects
            .emotes
            .entry(key)
            .or_default()
            .insert((ItemKind::Emote, name_idx));
    }
}
