
error[E0308]: mismatched types
  --> src/lib.rs:20:13
   |
20 |             val
   |             ^^^ expected type parameter, found u8
   |
   = note: expected type `T`
              found type `u8`
   = help: type parameters must be constrained to match other types
   = note: for more information, visit https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters
