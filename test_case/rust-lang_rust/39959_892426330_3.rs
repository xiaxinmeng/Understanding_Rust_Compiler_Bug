
error[E0275]: overflow evaluating the requirement `_: Sized`
  --> src/main.rs:20:9
   |
20 |     wtr.write::<&[_]>(42).unwrap();
   |         ^^^^^
   |
   = help: consider adding a `#![recursion_limit="20"]` attribute to your crate (`playground`)
note: required because of the requirements on the impl of `CanEncode` for `&[_]`
  --> src/main.rs:12:13
   |
12 | impl<'a, T> CanEncode for &'a [T] where
   |             ^^^^^^^^^     ^^^^^^^
   = note: 10 redundant requirements hidden
   = note: required because of the requirements on the impl of `CanEncode` for `&[[[[[[[[[[[_]]]]]]]]]]]`
