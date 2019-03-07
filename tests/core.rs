/* ------------------------------------------------------------
    PrettyGrammar
    Project.Github: "https://github.com/kerryeon/pretty-grammar"
---------------------------------------------------------------
    Author:
        Name: "kerryeon"
        Email: "besqer996@gnu.ac.kr"
        Github: "https://github.com/kerryeon"
    Generated:
        Date: "3/7/2019"
------------------------------------------------------------ */

use pretty_grammar::*;

#[test]
fn comma_translate() {
    // For Comma
    assert_eq!(
        "감자",
        translate!(
                "감자" with
                lang: "kr",
        ));
}

#[test]
fn comma_format_dyn() {
    // For Comma
    assert_eq!(
        "아버지가방에들어가신다.",
        format_dyn!(
                "{name}가{place}에{act}다." with
                name: "아버지",
                place: "방",
                act: "들어가신",
        ));
}
