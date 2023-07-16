
error[E0308]: mismatched types
  --> src/main.rs:33:9
   |
28 |     fn map<F, O>(self, mut f: F) -> <Self::HKT as TypeToType<O>>::Output
   |                                     ------------------------------------ expected `<Vec_ as TypeToType<O>>::Output` because of return type
...
33 |         r
   |         ^ expected associated type, found struct `std::vec::Vec`
   |
   = note: expected associated type `<Vec_ as TypeToType<O>>::Output`
                       found struct `std::vec::Vec<O>`
   = note: consider constraining the associated type `<Vec_ as TypeToType<O>>::Output` to `std::vec::Vec<O>` or calling a method that returns `<Vec_ as TypeToType<O>>::Output`
   = note: for more information, visit https://doc.rust-lang.org/book/ch19-03-advanced-traits.html
