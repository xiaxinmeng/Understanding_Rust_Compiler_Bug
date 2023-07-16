
error[E0013]: constants cannot refer to statics, use a constant instead
 --> src/lib.rs:2:22
  |
2 | const FOOPTR: &i32 = &FOO;
  |                      ^^^^
