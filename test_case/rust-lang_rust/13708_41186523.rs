 rust
macro_rules! log_with_location(
    ($lvl:expr, $fmt $($arg:tt)*) => ({
        log!($lvl, concat!("{} {}:{}", $fmt), ::log::level_to_str($lvl), file!(), line!() $($arg)*)
    })
)
