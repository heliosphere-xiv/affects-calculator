use ironworks::sestring::SeString;

use crate::schema::MetadataExtractor;

#[derive(Debug)]
pub struct Companion<'a> {
    pub singular: SeString<'a>,
    pub model: u16,
}

impl MetadataExtractor for Companion<'_> {
    type Error = super::Error;

    fn name() -> String {
        "Companion".into()
    }

    fn populate_row(row: ironworks::excel::Row) -> Result<Self, Self::Error> {
        let item = crate::populate!(row, [singular, 0, into_string], [model, 8, into_u16],);

        Ok(item)
    }
}
