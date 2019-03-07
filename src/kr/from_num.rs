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

const FILTER: &'static str = "[0-9]";
const OFFSET: u32 = 0x0030;

// May be incorrect when bigger than 1e+12 (조).
pub fn swap(msg: String, from: String, on: &'static str, off: &'static str) -> String {
    _swap(msg, from, on, off, FILTER,
          |vec| match vec[0] - OFFSET {
              2 | 4 | 5 | 9 => true,
              _ => false,
          })
}
