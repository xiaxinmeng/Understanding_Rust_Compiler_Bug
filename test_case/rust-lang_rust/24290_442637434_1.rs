
warning: literal out of range for isize
 --> src/lib.rs:2:10
  |
2 |     V1 = 0xFFFF_FFFF_FFFF_FFFF,
  |          ^^^^^^^^^^^^^^^^^^^^^
  |
  = note: #[warn(overflowing_literals)] on by default
  = note: the literal `0xFFFF_FFFF_FFFF_FFFF` (decimal `18446744073709551615`) does not fit into an `isize` and will become `-1isize`
