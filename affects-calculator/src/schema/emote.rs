use ironworks::sestring::SeString;

use crate::schema::MetadataExtractor;

#[derive(Debug)]
pub struct Emote<'a> {
    pub name: SeString<'a>,
    pub action_timelines: Vec<u16>,
    pub text_command: i32,
}

impl MetadataExtractor for Emote<'_> {
    type Error = super::Error;

    fn name() -> String {
        "Emote".into()
    }

    fn populate_row(row: ironworks::excel::Row) -> Result<Self, Self::Error> {
        let timelines = (0..7)
            .map(|i| row.field(1 + i))
            .collect::<Result<Vec<_>, _>>()
            .map_err(super::Error::Ironworks)?
            .into_iter()
            .map(|field| field.into_u16())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| super::Error::FieldWrongType)?;

        let item = crate::populate!(
            row,
            [name, 0, into_string],
            [text_command, 19, into_i32],
            action_timelines: timelines,
        );

        Ok(item)
    }
}
