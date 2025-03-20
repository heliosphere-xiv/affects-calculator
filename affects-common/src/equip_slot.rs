use std::str::FromStr;

use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(
    Debug, Serialize_repr, Deserialize_repr, Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Copy,
)]
#[repr(u8)]
pub enum EquipSlot {
    Head,
    Hands,
    Legs,
    Feet,
    Body,
    Ears,
    Neck,
    RFinger,
    LFinger,
    Wrists,
}

impl FromStr for EquipSlot {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let slot = match s {
            "met" => Self::Head,
            "glv" => Self::Hands,
            "dwn" => Self::Legs,
            "sho" => Self::Feet,
            "top" => Self::Body,
            "ear" => Self::Ears,
            "nek" => Self::Neck,
            "rir" => Self::RFinger,
            "ril" => Self::LFinger,
            "wrs" => Self::Wrists,

            _ => return Err(()),
        };

        Ok(slot)
    }
}

impl EquipSlot {
    pub fn to_id(self) -> u64 {
        match self {
            Self::Head => 3,
            Self::Hands => 5,
            Self::Legs => 7,
            Self::Feet => 8,
            Self::Body => 4,
            Self::Ears => 9,
            Self::Neck => 10,
            Self::RFinger => 12,
            Self::LFinger => 14,
            Self::Wrists => 11,
        }
    }

    pub fn to_imc_part_idx(self) -> Option<usize> {
        let idx = match self {
            Self::Head => 0,
            Self::Body => 1,
            Self::Hands => 2,
            Self::Legs => 3,
            Self::Feet => 4,

            Self::Ears => 0,
            Self::Neck => 1,
            Self::Wrists => 2,
            Self::RFinger => 3,
            Self::LFinger => 4,
        };

        Some(idx)
    }

    pub fn is_accessory(self) -> bool {
        matches!(
            self,
            Self::Ears | Self::Neck | Self::Wrists | Self::RFinger | Self::LFinger
        )
    }

    pub fn abbreviation(&self) -> &str {
        match self {
            Self::Head => "m",
            Self::Hands => "g",
            Self::Legs => "d",
            Self::Feet => "s",
            Self::Body => "t",
            _ => "",
        }
    }
}

impl std::fmt::Display for EquipSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Head => "Helmet",
            Self::Hands => "Gloves",
            Self::Legs => "Pants",
            Self::Feet => "Shoes",
            Self::Body => "Chestpiece",
            Self::Ears => "Earrings",
            Self::Neck => "Necklace",
            Self::RFinger | Self::LFinger => "Ring",
            Self::Wrists => "Bracelet",
        };

        write!(f, "{s}")
    }
}
