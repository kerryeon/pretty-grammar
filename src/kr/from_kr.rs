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

const FILTER: &'static str = "[ㄱ-힣]";
const OFFSET: u32 = 0xAC00;
const STRIDE_JONGSEONG: u32 = 28;

pub fn swap(msg: String, from: String, on: &'static str, off: &'static str) -> String {
    _swap(msg, from, on, off, FILTER,
          |vec| (vec[0] - OFFSET) % STRIDE_JONGSEONG == 0)
}
