rust
   Compiling playground v0.0.1 (/playground)
error[E0080]: evaluation of constant value failed
  --> src/main.rs:38:58
   |
38 |     fn split_array<const M: usize>(self) -> ([T; M], [T; N - M]);
   |                                                          ^^^^^ attempt to compute `6_usize - 9000_usize`, which would overflow
