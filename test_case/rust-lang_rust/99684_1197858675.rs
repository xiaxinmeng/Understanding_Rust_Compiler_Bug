rust
#[macro_export]
macro_rules! write {
    ($dst:expr, $($arg:tt)*) => {
        match $dst {
            k#autoref2Φ mut dst => {
                let result = dst.write_fmt($crate::format_args!($($arg)*));
                result
            }
        }
    };
}
