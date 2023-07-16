
warning: attempt to divide with overflow
 --> test.rs:2:20
  |
2 |     println!("{}", -128i8 / -1);
  |                    ^^^^^^^^^^^
  |
  = note: #[warn(const_err)] on by default

warning: this expression will panic at run-time
 --> test.rs:2:20
  |
2 |     println!("{}", -128i8 / -1);
  |                    ^^^^^^^^^^^ attempt to divide with overflow

