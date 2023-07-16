Rust
error[E0271]: type mismatch resolving `<<T as Trait>::F as futures::Future>::Output == u8`
 --> src/main.rs:5:17
  |
5 | type G<'a, T> = impl Future<Output = u8>;
  |                 ^^^^^^^^^^^^^^^^^^^^^^^^ expected `u8`, found associated type
  |
  = note:         expected type `u8`
          found associated type `<<T as Trait>::F as futures::Future>::Output`
  = help: consider constraining the associated type `<<T as Trait>::F as futures::Future>::Output` to `u8`
  = note: for more information, visit https://doc.rust-lang.org/book/ch19-03-advanced-traits.html

error[E0277]: the trait bound `T: Trait` is not satisfied
 --> src/main.rs:5:17
  |
5 | type G<'a, T> = impl Future<Output = u8>;
  |                 ^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Trait` is not implemented for `T`
  |
help: consider restricting type parameter `T`
  |
5 | type G<'a, T: Trait> = impl Future<Output = u8>;
  |             +++++++
