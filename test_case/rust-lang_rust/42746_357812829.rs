
error[E0308]: mismatched types
 --> src/main.rs:2:14
  |
2 | let i: i32 = Some(42);
  |              ^^^^^^^^ expected i32, found enum `std::option::Option`
  |
  = note: expected type `i32`
             found type `std::option::Option<{integer}>`

error: aborting due to previous error
