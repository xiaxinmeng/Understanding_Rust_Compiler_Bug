rust
macro_rules! log_newline {
    ($($t:tt)*) => {
        $crate::log_fmt(format_args!("log: {}\n", format_args!($($t)*)))
    }
}
