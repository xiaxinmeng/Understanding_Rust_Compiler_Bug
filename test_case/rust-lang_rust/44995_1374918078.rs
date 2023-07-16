
error[[E0405]](https://doc.rust-lang.org/stable/error-index.html#E0405): cannot find trait `Add` in this scope
 --> src/main.rs:4:6
  |
4 | impl Add for X {} // trigger unused import
  |      ^^^ not found in this scope
  |
help: consider importing one of these items
  |
1 | [use core::ops::Add;](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018#)
  |
1 | [use nom::lib::std::ops::Add;](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018#)
  |
1 | [use pin_utils::core_reexport::ops::Add;](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018#)
  |
1 | [use std::ops::Add;](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018#)
  |
