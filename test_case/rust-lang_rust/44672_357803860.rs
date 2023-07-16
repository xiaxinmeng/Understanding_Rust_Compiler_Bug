
error[E0308]: mismatched types
 --> src/main.rs:5:9
  |
5 |     foo(s);
  |         ^
  |         |
  |         expected struct `std::string::String`, found &str
  |         help: try using a conversion method: `s.to_string()`
  |
  = note: expected type `std::string::String`
             found type `&str`
