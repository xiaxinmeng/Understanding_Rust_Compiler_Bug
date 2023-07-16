
error: generic parameters may not be used in const operations
 --> src/main.rs:3:23
  |
3 |     let _: [u8; sof::<T>()];
  |                       ^ cannot perform const operation using `T`
  |
  = note: type parameters may not be used in const expressions
  = help: use `#![feature(const_generics)]` and `#![feature(const_evaluatable_checked)]` to allow generic const expressions

warning: type parameter `usize` should have an upper camel case name
 --> src/main.rs:1:8
  |
1 | fn sof<usize>() -> usize {}
  |        ^^^^^ help: convert the identifier to upper camel case (notice the capitalization): `Usize`
  |
  = note: `#[warn(non_camel_case_types)]` on by default

error[E0308]: mismatched types
 --> src/main.rs:1:20
  |
1 | fn sof<usize>() -> usize {}
  |    --- -----       ^^^^^ expected type parameter `usize`, found `()`
  |    |   |
  |    |   this type parameter
  |    implicitly returns `()` as its body has no tail or `return` expression
  |
  = note: expected type parameter `usize`
                  found unit type `()`

error: aborting due to 2 previous errors; 1 warning emitted
