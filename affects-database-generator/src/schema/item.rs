use ironworks::sestring::SeString;

use crate::schema::MetadataExtractor;

#[derive(Debug)]
pub struct Item<'a> {
    pub row_id: u32,
    // pub singular: SeString<'a>,
    // pub plural: SeString<'a>,
    pub name: SeString<'a>,
    pub item_ui_category: u8,
    pub equip_slot_category: u8,
    pub model_main: u64,
    pub model_sub: u64,
}

impl MetadataExtractor for Item<'_> {
    type Error = super::Error;

    fn name() -> String {
        "Item".into()
    }

    fn populate_row(row: ironworks::excel::Row) -> Result<Self, Self::Error> {
        let item = crate::populate!(
            row,
            // [singular, 0, into_string],
            // [plural, 2, into_string],
            [name, 9, into_string],
            [item_ui_category, 15, into_u8],
            [equip_slot_category, 17, into_u8],
            [model_main, 47, into_u64],
            [model_sub, 48, into_u64],
            row_id: row.row_id(),
        );

        Ok(item)
    }
}
