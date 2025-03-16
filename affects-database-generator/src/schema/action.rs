use ironworks::sestring::SeString;

use crate::schema::MetadataExtractor;

#[derive(Debug)]
pub struct Action<'a> {
    pub name: SeString<'a>,
    pub animation_start: u8,
    pub animation_end: i16,
    pub animation_hit: u16,
    // pub vfx: u16,
}

impl MetadataExtractor for Action<'_> {
    type Error = super::Error;

    fn name() -> String {
        "Action".into()
    }

    fn populate_row(row: ironworks::excel::Row) -> Result<Self, Self::Error> {
        let item = crate::populate!(
            row,
            [name, 0, into_string],
            [animation_start, 5, into_u8],
            // [vfx, 6, into_u16],
            [animation_end, 7, into_i16],
            [animation_hit, 8, into_u16],
        );

        Ok(item)
    }
}
