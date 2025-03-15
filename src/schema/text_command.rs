use ironworks::sestring::SeString;

use crate::schema::MetadataExtractor;

#[derive(Debug)]
pub struct TextCommand<'a> {
    pub command: SeString<'a>,
}

impl MetadataExtractor for TextCommand<'_> {
    type Error = super::Error;

    fn name() -> String {
        "TextCommand".into()
    }

    fn populate_row(row: ironworks::excel::Row) -> Result<Self, Self::Error> {
        let item = crate::populate!(row, [command, 5, into_string],);

        Ok(item)
    }
}
