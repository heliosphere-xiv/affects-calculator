use nom::{Parser, branch::alt, bytes::complete::tag, sequence::preceded};

use crate::parser::{GamePath, IResult};

mod icon;
mod map;

pub(crate) fn ui_path(input: &str) -> IResult<&str, GamePath> {
    preceded(tag("ui/"), alt((icon::ui_icon_path, map::ui_map_path))).parse(input)
}
