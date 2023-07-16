 rust
#[macro_export]
macro_rules! matches {
    ($expression: expr, $($pattern:tt)+) => {
        _tt_as_expr_hack! {
            match $expression {
                $($pattern)+ => true,
                _ => false
            }
        }
    }
}


/// Work around "error: unexpected token: `an interpolated tt`", whatever that means.
#[macro_export]
macro_rules! _tt_as_expr_hack {
    ($value:expr) => ($value)
}
