
error[E0658]: usage of qualified paths in this context is experimental
  --> src/lib.rs:47:21
   |
47 |                 Err(<$foo as Foo>::Error::Corge(_)) => (),
   |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
72 |     test_foo_impl!(Grault);
   |     ----------------------- in this macro invocation
   |
   = note: see issue #80080 <https://github.com/rust-lang/rust/issues/80080> for more information
   = help: add `#![feature(more_qualified_paths)]` to the crate attributes to enable
   = note: this error originates in the macro `test_foo_impl` (in Nightly builds, run with -Z macro-backtrace for more info)

