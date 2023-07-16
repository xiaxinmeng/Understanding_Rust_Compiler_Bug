rust
macro_rules! write {
    ($dst:expr, $($arg:tt)*) => {
        match autoref_mut!($dst) {
            dst => {
                let result = dst.write_fmt($crate::format_args!($($arg)*));
                result
            }
        }
    };
}
