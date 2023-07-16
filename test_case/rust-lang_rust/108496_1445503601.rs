
warning: `--x` could be misinterpreted as pre-decrement by C programmers, is usually a no-op
 --> e.rs:3:9
  |
3 | let _ = --i;
  |         ^^^
  |
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#double_neg
  = note: `#[warn(clippy::double_neg)]` on by default
