
error[E0369]: cannot add `&str` to `&str`
 --> src/main.rs:4:21
  |
4 |     let addr = host + port;
  |                ---- ^ ---- &str
  |                |    |
  |                |    `+` cannot be used to concatenate two `&str` strings
  |                &str
  |
help: `to_owned()` can be used to create an owned `String` from a string reference. String concatenation appends the string on the right to the string on the left and may require reallocation. This requires ownership of the string on the left
  |
4 |     let addr = host.to_owned() + port;
  |                ^^^^^^^^^^^^^^^
