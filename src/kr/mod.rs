/* ------------------------------------------------------------
    PrettyGrammar
    Project.Github: "https://github.com/kerryeon/pretty-grammar"
---------------------------------------------------------------
    Author:
        Name: "kerryeon"
        Email: "besqer996@gnu.ac.kr"
        Github: "https://github.com/kerryeon"
    Generated:
        Date: "3/6/2019"
------------------------------------------------------------ */

mod from_en;
mod from_num;
mod from_kr;

use regex::Regex;

type TestOff = fn (TokenVec) -> bool;
type TokenVec = Vec<u32>;
type Swap = fn (String, String, &'static str, &'static str) -> String;

const FILTER: [(&str, &str); 15] = [
    // 받침이 있는 경우, 받침이 없는 경우
    ("은", "는"),
    ("이", "가"),
    ("을", "를"),
    ("과", "와"),
    ("아", "야"),
    ("이고", "고"),
    ("이나", "나"),
    ("이다", "다"),
    ("이라", "라"),
    // 이라고, 이라도, ...
    ("이랑", "랑"),
    ("으로", "로"),
    // 으로서, 으로써, ...
    ("이며", "며"),
    ("이면", "면"),
    ("이여", "여"),
    ("이시여", "여"),
];

const FILTER_SWAP: [Swap; 3] = [
    from_en::swap,
    from_num::swap,
    from_kr::swap,
];

pub fn call(mut msg: String) -> String {
    for (on, off) in FILTER.iter() {
        for swap_fn in FILTER_SWAP.iter() {
            msg = swap_fn(msg, format!("<{}>", on), on, off);
            msg = swap_fn(msg, format!("<{}>", off), on, off);
        }
    }
    msg
}

fn swap(mut msg: String, from: String, on: &'static str, off: &'static str,
        filter_range: &'static str, test_off: TestOff) -> String {
    // Get Regex
    let filter = Regex::new(format!(
        r"(?P<last>{}){}", filter_range, from
    ).as_str()).unwrap();
    // Test
    for token in filter.captures_iter(msg.clone().as_str()) {
        let token_vec: TokenVec = (&token["last"]).chars().map(|c| c as u32).collect();
        let to = match test_off(token_vec) {
            true => off,
            false => on,
        };
        let msg_new = msg.replace(from.as_str(), to);
        msg = msg_new;
    }
    msg
}
