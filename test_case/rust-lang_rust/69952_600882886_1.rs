bash
error: lifetime parameter `'a` only used once
 --> src/main.rs:4:12
  |
3 | #[derive(Eq)]
  |          -- ...is used only here
4 | struct Foo<'a, T> {
  |            ^^ this lifetime...
  |
note: the lint level is defined here
 --> src/main.rs:1:8
  |
1 | #[deny(single_use_lifetimes)]
  |        ^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error

error: could not compile `bisectit`.

To learn more, run the command again with --verbose.
