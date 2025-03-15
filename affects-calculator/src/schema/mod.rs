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
mod error;
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
    error::Error,
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
        Self {
            $(
                $field_name: $row
                    .field($field)
                    .map_err($crate::schema::Error::Ironworks)?
                    .$converter()
                    .map_err(|_| $crate::schema::Error::FieldWrongType)?,
            )*
            $(
                $field_name_2: $value,
            )*
        }
    }}
}
