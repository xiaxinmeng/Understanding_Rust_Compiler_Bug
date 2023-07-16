
error[E0275]: overflow evaluating the requirement `_: Sized`
  --> src/lib.rs:11:5
   |
8  | fn call<E: Trait>() {}
   |            ----- required by this bound in `call`
...
11 |     call::<&Struct<_>>();
   |     ^^^^^^^^^^^^^^^^^^
   |
   = help: consider adding a `#![recursion_limit="256"]` attribute to your crate (`playground`)
note: required because of the requirements on the impl of `Trait` for `&Struct<_>`
  --> src/lib.rs:6:13
   |
6  | impl<'a, T> Trait for &'a Struct<T> where &'a T: Trait {}
   |             ^^^^^     ^^^^^^^^^^^^^
   = note: 128 redundant requirements hidden
   = note: required because of the requirements on the impl of `Trait` for `&Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<Struct<_>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>`
