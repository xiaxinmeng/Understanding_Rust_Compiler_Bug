
error[E0308]: mismatched types
 --> src/main.rs:2:22
  |
2 | let i: Option<i32> = 42;
  |                      ^^
  |                      |
  |                      expected enum `std::option::Option`, found integral variable
  |                      help: try using a variant of the expected type: `Some(42)`
  |
  = note: expected type `std::option::Option<i32>`
             found type `{integer}`

error[E0308]: mismatched types
 --> src/main.rs:3:22
  |
3 | let i: Option<i32> = 42i32;
  |                      ^^^^^
  |                      |
  |                      expected enum `std::option::Option`, found i32
  |                      help: try using a variant of the expected type: `Some(42i32)`
  |
  = note: expected type `std::option::Option<i32>`
             found type `i32`

error: aborting due to 2 previous errors
