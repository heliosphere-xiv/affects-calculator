use ironworks::sestring::SeString;

use crate::schema::MetadataExtractor;

pub struct Item<'a> {
    pub singular: SeString<'a>,
    pub plural: SeString<'a>,
    pub name: SeString<'a>,
    pub model_main: u64,
    pub model_sub: u64,
}

impl MetadataExtractor for Item<'_> {
    type Error = super::Error;

    fn name() -> String {
        "Item".into()
    }

    fn populate_row(row: ironworks::excel::Row) -> Result<Self, Self::Error> {
        // println!("48: {:#?}", row.field(48));
        let item = crate::populate!(
            row,
            "Item".into(),
            [singular, 0, into_string],
            [plural, 2, into_string],
            [name, 9, into_string],
            [model_main, 47, into_u64],
            [model_sub, 48, into_u64],
        );

        Ok(item)
    }
}
