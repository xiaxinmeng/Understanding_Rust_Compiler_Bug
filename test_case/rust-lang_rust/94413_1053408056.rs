plain
  |
5 | #![feature(array_zip_map)]
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files
  = note: outer attributes, like `#[test]`, annotate the item following them
error: crate-level attribute should be in the root module
 --> src/array/mod.rs:517:1
  |
5 | #![feature(array_zip_map)]
---
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
---- src/array/mod.rs - array::[T;N]::zip_map_assign (line 578) stdout ----
error: an inner attribute is not permitted in this context
 --> src/array/mod.rs:580:1
  |
5 | #![feature(array_zip_map)]
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files
  = note: outer attributes, like `#[test]`, annotate the item following them
error: crate-level attribute should be in the root module
 --> src/array/mod.rs:580:1
  |
5 | #![feature(array_zip_map)]
---
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
