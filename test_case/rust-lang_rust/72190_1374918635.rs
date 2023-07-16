
error: generic parameters may not be used in const operations
 --> src/lib.rs:2:24
  |
2 |     fn array() -> [u8; Self::SIZE];
  |                        ^^^^^^^^^^ cannot perform const operation using `Self`
  |
  = note: type parameters may not be used in const expressions
  = help: use `#![feature(generic_const_exprs)]` to allow generic const expressions
