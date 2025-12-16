use nom::{
    Parser,
    bytes::complete::tag,
    character::complete::u64,
    combinator::{map, opt},
    sequence::{delimited, terminated},
};

use crate::{GamePath, IResult, Language, simple_part_enum};

pub(crate) fn ui_icon_path(input: &str) -> IResult<&str, GamePath<'_>> {
    map(
        (
            delimited(tag("icon/"), u64, tag("/")),
            opt(terminated(simple_part_enum::<Language>, tag("/"))),
            opt(tag("hq/")),
            u64,
            terminated(opt(tag("_hr1")), tag(".tex")),
        ),
        |(group, language, hq, primary_id, hires)| GamePath::Icon {
            group,
            primary_id,
            language,
            hq: hq.is_some(),
            hires: hires.is_some(),
        },
    )
    .parse(input)
}

#[cfg(test)]
mod test {
    use crate::{GamePath, Language, test::test_path};

    #[test]
    fn simple() {
        const PATH: &str = "ui/icon/218000/218375.tex";
        test_path(
            PATH,
            GamePath::Icon {
                group: 218000,
                primary_id: 218375,
                language: None,
                hq: false,
                hires: false,
            },
        );
    }

    #[test]
    fn language() {
        const PATH: &str = "ui/icon/121000/ja/121697.tex";
        test_path(
            PATH,
            GamePath::Icon {
                group: 121000,
                primary_id: 121697,
                language: Some(Language::Japanese),
                hq: false,
                hires: false,
            },
        );
    }

    #[test]
    fn hq() {
        const PATH: &str = "ui/icon/039000/hq/039110.tex";
        test_path(
            PATH,
            GamePath::Icon {
                group: 39000,
                primary_id: 39110,
                language: None,
                hq: true,
                hires: false,
            },
        );
    }

    #[test]
    fn simple_hires() {
        const PATH: &str = "ui/icon/213000/213263_hr1.tex";
        test_path(
            PATH,
            GamePath::Icon {
                group: 213000,
                primary_id: 213263,
                language: None,
                hq: false,
                hires: true,
            },
        );
    }

    #[test]
    fn language_hires() {
        const PATH: &str = "ui/icon/180000/en/180566_hr1.tex";
        test_path(
            PATH,
            GamePath::Icon {
                group: 180000,
                primary_id: 180566,
                language: Some(Language::English),
                hq: false,
                hires: true,
            },
        );
    }

    #[test]
    fn hq_hires() {
        const PATH: &str = "ui/icon/057000/hq/057199_hr1.tex";
        test_path(
            PATH,
            GamePath::Icon {
                group: 57000,
                primary_id: 57199,
                language: None,
                hq: true,
                hires: true,
            },
        );
    }
}
