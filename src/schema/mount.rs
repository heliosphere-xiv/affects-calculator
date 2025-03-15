use ironworks::sestring::SeString;

use crate::schema::MetadataExtractor;

#[derive(Debug)]
pub struct Mount<'a> {
    pub singular: SeString<'a>,
    pub model_chara: i32,
}

impl MetadataExtractor for Mount<'_> {
    type Error = super::Error;

    fn name() -> String {
        "Mount".into()
    }

    fn populate_row(row: ironworks::excel::Row) -> Result<Self, Self::Error> {
        let item = crate::populate!(row, [singular, 0, into_string], [model_chara, 8, into_i32],);

        Ok(item)
    }
}
