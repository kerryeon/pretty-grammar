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
    // From Korean
    assert_eq!(
        "철수는 영희를 좋아합니다.",
        translate!(
                "{name}<은> {obj}<를> 좋아합니다." with
                lang: "kr",
                name: "철수",
                obj: "영희"
        ));
}
