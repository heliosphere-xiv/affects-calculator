use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::EquipSlot;

type NameSet = BTreeSet<(ItemKind, u16)>;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Affects {
    pub names: Vec<String>,
    /// slot => model => variant => set of name indices
    pub equipment: BTreeMap<EquipSlot, BTreeMap<u16, BTreeMap<u8, NameSet>>>,
    /// model => secondary => variant => set of name indices
    pub weapons: BTreeMap<u16, BTreeMap<u16, BTreeMap<u8, NameSet>>>,
    /// timeline key => set of name indices (name, command)
    pub emotes: BTreeMap<String, BTreeSet<(ItemKind, u16, Option<u16>)>>,
    /// model => base => variant => set of name indices
    pub monsters: BTreeMap<u16, BTreeMap<u8, BTreeMap<u8, NameSet>>>,
    /// model => base => variant => set of name indices
    pub demihumans: BTreeMap<u16, BTreeMap<u8, BTreeMap<u8, NameSet>>>,
    /// animation id => set of name indices
    pub actions: BTreeMap<String, NameSet>,
    /// map id => set of name indices
    pub maps: BTreeMap<String, NameSet>,

    pub vfx: VfxMaps,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct VfxMaps {
    /// model => vfx => (slot, variant)s
    pub equipment: BTreeMap<u16, BTreeMap<u8, BTreeSet<(EquipSlot, u8)>>>,
    /// model => base => vfx => variants
    pub monsters: BTreeMap<u16, BTreeMap<u8, BTreeMap<u8, BTreeSet<u8>>>>,
    /// model => base => vfx => variants
    pub demihumans: BTreeMap<u16, BTreeMap<u8, BTreeMap<u8, BTreeSet<u8>>>>,
    /// model => weapon => vfx => variants
    pub weapons: BTreeMap<u16, BTreeMap<u8, BTreeMap<u8, BTreeSet<u8>>>>,
}

#[derive(Debug, Deserialize_repr, Serialize_repr, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
#[repr(u8)]
pub enum ItemKind {
    Gear,
    Weapon,
    Emote,
    BattleNpc,
    EventNpc,
    Minion,
    Mount,
    FashionAccessory,
    Customisation,
    Action,
    Map,
    Icon,
    Font,
    Miscellaneous,
    Animation,
}

impl std::fmt::Display for ItemKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Gear => "Gear",
            Self::Weapon => "Weapon",
            Self::Emote => "Emote",
            Self::BattleNpc => "Battle NPC",
            Self::EventNpc => "Event NPC",
            Self::Minion => "Minion",
            Self::Mount => "Mount",
            Self::FashionAccessory => "Fashion Accessory",
            Self::Customisation => "Customisation",
            Self::Action => "Action",
            Self::Map => "Map",
            Self::Icon => "Icon",
            Self::Font => "Font",
            Self::Miscellaneous => "Miscellaneous",
            Self::Animation => "Animation",
        };

        write!(f, "{s}")
    }
}
