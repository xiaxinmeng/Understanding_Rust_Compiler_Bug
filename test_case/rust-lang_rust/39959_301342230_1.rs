
rustc 1.17.0 (56124baa9 2017-04-24)
error[E0275]: overflow evaluating the requirement `_: std::marker::Sized`
  --> <anon>:20:9
   |
20 |     wtr.write::<&[_]>(42).unwrap();
   |         ^^^^^
   |
   = help: consider adding a `#![recursion_limit="20"]` attribute to your crate
   = note: required because of the requirements on the impl of `CanEncode` for `&[_]`
   = note: required because of the requirements on the impl of `CanEncode` for `&[[_]]`
   = note: required because of the requirements on the impl of `CanEncode` for `&[[[_]]]`
   = note: required because of the requirements on the impl of `CanEncode` for `&[[[[_]]]]`
   = note: required because of the requirements on the impl of `CanEncode` for `&[[[[[_]]]]]`
   = note: required because of the requirements on the impl of `CanEncode` for `&[[[[[[_]]]]]]`
   = note: required because of the requirements on the impl of `CanEncode` for `&[[[[[[[_]]]]]]]`
   = note: required because of the requirements on the impl of `CanEncode` for `&[[[[[[[[_]]]]]]]]`
   = note: required because of the requirements on the impl of `CanEncode` for `&[[[[[[[[[_]]]]]]]]]`
   = note: required because of the requirements on the impl of `CanEncode` for `&[[[[[[[[[[_]]]]]]]]]]`

error: aborting due to previous error

Compilation failed.
