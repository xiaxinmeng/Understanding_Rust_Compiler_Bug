 rust
macro_rules! check_errno {
    ($($errno:ident),+) => {{
        $( assert_errno_eq(stringify!($errno), $errno as c_int); )+
    }}
}
