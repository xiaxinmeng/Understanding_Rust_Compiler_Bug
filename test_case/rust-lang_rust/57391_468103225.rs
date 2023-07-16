console
warning: the operation is ineffective. Consider reducing it to `Duration::SECOND`
 --> src/main.rs:7:19
  |
7 |     thread::sleep(1 * Duration::SECOND);
  |                   ^^^^^^^^^^^^^^^^^^^^
  |
  = note: #[warn(clippy::identity_op)] on by default
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#identity_op
