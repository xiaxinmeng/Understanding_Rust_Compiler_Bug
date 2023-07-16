
error[E0425]: cannot find value `x` in this scope
 --> src/main.rs:3:13
  |
3 |     if Some(x) = foo {}
  |             ^ not found in this scope

error[E0308]: mismatched types
 --> src/main.rs:3:8
  |
3 |     if Some(x) = foo {}
  |        ^^^^^^^^^^^^^
  |        |
  |        help: did you mean to compare equality?: `Some(x) == foo`
  |        expected bool, found ()
  |
  = note: expected type `bool`
             found type `()`
