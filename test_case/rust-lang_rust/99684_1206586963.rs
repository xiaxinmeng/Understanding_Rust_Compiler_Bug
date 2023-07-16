rust
// key macro rule
($dst:expr, $($arg:tt)*) => ({
    use $crate::ByRef as _;
    match $dst.__by_ref() {
        dst => {
            let result = dst.write_fmt($crate::format_args!($($arg)*));
            result
        }
    }
});
