use ironworks::sestring::SeString;

use crate::schema::MetadataExtractor;

#[derive(Debug)]
pub struct Map<'a> {
    pub id: SeString<'a>,
    pub place_name_region: u16,
    pub place_name: u16,
    pub place_name_sub: u16,
}

impl MetadataExtractor for Map<'_> {
    type Error = super::Error;

    fn name() -> String {
        "Map".into()
    }

    fn populate_row(row: ironworks::excel::Row) -> Result<Self, Self::Error> {
        let item = crate::populate!(
            row,
            [id, 6, into_string],
            [place_name_region, 10, into_u16],
            [place_name, 11, into_u16],
            [place_name_sub, 12, into_u16],
        );

        Ok(item)
    }
}
