rust
// Does not work if $arg references $dst such as `write!(w, "{}", w.foo)`, nor (less
// importantly) if $dst contains a return or break or continue or await.
macro_rules! write {
    ($dst:expr, $($arg:tt)*) => {
        match |args: $crate::fmt::Arguments<'_>| $dst.write_fmt(args) {
            mut write_fmt => {
                let result = write_fmt($crate::format_args!($($arg)*));
                result
            }
        }
    };
}
