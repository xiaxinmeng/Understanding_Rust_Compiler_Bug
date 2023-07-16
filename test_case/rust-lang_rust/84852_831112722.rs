rust
#[macro_export]
#[stable(feature = "rust1", since = "1.0.0")]
#[allow_internal_unstable(format_args_nl)]
macro_rules! writeln {
    ($dst:expr $(,)?) => (
        $crate::write!($dst, "\n")
    );
    ($dst:expr, $($arg:tt)*) => (
        $dst.write_fmt($crate::format_args_nl!($($arg)*))
    );
}
