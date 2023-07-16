
error[E0308]: mismatched types
 --> src/main.rs:2:21
  |
2 |     let x: String = "";
  |                     ^^
  |                     |
  |                     expected struct `std::string::String`, found reference
  |                     help: try using a conversion method: `"".to_string()`
  |
  = note: expected type `std::string::String`
             found type `&'static str`
