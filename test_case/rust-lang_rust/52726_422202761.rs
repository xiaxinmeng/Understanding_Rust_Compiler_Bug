rust
#[macro_export]
#[doc(hidden)]
macro_rules! crossbeam_unreachable {
    ($($args:tt)*) => {
        unreachable! { $($args)* }
    };
}
