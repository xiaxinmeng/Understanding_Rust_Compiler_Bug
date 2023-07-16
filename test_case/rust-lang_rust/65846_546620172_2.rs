rust
error: expected type, found `0`
 --> src/lib.rs:6:13
  |
6 | mac!(label: 0, MARKER);
  |           - ^ expected type
  |           |
  |           tried to parse a type due to this type ascription
  |
  = note: `#![feature(type_ascription)]` lets you annotate an expression with a type: `<expr>: <type>`
  = note: for more information, see https://github.com/rust-lang/rust/issues/23416
