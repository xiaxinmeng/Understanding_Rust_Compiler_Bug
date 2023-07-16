
error: attempt to negate with overflow
 --> src/lib.rs:8:20
  |
8 |     const N: i32 = -i32::MIN + T::N;
  |                    ^^^^^^^^^
  |
  = note: `#[deny(const_err)]` on by default

error: this expression will panic at runtime
 --> src/lib.rs:8:20
  |
8 |     const N: i32 = -i32::MIN + T::N;
  |                    ^^^^^^^^^ attempt to negate with overflow
