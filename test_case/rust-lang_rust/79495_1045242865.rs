`
error: generic parameters may not be used in const operations
 --> out.rs:9:35
  |
9 |     Some(T) = std::mem::size_of::<T>(),
  |                                   ^ cannot perform const operation using `T`
  |
  = note: type parameters may not be used in const expressions
  = help: use `#![feature(generic_const_exprs)]` to allow generic const expressions
