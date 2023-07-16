
   Compiling playground v0.0.1 (/playground)
error: lifetime parameter `'a` only used once
 --> src/lib.rs:4:12
  |
3 | #[derive(Eq)]
  |          -- ...is used only here
4 | struct Foo<'a, T> {
  |            ^^ this lifetime...
  |
note: lint level defined here
 --> src/lib.rs:1:8
  |
1 | #[deny(single_use_lifetimes)]
