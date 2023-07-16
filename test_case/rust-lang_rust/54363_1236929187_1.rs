rust
// main crate, that reexports the macro
macro_rules! the_macro {
    // `$crate` is expanded early it seems, and can be reinterpreted as a path
    // I've tested with `macro_rules!` and this still works if this macro is used from outside
    ($($args:tt)*) => { $crate::the_crate_macros::the_macro!($crate; $($args)*) }
}
