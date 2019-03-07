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

#![feature(external_doc)]
#[doc(include = "../README.md")]

mod core;
mod kr;

pub use self::core::translate;

type FormatMap = (&'static str, fn (String) -> String);

const FILTER: [FormatMap; 1] = [
    ("kr", kr::call),
];
