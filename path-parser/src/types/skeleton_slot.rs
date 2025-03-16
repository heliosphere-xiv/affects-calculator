use std::str::FromStr;

use nom::{Parser, bytes::complete::take_till, combinator::map_res};

use crate::IResult;

enum_str! {
    pub enum SkeletonSlot {
        Head => "met",
        Hands => "glv",
        Legs => "dwn",
        Feet => "sho",
        Body => "top",
        Ears => "ear",
        Neck => "nek",
        RFinger => "rir",
        LFinger => "ril",
        Wrists => "wrs",
        Base => "base",
        Face => "face",
        Hair => "hair",
    }
}

impl SkeletonSlot {
    pub fn abbreviation(self) -> &'static str {
        match self {
            Self::Head => "m",
            Self::Hands => "g",
            Self::Legs => "d",
            Self::Feet => "s",
            Self::Body => "t",
            Self::Base => "b",
            Self::Face => "f",
            Self::Hair => "h",

            _ => "",
        }
    }
}

impl std::fmt::Display for SkeletonSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Head => "Helmet",
            Self::Hands => "Gloves",
            Self::Legs => "Pants",
            Self::Feet => "Shoes",
            Self::Body => "Top",
            Self::Ears => "Earrings",
            Self::Neck => "Necklace",
            Self::RFinger => "Ring",
            Self::LFinger => "Ring",
            Self::Wrists => "Bracelet",
            Self::Base => "Base",
            Self::Face => "Face",
            Self::Hair => "Hair",
        };

        write!(f, "{s}")
    }
}

pub fn skeleton_slot(input: &str) -> IResult<&str, SkeletonSlot> {
    map_res(take_till(|c| c == '/'), SkeletonSlot::from_str).parse(input)
}
