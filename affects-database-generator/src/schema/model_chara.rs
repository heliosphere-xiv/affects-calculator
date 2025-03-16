use anyhow::Context;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::schema::MetadataExtractor;

#[derive(Debug)]
pub struct ModelChara {
    pub row_id: u32,
    pub kind: ModelCharaKind,
    pub model: u16,
    pub base: u8,
    pub variant: u8,
}

impl MetadataExtractor for ModelChara {
    type Error = super::Error;

    fn name() -> String {
        "ModelChara".into()
    }

    fn populate_row(row: ironworks::excel::Row) -> Result<Self, Self::Error> {
        let kind = ModelCharaKind::from(
            row.field(0)
                .context("could not get field")?
                .into_u8()
                .map_err(|_| anyhow::format_err!("field was wrong type"))?,
        );

        let item = crate::populate!(
            row,
            [model, 1, into_u16],
            [base, 2, into_u8],
            [variant, 3, into_u8],
            kind: kind,
            row_id: row.row_id(),
        );

        Ok(item)
    }
}

#[derive(
    Debug, Clone, Copy, Deserialize_repr, Serialize_repr, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[repr(u8)]
pub enum ModelCharaKind {
    Monster,
    Demihuman,
    Other,
}

impl ModelCharaKind {
    pub fn is_other(self) -> bool {
        matches!(self, Self::Other)
    }
}

impl From<u8> for ModelCharaKind {
    fn from(value: u8) -> Self {
        match value {
            2 => Self::Demihuman,
            3 => Self::Monster,
            _ => Self::Other,
        }
    }
}
