
error: generic parameters may not be used in const operations
 --> src/lib.rs:2:40
  |
2 |     fn f() -> [u8; std::mem::size_of::<Self>()];
  |                                        ^^^^ cannot perform const operation using `Self`
  |
  = note: type parameters may not be used in const expressions
  = help: use `#![feature(generic_const_exprs)]` to allow generic const expressions

error: could not compile `playground` due to previous error
