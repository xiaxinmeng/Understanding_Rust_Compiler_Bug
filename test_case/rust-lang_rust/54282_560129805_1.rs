
~\tmp> rustc --cap-lints=allow --cap-lints=deny foo.rs
~\tmp> rustc --cap-lints=deny --cap-lints=allow foo.rs
warning: unused bitwise operation that must be used
 --> foo.rs:2:5
  |
2 |     100u8 << 10;
  |     ^^^^^^^^^^^
  |
  = note: `#[warn(unused_must_use)]` on by default

error: attempt to shift left with overflow
 --> foo.rs:2:5
  |
2 |     100u8 << 10;
  |     ^^^^^^^^^^^
  |
  = note: `#[deny(exceeding_bitshifts)]` on by default
