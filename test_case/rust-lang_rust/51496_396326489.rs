rust
#![feature(use_extern_macros)]

#[macro_export]
macro_rules! noop {
    () => {
        $crate::noop_helper!{}
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! noop_helper {
    () => {};
}

pub fn call() {
    noop!{}
}
