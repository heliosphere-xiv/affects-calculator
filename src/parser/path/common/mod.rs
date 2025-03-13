mod font;

use nom::{Parser, bytes::complete::tag, sequence::preceded};

use crate::parser::{GamePath, IResult, path::common::font::common_font_path};

pub(crate) fn common_path(input: &str) -> IResult<&str, GamePath> {
    preceded(tag("common/"), common_font_path).parse(input)
}
