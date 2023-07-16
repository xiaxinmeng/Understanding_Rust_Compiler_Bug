
error[E0308]: mismatched types
 --> src/main.rs:1:23
  |
1 | fn foo(bar: usize) -> usize {
  |    ---                ^^^^^ expected usize, found ()
  |    |
  |    this function's body doesn't return
  |
  = note: expected type `usize`
             found type `()`
