
error[E0308]: mismatched types
  --> src/main.rs:16:22
   |
11 | impl<T> MyOption<T> {
   |      - this type parameter
...
16 |                 Some(value)
   |                      ^^^^^ expected type parameter `T`, found `&mut T`
   |
   = note: expected type parameter `T`
           found mutable reference `&mut T`
   = help: type parameters must be constrained to match other types
   = note: for more information, visit https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters
