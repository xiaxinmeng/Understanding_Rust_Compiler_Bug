
error[E0308]: mismatched types
 --> src/main.rs:2:5
  |
2 |     match () { _ => true } && true;
  |     ^^^^^^^^^^^^^^^^^^^^^^- help: consider using a semicolon here
  |     |
  |     expected `()`, found `bool`

error: aborting due to previous error
