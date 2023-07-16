
warning: variable does not need to be mutable
 --> src/main.rs:8:13
  |
8 |         let mut foo = String::from("foo");
  |             ----^^^
  |             |
  |             help: remove this `mut`
  |
  = note: #[warn(unused_mut)] on by default)
