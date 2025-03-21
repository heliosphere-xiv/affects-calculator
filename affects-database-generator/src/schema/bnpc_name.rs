use ironworks::sestring::SeString;

use crate::schema::MetadataExtractor;

#[derive(Debug)]
pub struct BNpcName<'a> {
    pub row_id: u32,
    pub singular: SeString<'a>,
}

impl MetadataExtractor for BNpcName<'_> {
    type Error = super::Error;

    fn name() -> String {
        "BNpcName".into()
    }

    fn populate_row(row: ironworks::excel::Row) -> Result<Self, Self::Error> {
        let item = crate::populate!(
            row,
            [singular, 0, into_string],
            row_id: row.row_id(),
        );

        Ok(item)
    }
}
