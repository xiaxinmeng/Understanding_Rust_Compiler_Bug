rust
macro_rules! next {
    ($name:ident($($arg:expr),*)) => {
        crate::lookup(
            $name,
            concat!(stringify!($name), "\0").as_ptr()
        )($($arg),*)
    };
}
