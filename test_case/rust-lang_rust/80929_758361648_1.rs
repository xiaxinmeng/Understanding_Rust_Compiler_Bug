
error[E0308]: mismatched types
 --> src/main.rs:8:11
  |
8 |     Foo { bar };
  |           ^^^ expected `i32`, found `()`
  |
  = note: expected fn pointer `fn() -> i32`
                found fn item `fn() {bar}`
