mod action;
mod action_cast_timeline;
mod action_timeline;
mod bnpc_base;
mod bnpc_name;
mod companion;
mod emote;
mod enpc_base;
mod enpc_resident;
mod equip_slot_category;
mod extractor;
mod item;
mod map;
mod model_chara;
mod mount;
mod npc_equip;
mod ornament;
mod place_name;
mod provider;
mod text_command;

pub use self::{
    action::Action,
    action_cast_timeline::ActionCastTimeline,
    action_timeline::ActionTimeline,
    bnpc_base::BNpcBase,
    bnpc_name::BNpcName,
    companion::Companion,
    emote::Emote,
    enpc_base::ENpcBase,
    enpc_resident::ENpcResident,
    equip_slot_category::EquipSlotCategory,
    extractor::MetadataExtractor,
    item::Item,
    map::Map,
    model_chara::{ModelChara, ModelCharaKind},
    mount::Mount,
    npc_equip::NpcEquip,
    ornament::Ornament,
    place_name::PlaceName,
    provider::MetadataProvider,
    text_command::TextCommand,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Anyhow(anyhow::Error),
}

impl From<anyhow::Error> for Error {
    fn from(value: anyhow::Error) -> Self {
        Self::Anyhow(value)
    }
}

#[macro_export]
macro_rules! populate {
    (
        $row: expr,
        $(
            [$field_name: ident, $field: expr, $converter: ident],
        )*
        $(
            $field_name_2: ident: $value: expr,
        )*
    ) => {{
        use ::anyhow::Context;

        Self {
            $(
                $field_name: $row
                    .field($field)
                    .context("could not get field")?
                    .$converter()
                    .map_err(|_| ::anyhow::format_err!("field {} was wrong type", stringify!($field_name)))?,
            )*
            $(
                $field_name_2: $value,
            )*
        }
    }}
}
