
    ($dst:expr, $($arg:tt)*) => { 
        $dst.write_fmt($crate::format_args!($($arg)*))
    };
