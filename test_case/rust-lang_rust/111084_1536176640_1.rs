text
warning: unreachable expression (snip)

error[E0308]: mismatched types
 --> src/lib.rs:3:51
  |
3 | const S: &'static str = {unreachable_unchecked(); 5};
  |                                                   ^ expected `&str`, found integer

note: erroneous constant used
 --> src/lib.rs:4:33
  |
4 | pub const BYTES: &[u8] = &[0; { S.len() }];
  |                                 ^

For more information about this error, try `rustc --explain E0308`.
