rust
error[E0658]: attributes on function parameters are unstable
 --> src/lib.rs:4:12
  |
4 |     fn _f1(#[allow(lint)] p1: u8) {}
  |            ^^^^^^^^^^^^^^
  |
  = note: for more information, see https://github.com/rust-lang/rust/issues/60406
