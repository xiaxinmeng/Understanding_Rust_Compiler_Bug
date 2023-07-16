
error[E0308]: mismatched types
 --> src/lib.rs:3:5
  |
3 |     10
  |     ^^ expected (), found integer
  |
  = note: expected type `()`
             found type `{integer}`

error[E0308]: mismatched types
 --> src/lib.rs:5:5
  |
5 |     20
  |     ^^ expected (), found integer
  |
  = note: expected type `()`
             found type `{integer}`

error[E0308]: mismatched types
 --> src/lib.rs:1:20
  |
1 | fn example_fn() -> i32 {
  |    ----------      ^^^ expected i32, found ()
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
  |
  = note: expected type `i32`
             found type `()`
