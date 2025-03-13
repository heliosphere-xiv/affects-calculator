use std::error::Error;

pub trait MetadataExtractor {
    type Error: Error;

    fn name() -> String;
    fn populate_row(row: ironworks::excel::Row) -> Result<Self, Self::Error>
    where
        Self: Sized;
}
