rust
error[E0308]: mismatched types
 --> ./p/n.rs:7:9
  |
7 |     bar("hello".to_string() + "world");
  |     --- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |     |   |
  |     |   expected `&str`, found struct `String`
  |     |   help: consider borrowing here: `&("hello".to_string() + "world")`
  |     arguments to this function are incorrect
  |
note: function defined here
 --> ./p/n.rs:2:4
  |
2 | fn bar(val: &str) {
  |    ^^^ ---------

error: aborting due to previous error
