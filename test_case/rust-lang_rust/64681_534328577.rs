
error[E0308]: mismatched types
 --> src/lib.rs:9:5
  |
4 | fn f<S, R>(rng: &mut R) -> bool
  |                            ---- expected `bool` because of return type
...
9 |     rng.gen()
  |     ^^^^^^^^^ expected bool, found type parameter
  |
  = note: expected type `bool`
             found type `S`
  = help: type parameters must be constrained to match other types
  = note: for more information, visit https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters
