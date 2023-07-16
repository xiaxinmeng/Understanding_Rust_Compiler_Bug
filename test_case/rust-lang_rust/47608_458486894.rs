rust
error[E0658]: The attribute `serde` is currently unknown to the compiler and may have meaning added to it in the future (see issue #29642)
 --> src/main.rs:2:3
  |
2 | #[serde(untagged)]
  |   ^^^^^
  |
  = help: add #![feature(custom_attribute)] to the crate attributes to enable

error: cannot find derive macro `Serialize` in this scope
 --> src/main.rs:1:10
  |
1 | #[derive(Serialize)]
  |          ^^^^^^^^^
