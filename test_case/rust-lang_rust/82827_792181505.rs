
error[E0658]: `let` expressions in this position are experimental
 --> f10.rs:4:9
  |
4 |     if (let Some(x) = y) {
  |         ^^^^^^^^^^^^^^^
  |
  = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
  = help: add `#![feature(let_chains)]` to the crate attributes to enable

error: invalid parentheses around `let` expression in `if let`
 --> f10.rs:4:8
  |
4 |     if (let Some(x) = y) {
  |        ^               ^
  |
  = note: only supported directly without parentheses in conditions of `if`- and `while`-expressions, as well as in `let` chains within parentheses
help: `if let` needs to be written without parentheses
  |
4 |     if let Some(x) = y {
  |       --             --

error: aborting due to 2 previous errors
