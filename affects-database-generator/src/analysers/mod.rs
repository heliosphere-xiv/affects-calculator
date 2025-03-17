use std::collections::BTreeMap;

use affects_common::{Affects, ItemKind};
use ironworks::{Ironworks, excel::Excel};

use crate::containers::BNpcContainer;

mod actions;
mod bnpcs;
mod emotes;
mod enpcs;
pub mod imc;
mod items;
mod maps;
mod minions;
mod mounts;
mod ornaments;

pub use self::{
    actions::analyse_actions, bnpcs::analyse_bnpcs, emotes::analyse_emotes, enpcs::analyse_enpcs,
    items::analyse_items, maps::analyse_maps, minions::analyse_minions, mounts::analyse_mounts,
    ornaments::analyse_ornaments,
};

pub struct GeneratorContext<'a> {
    pub affects: &'a mut Affects,
    pub ironworks: &'a Ironworks,
    pub excel: &'a Excel,
    pub name_map: &'a mut BTreeMap<String, u16>,
    pub bnpcs: &'a BNpcContainer,
}

impl GeneratorContext<'_> {
    pub fn get_name_idx<S: Into<String>>(&mut self, kind: ItemKind, name: S) -> u16 {
        let name = name.into();
        if let Some(&idx) = self.name_map.get(&name) {
            return idx;
        }

        self.affects.names.push(name.clone());
        let name_idx = self.affects.names.len() - 1;
        let name_idx = u16::try_from(name_idx).expect("name idx exceeded 16 bits");

        self.name_map.insert(name, name_idx);

        self.affects
            .name_kinds
            .entry(name_idx)
            .or_default()
            .insert(kind);

        name_idx
    }
}
