
error[E0581]: return type references an anonymous lifetime which is not constrained by the fn input types
 --> src/main.rs:1:15
  |
1 | fn f(_: X) -> X {
  |               ^
  |
  = note: lifetimes appearing in an associated type are not considered constrained
