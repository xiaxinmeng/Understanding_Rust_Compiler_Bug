rust
// Does not work if $dst is declared `f: &mut W` where the reference is mutable
// but the binding is not.
macro_rules! write {
    ($dst:expr, $($arg:tt)*) => {
        match &mut $dst {
            dst => {
                let result = dst.write_fmt($crate::format_args!($($arg)*));
                result
            }
        }
    };
}
