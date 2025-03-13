use std::marker::PhantomData;

use ironworks::excel::SheetMetadata;

use crate::schema::MetadataExtractor;

pub struct MetadataProvider<S> {
    _phantom: PhantomData<S>,
}

impl<S: MetadataExtractor> MetadataProvider<S> {
    pub fn for_sheet() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<S: MetadataExtractor> SheetMetadata for MetadataProvider<S> {
    type Error = <S as MetadataExtractor>::Error;
    type Row = S;

    fn name(&self) -> String {
        <S as MetadataExtractor>::name()
    }

    fn populate_row(&self, row: ironworks::excel::Row) -> Result<Self::Row, Self::Error> {
        <S as MetadataExtractor>::populate_row(row)
    }
}
