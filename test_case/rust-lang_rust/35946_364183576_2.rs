
error[E0277]: the trait bound `std::option::NoneError: std::convert::From<std::io::Error>` is not satisfied
 --> src/main.rs:2:8
  |
2 |     Ok(std::fs::File::open("foo")?)
  |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::From<std::io::Error>` is not implemented for `std::option::NoneError`
  |
  = note: required by `std::convert::From::from`

error[E0308]: mismatched types
 --> src/main.rs:2:5
  |
1 | fn foo() -> Option<()> {
  |             ---------- expected `std::option::Option<()>` because of return type
2 |     Ok(std::fs::File::open("foo")?)
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `std::option::Option`, found enum `std::result::Result`
  |
  = note: expected type `std::option::Option<()>`
             found type `std::result::Result<std::fs::File, _>`
