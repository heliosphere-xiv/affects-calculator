use nom::{
    Parser,
    bytes::complete::{tag, take},
    character::complete::one_of,
    combinator::{map, opt},
    sequence::{delimited, preceded, terminated},
};

use crate::{GamePath, IResult, n_digit_id};

const ALPHA: &str = "abcdefghijklmnopqrstuvwxyz";

pub(crate) fn ui_map_path(input: &str) -> IResult<&str, GamePath> {
    let (left, (primary_id, variant)) = (
        delimited(tag("map/"), take(4_usize), tag("/")),
        terminated(n_digit_id::<u8>(2), tag("/")),
    )
        .parse(input)?;
    map(
        (
            preceded(
                (tag(primary_id), tag(&*format!("{variant:<02}"))),
                opt(one_of(ALPHA)),
            ), // FIXME
            terminated(opt(preceded(tag("_"), one_of(ALPHA))), tag(".tex")),
        ),
        |(suffix, extra)| GamePath::Map {
            primary_id,
            variant,
            suffix,
            extra,
        },
    )
    .parse(left)
}

#[cfg(test)]
mod test {
    use crate::{GamePath, test::test_path};

    #[test]
    fn simple() {
        const PATH: &str = "ui/map/z6r1/02/z6r102.tex";
        test_path(
            PATH,
            GamePath::Map {
                primary_id: "z6r1",
                variant: 2,
                suffix: None,
                extra: None,
            },
        );
    }

    #[test]
    fn suffix() {
        const PATH: &str = "ui/map/z6r1/02/z6r102d.tex";
        test_path(
            PATH,
            GamePath::Map {
                primary_id: "z6r1",
                variant: 2,
                suffix: Some('d'),
                extra: None,
            },
        );
    }

    #[test]
    fn extra() {
        const PATH: &str = "ui/map/z6r1/02/z6r102_m.tex";
        test_path(
            PATH,
            GamePath::Map {
                primary_id: "z6r1",
                variant: 2,
                suffix: None,
                extra: Some('m'),
            },
        );
    }

    #[test]
    fn suffix_extra() {
        const PATH: &str = "ui/map/z6r1/02/z6r102d_m.tex";
        test_path(
            PATH,
            GamePath::Map {
                primary_id: "z6r1",
                variant: 2,
                suffix: Some('d'),
                extra: Some('m'),
            },
        );
    }
}
