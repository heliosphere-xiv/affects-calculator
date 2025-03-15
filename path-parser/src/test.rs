use nom_language::error::convert_error;

use crate::GamePath;

pub(crate) fn test_path(path: &str, expected: GamePath) {
    let res = super::game_path(path);
    match res {
        Ok(r) => assert_eq!(("", expected), r),
        Err(e) => match e {
            nom::Err::Incomplete(needed) => panic!("needed: {:?}", needed),
            nom::Err::Error(e) => panic!("{}", convert_error(path, e)),
            nom::Err::Failure(e) => panic!("{}", convert_error(path, e)),
        },
    }
}

#[test]
pub fn test_raw_part_single() {
    assert_eq!(Ok(("", "hello")), super::raw_part("hello"));
}

#[test]
pub fn test_raw_part_multi() {
    let path = "hello/world";
    assert_eq!(Ok(("/world", "hello")), super::raw_part(path));
}
