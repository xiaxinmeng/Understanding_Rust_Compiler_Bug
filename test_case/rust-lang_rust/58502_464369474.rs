rust
#![allow(dead_code)]

#[macro_export]
macro_rules! wrapper {
    ($($tokens:tt)*) => { $( $tokens )* }
}

#[macro_export]
macro_rules! struct_wrapper {
    ($($tokens:tt)*) => { struct $($tokens)* {} }
}
