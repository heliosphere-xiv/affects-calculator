mod error;
mod extractor;
mod item;
mod provider;

pub use self::{
    error::Error, extractor::MetadataExtractor, item::Item, provider::MetadataProvider,
};

#[macro_export]
macro_rules! populate {
    ($row: expr, $name: expr, $([$field_name: ident, $field: expr, $converter: ident]),+ $(,)?) => {{
        Self {
            $(
                $field_name: $row
                    .field($field)
                    .map_err($crate::schema::Error::Ironworks)?
                    .$converter()
                    .map_err(|_| $crate::schema::Error::FieldWrongType)?,
            )+
        }
    }}
}
