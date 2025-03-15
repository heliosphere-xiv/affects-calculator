#[derive(Debug, thiserror::Error)]
pub enum Error<'a> {
    #[cfg(not(test))]
    #[error("error parsing path: {0}")]
    Nom(nom::Err<nom::error::Error<&'a str>>),
    #[cfg(test)]
    #[error("error parsing path: {0}")]
    Nom(nom::Err<nom_language::error::VerboseError<&'a str>>),
    #[error("did not completely parse path")]
    IncompleteParse,
    #[error("path id has mismatched file ids (expected {expected} but found {actual})")]
    MismatchedPathIds { expected: u32, actual: u32 },
}
