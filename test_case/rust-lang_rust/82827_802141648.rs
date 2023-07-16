
error[E0658]: `let` expressions in this position are experimental
 --> src/main.rs:4:7
  |
4 |   if (let Some(y) = x) {
  |       ^^^^^^^^^^^^^^^
  |
  = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
  = help: add `#![feature(let_chains)]` to the crate attributes to enable
  = help: you can write `matches!(<expr>, <pattern>)` instead of `let <pattern> = <expr>`

error: invalid parentheses around `let` expression in `if let`
 --> src/main.rs:4:6
  |
4 |   if (let Some(y) = x) {
  |      ^               ^
  |
help: `if let` needs to be written without parentheses
  |
4 |   if let Some(y) = x {
  |     --             --
