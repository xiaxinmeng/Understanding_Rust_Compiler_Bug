
error[E0308]: mismatched types
 --> src/lib.rs:5:43
  |
5 |     fn fun(x: [u8; Self::C]) -> [u8; 0] { x }
  |                                 -------   ^ expected `0usize`, found `Self::C`
  |                                 |
  |                                 expected `[u8; 0]` because of return type
  |
  = note: expected array `[u8; 0]`
             found array `[u8; _]`
