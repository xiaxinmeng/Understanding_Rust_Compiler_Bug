 Rust
#![macro_escape]
macro_rules! assert_ (
    ($cond:expr, $($arg:expr)+) => (
        if !$cond { fail!($($arg),+) }
    )
) // 6 lines
