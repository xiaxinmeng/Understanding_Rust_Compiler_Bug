rust
macro_rules! format {
    ($($arg:tt)*) => ((|| {$crate::fmt::format($crate::__export::format_args!($($arg)*))})())
}
