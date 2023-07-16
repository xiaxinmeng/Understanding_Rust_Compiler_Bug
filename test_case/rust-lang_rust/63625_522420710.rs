rust
#![feature(const_fn, const_panic)]

#[doc(hidden)]
pub use core as core_;

#[macro_export]
macro_rules! static_assert {
    ($cond:expr) => {
        $crate::static_assert!($cond, concat!("assertion failed: ", stringify!($cond)));
    };
    ($cond:expr, $($t:tt)+) => {
        #[forbid(const_err)]
        const _: () = {
            if !$cond {
                $crate::core_::panic!($($t)+)
            }
        };
    };
}

//static_assert!(1 + 1 == 3); // not actually possible yet
