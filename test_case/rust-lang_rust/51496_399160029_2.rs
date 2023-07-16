rust
#[macro_export(local_inner_macros)]
macro_rules! call_my_callback {
    () => {
        call_my_callback_with_helpers! { call_my_callback_helper }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! call_my_callback_with_helpers {
    ($helper:ident) => {
        $helper! { callback }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! call_my_callback_helper {
    ($callback:ident) => {
        $callback!{}
    };
}
