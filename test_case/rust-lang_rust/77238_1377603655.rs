
error[[E0425]](https://doc.rust-lang.org/nightly/error-index.html#E0425): cannot find value `foo` in this scope
 --> src/main.rs:2:18
  |
2 |     if Some(3) = foo {}
  |                  ^^^ not found in this scope

error[[E0070]](https://doc.rust-lang.org/nightly/error-index.html#E0070): invalid left-hand side of assignment
 --> src/main.rs:2:16
  |
2 |     if Some(3) = foo {}
  |             -  ^
  |             |
  |             cannot assign to this expression

error[[E0308]](https://doc.rust-lang.org/nightly/error-index.html#E0308): mismatched types
 --> src/main.rs:2:8
  |
2 |     if Some(3) = foo {}
  |        ^^^^^^^^^^^^^ expected `bool`, found `()`
  |
help: consider adding `let`
  |
2 |     if let Some(3) = foo {}
  |        +++
