use ironworks::sestring::SeString;

use crate::schema::MetadataExtractor;

#[derive(Debug)]
pub struct ENpcResident<'a> {
    pub row_id: u32,
    pub singular: SeString<'a>,
    pub plural: SeString<'a>,
}

impl MetadataExtractor for ENpcResident<'_> {
    type Error = super::Error;

    fn name() -> String {
        "ENpcResident".into()
    }

    fn populate_row(row: ironworks::excel::Row) -> Result<Self, Self::Error> {
        let item = crate::populate!(
            row,
            [singular, 0, into_string],
            [plural, 2, into_string],
            row_id: row.row_id(),
        );

        Ok(item)
    }
}
