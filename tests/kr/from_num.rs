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
    // From Numeric
    assert_eq!(
        "900과 99를 더하면 999가 됩니다.",
        translate!(
                "{left}<와> {right}<를> 더하면 {answer}<가> 됩니다." with
                lang: "kr",
                left: "900",
                right: "99",
                answer: "999"
        ));
}
