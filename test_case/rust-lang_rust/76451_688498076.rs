
error[E0261]: use of undeclared lifetime name `'a`
 --> src/lib.rs:6:19
  |
6 |     type Assoc = &'a i32;
  |                   ^^ undeclared lifetime
  |
  = help: if you want to experiment with in-band lifetime bindings, add `#![feature(in_band_lifetimes)]` to the crate attributes
help: consider introducing lifetime `'a` here
  |
5 | impl<'a> HasAssoc for i32 {
  |     ^^^^
help: consider introducing lifetime `'a` here
  |
6 |     type Assoc<'a> = &'a i32;
  |               ^^^^
