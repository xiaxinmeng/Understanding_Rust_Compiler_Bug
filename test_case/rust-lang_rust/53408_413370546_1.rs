rust
error: import from `log` is ambiguous
 --> src/main.rs:3:5
  |
3 | use log::{self as log};
  |     ^^^   ----------- could also refer to `self::log`
  |     |
  |     could refer to external crate `::log`
  |
  = help: write `::log` or `self::log` explicitly instead
  = note: relative `use` paths enabled by `#![feature(uniform_paths)]`
