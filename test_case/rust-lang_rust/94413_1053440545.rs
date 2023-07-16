plain

   Doc-tests core

running 3745 tests
..................................i..i......iiiiii......................................F.F......... 100/3745
..................................................................................ii................ 300/3745
.................................................................................................... 400/3745
.....................................i.............................................................. 500/3745
.................................................................................................... 600/3745
---
---- src/array/mod.rs - array::[T;N]::zip_map (line 515) stdout ----
error: an inner attribute is not permitted in this context
 --> src/array/mod.rs:517:1
  |
5 | #![feature(array_zip, array_zip_map)]
  |
  |
  = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files
  = note: outer attributes, like `#[test]`, annotate the item following them
error: crate-level attribute should be in the root module
 --> src/array/mod.rs:517:1
  |
  |
5 | #![feature(array_zip, array_zip_map)]
  |
note: the lint level is defined here
 --> src/array/mod.rs:513:9
  |
  |
1 | #![deny(warnings)]
  |         ^^^^^^^^
  = note: `#[deny(unused_attributes)]` implied by `#[deny(warnings)]`

error[E0658]: use of unstable library feature 'array_zip'
   |
   |
11 | let output1 = x.zip(y).map(|(a, b)| op(a, b));
   |
   = note: see issue #80094 <https://github.com/rust-lang/rust/issues/80094> for more information
   = note: see issue #80094 <https://github.com/rust-lang/rust/issues/80094> for more information
   = help: add `#![feature(array_zip)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'array_zip_map'
   |
   |
12 | let output2 = x.zip_map(y, op);
   |
   |
   = help: add `#![feature(array_zip_map)]` to the crate attributes to enable
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.
---- src/array/mod.rs - array::[T;N]::zip_map_assign (line 579) stdout ----
error: an inner attribute is not permitted in this context
 --> src/array/mod.rs:581:1
  |
5 | #![feature(array_zip_map)]
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files
  = note: outer attributes, like `#[test]`, annotate the item following them
error: unused import: `std::ops::Add`
 --> src/array/mod.rs:580:5
  |
4 | use std::ops::Add;
---
1 | #![deny(warnings)]
  |         ^^^^^^^^
  = note: `#[deny(unused_attributes)]` implied by `#[deny(warnings)]`

error[E0658]: use of unstable library feature 'array_zip_map'
   |
   |
10 | x.zip_map_assign(y, i32::add_assign);
   |
   |
   = help: add `#![feature(array_zip_map)]` to the crate attributes to enable

error[E0599]: no function or associated item named `add_assign` found for type `i32` in the current scope
   |
   |
10 | x.zip_map_assign(y, i32::add_assign);
   |                          ^^^^^^^^^^ function or associated item not found in `i32`
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
   |
3  | use std::ops::AddAssign;
