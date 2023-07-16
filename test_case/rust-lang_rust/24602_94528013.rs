 rust
macro_rules! write {
    ($dst:expr, $($arg:tt)*) => {{
        use $crate::fmt::Write;
        use $crate::io::Write;
        $dst.write_fmt(format_args!($($arg)*))
    }}
}
