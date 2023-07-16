rust
error E0405: cannot find trait `Error` in this scope
 --> src/main.rs:1:33
  |
1 | fn main() -> Result<(), Box<dyn Error>> { Ok(()) }
  |                                 ^^^^^ not found in this scope
  |
help: consider importing one of these items
  |
1 | use core::error::Error;
