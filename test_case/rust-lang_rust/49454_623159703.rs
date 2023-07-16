rust
error[E0308]: mismatched types
  --> src/main.rs:22:45
   |
22 |     dependencies.into_iter().find(|&d| d == dep_name);
   |                                             ^^^^^^^^ expected struct `InternedString`, found `&str`
