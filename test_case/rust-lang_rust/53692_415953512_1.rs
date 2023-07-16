
error[E0308]: mismatched types
 --> src/main.rs:3:26
  |
3 |     let string: String = s.clone();
  |                          ^^^^^^^^^
  |                          |
  |                          expected struct `std::string::String`, found &str
  |                          help: try using a conversion method: `s.clone().to_string()`
  |
  = note: expected type `std::string::String`
             found type `&str`
