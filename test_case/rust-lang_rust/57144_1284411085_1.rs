
error: generic parameters may not be used in const operations
 --> src/lib.rs:5:31
  |
5 |         bytes: [u8; size_of::<T>()],
  |                               ^ cannot perform const operation using `T`
  |
  = note: type parameters may not be used in const expressions
  = help: use `#![feature(generic_const_exprs)]` to allow generic const expressions

error: constant expression depends on a generic parameter
  --> src/lib.rs:10:20
   |
10 |         bytes: [0; size_of::<T>()],
   |                    ^^^^^^^^^^^^^^
   |
   = note: this may fail depending on what value the parameter takes
