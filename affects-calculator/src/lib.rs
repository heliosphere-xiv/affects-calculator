pub mod formats;
pub mod schema;

mod error;

pub use self::error::Error;

pub type Result<'a, T, E = Error<'a>> = std::result::Result<T, E>;
