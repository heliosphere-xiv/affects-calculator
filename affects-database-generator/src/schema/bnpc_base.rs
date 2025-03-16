use crate::schema::MetadataExtractor;

#[derive(Debug)]
pub struct BNpcBase {
    pub row_id: u32,
    pub model_chara: u16,
    pub npc_equip: u16,
}

impl MetadataExtractor for BNpcBase {
    type Error = super::Error;

    fn name() -> String {
        "BNpcBase".into()
    }

    fn populate_row(row: ironworks::excel::Row) -> Result<Self, Self::Error> {
        let item = crate::populate!(
            row,
            [model_chara, 5, into_u16],
            [npc_equip, 7, into_u16],
            row_id: row.row_id(),
        );

        Ok(item)
    }
}
