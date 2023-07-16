
error[E0308]: mismatched types
 --> src/main.rs:7:21
  |
7 |     let s: String = foo("hi");
  |                     ^^^^^^^^^
  |                     |
  |                     expected struct `std::string::String`, found &str
  |                     help: try using a conversion method: `foo("hi").to_string()`
  |
  = note: expected type `std::string::String`
             found type `&str`

error: aborting due to previous error
