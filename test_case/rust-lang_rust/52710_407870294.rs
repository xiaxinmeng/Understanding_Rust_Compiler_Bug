
error: cannot concatenate a byte string literal
 --> src/main.rs:1:28
  |
1 | const FOO: &[u8] = concat!(b"foo", b"bar");
  |                            ^^^^^^

error: cannot concatenate a byte string literal
 --> src/main.rs:1:36
  |
1 | const FOO: &[u8] = concat!(b"foo", b"bar");
  |                                    ^^^^^^

error: aborting due to 2 previous errors
