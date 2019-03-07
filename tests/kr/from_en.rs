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
fn kr() {
    // From English
    assert_eq!(
        "Potato는 salt를 곁들이면 맛있습니다.",
        translate!(
                "{name}<은> {obj}<를> 곁들이면 맛있습니다." with
                lang: "kr",
                name: "Potato",
                obj: "salt"
        ));
}
