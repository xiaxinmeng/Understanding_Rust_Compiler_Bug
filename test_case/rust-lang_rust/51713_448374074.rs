rust
#[macro_export]
macro_rules! unreachable_unchecked {
    () => ({
        #[cfg(debug_assertions)]
        panic!("internal error: entered unreachable code")
        #[cfg(not(debug_assertions))]
        std::hint::unreachable_unchecked()
    });
    ($msg:expr) => ({
        unreachable_unchecked!("{}", $msg)
    });
    ($msg:expr,) => ({
        unreachable_unchecked!($msg)
    });
    ($fmt:expr, $($arg:tt)*) => ({
        #[cfg(debug_assertions)]
        panic!(concat!("internal error: entered unreachable code: ", $fmt), $($arg)*)
        #[cfg(not(debug_assertions))]
        std::hint::unreachable_unchecked()
    });
}
