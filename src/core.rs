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

pub fn translate(lang: &'static str, msg: String) -> String {
    for (target, func) in super::MAP.iter() {
        if *target == lang {
            return func(msg)
        }
    }
    // Not Found
    panic!()
}

#[macro_export]
macro_rules! translate {
    // [Example]
    // {msg} with lang: {lang}, name: {name}, age: {age}
    ($msg: tt with lang: $lang: tt, $($tag: ident: $aa: tt),* ) => ({
        $crate::translate(
            $lang, format_dyn!($msg with $($tag: $aa),*)
        )
    });
}

#[macro_export]
macro_rules! format_dyn {
    // [Example]
    // {msg} with name: {name}, age: {age}
    ($msg: tt with $($tag: ident: $aa: tt),* ) => ({
        let mut msg: String = $msg.to_owned();
        $(
            msg = msg.replace(format!("{{{}}}", stringify!($tag)).as_str(), $aa);
        )*
        msg
    });
}
