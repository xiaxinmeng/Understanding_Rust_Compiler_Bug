
error[E0277]: the size for values of type `str` cannot be known at compilation time
   --> src/main.rs:12:29
    |
12  |                 (arg0,) => [::std::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt)],
    |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
    |
    = help: the trait `Sized` is not implemented for `str`
