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

use super::swap as _swap;

const FILTER: &'static str = "[a-zA-Z]";

// May be incorrect.
pub fn swap(msg: String, from: String, on: &'static str, off: &'static str) -> String {
    _swap(msg, from, on, off, FILTER,
         |_| true)
}


#[cfg(test)]
mod tests {
    pub use crate::*;

    #[test]
    fn kr() {
        // From Korean
        assert_eq!(
            "Potato는 salt를 곁들이면 맛있습니다.",
            translate!(
                "{name}<은> {obj}<를> 곁들이면 맛있습니다." with
                lang: "kr",
                name: "Potato",
                obj: "salt"
        ));
    }
}
