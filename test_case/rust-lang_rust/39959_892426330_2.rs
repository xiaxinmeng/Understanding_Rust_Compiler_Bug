
error[E0275]: overflow evaluating the requirement `_: Sized`
 --> src/lib.rs:7:5
  |
4 | fn call<E: Trait>() {}
  |            ----- required by this bound in `call`
...
7 |     call::<&Vec<_>>();
  |     ^^^^^^^^^^^^^^^
  |
  = help: consider adding a `#![recursion_limit="256"]` attribute to your crate (`playground`)
note: required because of the requirements on the impl of `Trait` for `&Vec<_>`
 --> src/lib.rs:2:13
  |
2 | impl<'a, T> Trait for &'a Vec<T> where &'a T: Trait {}
  |             ^^^^^     ^^^^^^^^^^
  = note: 128 redundant requirements hidden
  = note: required because of the requirements on the impl of `Trait` for `&Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<_>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>`
