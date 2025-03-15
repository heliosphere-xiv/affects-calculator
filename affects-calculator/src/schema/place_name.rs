use ironworks::sestring::SeString;

use crate::schema::MetadataExtractor;

#[derive(Debug)]
pub struct PlaceName<'a> {
    pub name: SeString<'a>,
}

impl MetadataExtractor for PlaceName<'_> {
    type Error = super::Error;

    fn name() -> String {
        "PlaceName".into()
    }

    fn populate_row(row: ironworks::excel::Row) -> Result<Self, Self::Error> {
        let item = crate::populate!(row, [name, 0, into_string],);

        Ok(item)
    }
}
