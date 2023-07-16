
error[E0308]: mismatched types
 --> src\main.rs:8:5
  |
3 | fn test<Iter, X, Y>(x: X) -> Y
  |               -  -           - expected `Y` because of return type
  |               |  |
  |               |  expected type parameter
  |               found type parameter
...
8 |     x
  |     ^ expected type parameter `Y`, found type parameter `X`
  |
  = note: expected type parameter `Y`
             found type parameter `X`
  = note: a type parameter was expected, but a different one was found; you might be missing a type parameter or trait bound
  = note: for more information, visit https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters

