use std::str::FromStr;

use nom::{
    Parser,
    combinator::{eof, map_res, opt},
};

use crate::{IResult, raw_part};

pub enum FileOrPart<'a, T> {
    File(&'a str),
    Part(T),
}

impl<T> PartialEq for FileOrPart<'_, T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::File(l0), Self::File(r0)) => l0 == r0,
            (Self::Part(l0), Self::Part(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl<T> Eq for FileOrPart<'_, T> where T: Eq {}

impl<T> std::fmt::Debug for FileOrPart<'_, T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::File(arg0) => f.debug_tuple("File").field(arg0).finish(),
            Self::Part(arg0) => f.debug_tuple("Part").field(arg0).finish(),
        }
    }
}

pub fn file_or_part<T: FromStr>(input: &str) -> IResult<&str, FileOrPart<'_, T>> {
    map_res((raw_part, opt(eof)), |(part, eof)| {
        if eof.is_some() {
            return Ok(FileOrPart::File(part));
        }

        T::from_str(part).map(FileOrPart::Part)
    })
    .parse(input)
}
