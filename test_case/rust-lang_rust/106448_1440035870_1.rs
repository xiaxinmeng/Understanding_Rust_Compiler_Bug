
warning: unneeded `return` statement
 --> src/lib.rs:5:27
  |
5 |               // side effect ...
  |  _______________________________^
6 | |             return;
  | |__________________^
  |
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_return
  = note: `#[warn(clippy::needless_return)]` on by default
  = help: remove `return`
