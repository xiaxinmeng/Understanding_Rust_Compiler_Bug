
error[E0720]: opaque type expands to a recursive type
 --> src/main.rs:6:24
  |
6 | fn f(x: impl Trait) -> impl Trait {
  |                        ^^^^^^^^^^ expands to a recursive type
  |
  = note: type resolves to itself
