
error[E0308]: mismatched types
 --> src/lib.rs:2:30
  |
2 |     let f: Box<F> = Box::new(|| ());
  |                              ^^^^^ expected type parameter, found closure
  |
  = note: expected type `F`
             found type `[closure@src/lib.rs:2:30: 2:35]`
  = help: type parameters must be constrained to match other types
  = note: for more information, visit https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters
