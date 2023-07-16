
error[E0308]: mismatched types
 --> test.rs:4:5
  |
4 |     stdin().bytes().next()
  |     ^^^^^^^^^^^^^^^^^^^^^^- help: did you mean to add a semicolon here?: `;`
  |     |
  |     expected (), found enum `std::option::Option`
  |
  = note: expected type `()`
             found type `std::option::Option<std::result::Result<u8, std::io::Error>>`

error: aborting due to previous error
