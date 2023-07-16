rust
error[E0432]: unresolved import `super::NonZeroU8`
 --> ...\test.rs:2:9
  |
2 |     use super::NonZeroU8;
  |         ^^^^^^^^^^^^^^^^^ no `NonZeroU8` in the root

error[E0433]: failed to resolve: use of undeclared type `NonZeroU8`
  --> ...\test.rs:12:13
   |
12 |     let y = NonZeroU8::new(2).unwrap();
   |             ^^^^^^^^^ not found in this scope
   |
help: consider importing one of these items
   |
9  | use std::num::NonZeroU8;
   |
9  | use core::num::NonZeroU8;
   |
