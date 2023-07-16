
   Compiling util_proc v0.1.0 ( .. util_proc)
   Compiling util v0.1.0 ( .. util)
error[E0759]: `self` has an anonymous lifetime `'_` but it needs to satisfy a `'static` lifetime requirement
   --> src/lib.rs:817:18
    |
817 |         #[derive(EnumError, Debug)]
    |                  ^^^^^^^^^
    |                  |
    |                  this data with an anonymous lifetime `'_`...
    |                  ...is captured and required to live as long as `'static` here
    |
    = note: this error originates in the derive macro `EnumError` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider changing the trait object's explicit `'static` bound to the lifetime of argument `self`
    |
817 |         #[derive('_, Debug)]
    |                  ~~
help: alternatively, add an explicit `'static` bound to this reference
    |
817 |         #[derive(&'static MyError, Debug)]
    |                  ~~~~~~~~~~~~~~~~
