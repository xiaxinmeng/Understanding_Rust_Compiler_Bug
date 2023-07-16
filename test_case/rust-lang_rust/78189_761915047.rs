
error: generic parameters may not be used in const operations
 --> src/main.rs:8:24
  |
8 |     fn array() -> [(); Self::N];
  |                        ^^^^^^^ cannot perform const operation using `Self`
  |
  = note: type parameters may not be used in const expressions
  = help: use `#![feature(const_generics)]` and `#![feature(const_evaluatable_checked)]` to allow generic const expressions
