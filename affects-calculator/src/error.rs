#[derive(Debug, thiserror::Error)]
pub enum Error<'a> {
    #[error("error parsing path: {0}")]
    Parser(path_parser::Error<'a>),

    #[error("error parsing game data: {0}")]
    GameDataParse(String),
}

impl Error<'_> {
    pub fn game_data_parse<S: Into<String>>(message: S) -> Self {
        Self::GameDataParse(message.into())
    }
}
