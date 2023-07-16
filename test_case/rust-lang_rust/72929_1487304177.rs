
error: generic parameters may not be used in const operations
 --> src/lib.rs:3:23
  |
3 |     const ARRAY: [u8; Self::N];
  |                       ^^^^^^^ cannot perform const operation using `Self`
  |
  = note: type parameters may not be used in const expressions
  = help: use `#![feature(generic_const_exprs)]` to allow generic const expressions

error: generic parameters may not be used in const operations
 --> src/lib.rs:7:21
  |
7 |     fn f() -> [u8; <T as A>::N];
  |                     ^ cannot perform const operation using `T`
  |
  = note: type parameters may not be used in const expressions
  = help: use `#![feature(generic_const_exprs)]` to allow generic const expressions

error: could not compile `playground` (lib) due to 2 previous errors
