#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("ironworks error: {0}")]
    Ironworks(ironworks::Error),

    #[error("field was the wrong type")]
    FieldWrongType,
}
