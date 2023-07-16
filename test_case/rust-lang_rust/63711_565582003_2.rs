
error[E0308]: mismatched types
  --> file12.rs:20:13
   |
5  | impl<T: ToString> MyType<T> {
   |      - this type parameter
...
20 |             val
   |             ^^^ expected type parameter `T`, found `u8`
   |
   = note: expected type parameter `T`
                        found type `u8`
   = help: type parameters must be constrained to match other types
   = note: for more information, visit https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters
