use nom::{Parser, branch::alt, bytes::complete::tag, sequence::preceded};

pub use self::{
    accessory::AccessoryPath,
    character::{BodyType, BodyTypeSlot, CharacterPath, DecalType},
    demihuman::DemihumanPath,
    equipment::EquipmentPath,
    monster::MonsterPath,
    weapon::WeaponPath,
};
use crate::{
    GamePath, IResult,
    path::chara::{
        accessory::chara_accessory_path, character::chara_character_path,
        demihuman::chara_demihuman_path, equipment::chara_equipment_path,
        monster::chara_monster_path, weapon::chara_weapon_path,
    },
};

mod accessory;
mod character;
mod demihuman;
mod equipment;
mod monster;
mod weapon;

pub(crate) fn chara_path(input: &str) -> IResult<&str, GamePath> {
    preceded(
        tag("chara/"),
        alt((
            chara_equipment_path,
            chara_monster_path,
            chara_weapon_path,
            chara_demihuman_path,
            chara_accessory_path,
            chara_character_path,
        )),
    )
    .parse(input)
}
