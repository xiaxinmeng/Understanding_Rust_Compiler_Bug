rust
macro_rules! format {
    ("{}", $name:ident = $arg:expr $(,)?) => {{
        let res = $crate::fmt::Display::to_string(&$arg);
        res
    }};
    ("{}", $arg:expr $(,)?) => {{
        let res = $crate::fmt::Display::to_string(&$arg);
        res
    }};
    ...
}
