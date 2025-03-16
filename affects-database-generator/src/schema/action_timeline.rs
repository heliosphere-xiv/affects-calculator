use ironworks::sestring::SeString;

use crate::schema::MetadataExtractor;

#[derive(Debug)]
pub struct ActionTimeline<'a> {
    pub key: SeString<'a>,
}

impl MetadataExtractor for ActionTimeline<'_> {
    type Error = super::Error;

    fn name() -> String {
        "ActionTimeline".into()
    }

    fn populate_row(row: ironworks::excel::Row) -> Result<Self, Self::Error> {
        let item = crate::populate!(row, [key, 6, into_string],);

        Ok(item)
    }
}
