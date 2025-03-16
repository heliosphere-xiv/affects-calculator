use crate::schema::MetadataExtractor;

#[derive(Debug)]
pub struct ActionCastTimeline {
    pub action_timeline: u16,
    // pub vfx: u16,
}

impl MetadataExtractor for ActionCastTimeline {
    type Error = super::Error;

    fn name() -> String {
        "ActionCastTimeline".into()
    }

    fn populate_row(row: ironworks::excel::Row) -> Result<Self, Self::Error> {
        let item = crate::populate!(
            row,
            [action_timeline, 0, into_u16],
            // [vfx, 0, into_u16],
        );

        Ok(item)
    }
}
