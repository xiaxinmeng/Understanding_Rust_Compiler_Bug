 rust
macro_rules! print {
    ($fmt:tt $($arg:tt)*) => ($crate::io::_print(format_args!(concat!($fmt) $($arg)*)));
}
