rust
#[macro_export]
macro_rules! call_my_callback {
    () => {
        // Problematic because tagging local_inner_macros would look for
        // callback in this crate.
        call_my_callback_helper! { callback }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! call_my_callback_helper {
    ($callback:ident) => {
        $callback!{}
    };
}
