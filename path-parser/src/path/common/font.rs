use nom::{
    Parser,
    branch::alt,
    bytes::complete::{tag, take_till, take_while},
    combinator::{map, map_res, opt},
    sequence::{preceded, separated_pair, terminated},
};

use crate::{GamePath, IResult};

// common/font

pub(crate) fn common_font_path(input: &str) -> IResult<&str, GamePath<'_>> {
    preceded(tag("font/"), alt((common_font_tex, common_font_fdt))).parse(input)
}

fn common_font_tex(input: &str) -> IResult<&str, GamePath<'_>> {
    map(terminated(take_till(|c| c == '.'), tag(".tex")), |part| {
        GamePath::FontTexture(part)
    })
    .parse(input)
}

fn common_font_fdt(input: &str) -> IResult<&str, GamePath<'_>> {
    map(
        terminated(common_font_fdt_name, tag(".fdt")),
        |(name, size)| GamePath::FontFile { family: name, size },
    )
    .parse(input)
}

fn common_font_fdt_name(input: &str) -> IResult<&str, (&str, u8)> {
    terminated(
        separated_pair(
            take_till(|c| c == '_'),
            tag("_"),
            map_res(take_while(|c: char| c.is_ascii_digit()), |s: &str| {
                s.parse::<u8>()
            }),
        ),
        opt(tag("_lobby")),
    )
    .parse(input)
}

#[cfg(test)]
mod test {
    use crate::{GamePath, test::test_path};

    #[test]
    pub fn tex() {
        const PATH: &str = "common/font/fontIcon_Ps5.tex";

        test_path(PATH, GamePath::FontTexture("fontIcon_Ps5"));
    }

    #[test]
    pub fn fdt_simple() {
        const PATH: &str = "common/font/AXIS_12.fdt";

        test_path(
            PATH,
            GamePath::FontFile {
                family: "AXIS",
                size: 12,
            },
        );
    }

    #[test]
    pub fn fdt_lobby() {
        const PATH: &str = "common/font/Meidinger_16_lobby.fdt";

        test_path(
            PATH,
            GamePath::FontFile {
                family: "Meidinger",
                size: 16,
            },
        );
    }
}
