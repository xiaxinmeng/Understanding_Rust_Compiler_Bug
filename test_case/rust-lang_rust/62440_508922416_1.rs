
error[E0308]: mismatched types
 --> src/lib.rs:3:18
  |
3 | const TEST: () = foo;
  |                  ^^^ expected (), found fn item
  |
  = note: expected type `()`
             found type `fn() {foo}`
