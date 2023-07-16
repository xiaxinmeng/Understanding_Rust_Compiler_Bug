
error[E0581]: return type references an anonymous lifetime which is not constrained by the fn input types
 --> file.rs:6:13
  |
6 | fn foo() -> S<'_, '_> {
  |             ^^^^^^^^^
  |
  = note: lifetimes appearing in an associated type are not considered constrained
