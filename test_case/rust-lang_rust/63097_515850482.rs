text
error[E0072]: recursive type `S` has infinite size
 --> src/lib.rs:1:1
  |
1 | enum S {
  | ^^^^^^ recursive type has infinite size
2 |     T(S)
  |       - recursive without indirection
  |
  = help: insert indirection (e.g., a `Box`, `Rc`, or `&`) at some point to make `S` representable
