
error[E0720]: opaque type expands to a recursive type
 --> src/lib.rs:1:17
  |
1 | fn bar() -> Vec<impl Copy> {
  |                 ^^^^^^^^^ expands to a recursive type
  |
  = note: type resolves to itself
