
error[E0658]: `let` expressions in this position are unstable
 --> src/main.rs:2:8
  |
2 |     if let x = 3 && true {}
  |        ^^^^^^^^^
  |
  = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
  = help: add `#![feature(let_chains)]` to the crate attributes to enable

For more information about this error, try `rustc --explain E0658`.
error: could not compile `playground` due to previous error
