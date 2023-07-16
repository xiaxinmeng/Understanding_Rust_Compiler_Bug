rust
#![feature(use_extern_macros, macro_helper_hack)]

#[macro_export(macro_helper_hack)]
macro_rules! noop {
    () => {
        noop_helper!{}
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
