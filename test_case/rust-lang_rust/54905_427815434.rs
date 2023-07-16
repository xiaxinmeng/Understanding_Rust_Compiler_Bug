
warning: This generic shadows the built-in type `u16`
 --> src/main.rs:4:6
  |
4 | impl<u16> S<u16> {
  |      ^^^
  |
  = note: #[warn(clippy::builtin_type_shadow)] on by default
  = help: for further information visit https://rust-lang-nursery.github.io/rust-clippy/v0.0.212/index.html#builtin_type_shadow
