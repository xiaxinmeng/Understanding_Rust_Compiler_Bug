rust
error: import from `hex` is ambiguous
 --> src/main.rs:3:5
  |
3 | use hex;
  |     ^^^
  |     |
  |     could refer to external crate `::hex`
  |     could also refer to `self::hex`
  |
  = help: write `::hex` or `self::hex` explicitly instead
  = note: relative `use` paths enabled by `#![feature(uniform_paths)]`
