
error[E0308]: mismatched types
 --> src/main.rs:6:23
  |
6 |     print_with_string(c)
  |                       ^ expected struct `std::string::String`, found `&str`
help: try using a conversion method
  |
6 |     print_with_string(String::from(c))
  |                       ^^^^^^^^^^^^^ ^
