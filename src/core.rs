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

/// Correct the string to match the grammar.
///
/// # Examples
///
/// ```
/// use pretty_grammar::*;
///
/// let result = translate("kr", "철수<은> 영희<를> 좋아합니다.".to_owned());
/// assert_eq!("철수는 영희를 좋아합니다.", result);
/// ```
///
/// # Panics
/// You should not use unsupported `lang`s.
///
pub fn translate(lang: &'static str, msg: String) -> String {
    for (target, func) in super::FILTER.iter() {
        if *target == lang {
            return func(msg)
        }
    }
    // Not Found
    panic!()
}

/// Correct `dynamic` string to match the grammar.
///
/// # Examples
///
/// ```
/// use pretty_grammar::*;
///
/// let name = "철수";
/// let obj = "영희";
/// let format = "{name}<은> {obj}<를> 좋아합니다.";
/// let result = translate!(format with
///     lang: "kr",
///     name: name,
///     obj: obj,
/// );
/// assert_eq!("철수는 영희를 좋아합니다.", result);
/// ```
///
/// # Panics
/// You should not use unsupported `lang`s.
///
#[macro_export]
macro_rules! translate {
    ($msg: tt with lang: $lang: tt, $($tag: ident: $arg: tt),* ) => ({
        $crate::translate(
            $lang, format_dyn!($msg with $($tag: $arg),*)
        )
    });
    // For unnecessary comma
    ($msg: tt with lang: $lang: tt, $($tag: ident: $arg: tt),*, ) => ({
        translate!($msg with lang: $lang, $($tag: $arg),*)
    });
}

/// Creates a `String` using dynamic `format` and interpolation of runtime expressions.
///
/// # Examples
///
/// ```
/// use pretty_grammar::*;
///
/// let name = "철수";
/// let obj = "영희";
/// let format = "{name}는 {obj}를 좋아합니다.";
/// let result = format_dyn!(format with
///     name: name,
///     obj: obj,
/// );
/// assert_eq!("철수는 영희를 좋아합니다.", result);
/// ```
///
#[macro_export]
macro_rules! format_dyn {
    // [Example]
    // {msg} with name: {name}, age: {age}
    ($msg: tt with $($tag: ident: $arg: tt),* ) => ({
        let mut msg: String = $msg.to_owned();
        $(
            msg = msg.replace(format!("{{{}}}", stringify!($tag)).as_str(), $arg);
        )*
        msg
    });
    // For unnecessary comma
    ($msg: tt with $($tag: ident: $arg: tt),*, ) => ({
        format_dyn!($msg with $($tag: $arg),*)
    });
}
