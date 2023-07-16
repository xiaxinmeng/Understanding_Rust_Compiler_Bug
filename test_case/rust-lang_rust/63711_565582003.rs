
error[E0308]: mismatched types
  --> file12.rs:16:9
   |
15 |     fn foo(self) -> Result<usize, TryIntoError> {
   |                     --------------------------- expected `std::result::Result<usize, TryIntoError>` because of return type
16 |         TryInto::try_into(self)
   |         ^^^^^^^^^^^^^^^^^^^^^^^ expected struct `TryIntoError`, found associated type
   |
   = note: expected enum `std::result::Result<_, TryIntoError>`
              found enum `std::result::Result<_, <Self as std::convert::TryInto<usize>>::Error>`
   = note: consider constraining the associated type `<Self as std::convert::TryInto<usize>>::Error` to `TryIntoError`
   = note: for more information, visit https://doc.rust-lang.org/book/ch19-03-advanced-traits.html
