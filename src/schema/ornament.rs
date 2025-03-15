use ironworks::sestring::SeString;

use crate::schema::MetadataExtractor;

#[derive(Debug)]
pub struct Ornament<'a> {
    pub model: u16,
    pub singular: SeString<'a>,
}

impl MetadataExtractor for Ornament<'_> {
    type Error = super::Error;

    fn name() -> String {
        "Ornament".into()
    }

    fn populate_row(row: ironworks::excel::Row) -> Result<Self, Self::Error> {
        let item = crate::populate!(row, [model, 0, into_u16], [singular, 8, into_string],);

        Ok(item)
    }
}
