rust
// Causes $dst to be moved, so you can only write to it once.
// The correct behavior requires an autoref on $dst.
macro_rules! write {
    ($dst:expr, $($arg:tt)*) => {
        match $dst {
            mut dst => {
                let result = dst.write_fmt($crate::format_args!($($arg)*));
                result
            }
        }
    };
}
