rust
warning: `--x` could be misinterpreted as pre-decrement by C programmers, is usually a no-op
 --> src/main.rs:3:14
  |
3 |     let _b = --a;
  |              ^^^
  |
  = note: `#[warn(clippy::double_neg)]` on by default
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#double_neg
