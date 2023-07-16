
error[E0275]: overflow evaluating the requirement `&[u8]: X`
   |
   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`filter_list`)
note: required because of the requirements on the impl of `X` for `&&[u8]`
  --> examples/filter-list.rs:17:21
   |
17 | impl<T: X + ?Sized> X for &T {
   |                     ^     ^^
   = note: 128 redundant requirements hidden
   = note: required because of the requirements on the impl of `X` for `&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&[u8]`
