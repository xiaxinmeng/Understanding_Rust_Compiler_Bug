
error[E0658]: `let` expressions in this position are experimental
 --> f10.rs:4:9
  |
4 |     if (let Some(x) = y) {
  |         ^^^^^^^^^^^^^^^
  |
  = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
  = help: add `#![feature(let_chains)]` to the crate attributes to enable
