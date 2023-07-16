rust
    #[macro_export]
    #[allow_internal_unstable]
    macro_rules! println {
        ($($arg:tt)*) => {
            $crate::io::_print(format_argsln!($($arg)*))
        };
    }
    