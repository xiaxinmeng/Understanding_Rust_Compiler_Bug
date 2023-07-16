rust
// current form
macro_rules! format_args {
    ($fmt:expr) => ({ /* compiler built-in */ });
    ($fmt:expr, $($args:tt)*) => ({ /* compiler built-in */ });
}

// "best practices"
macro_rules! format_args {
    ($fmt:expr $(,)?) => ({ /* compiler built-in */ });
    ($fmt:expr, $($args:tt)+) => ({ /* compiler built-in */ });
}
