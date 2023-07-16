
error: the constant `N` is not of type `u64`
 --> src/lib.rs:5:26
  |
5 | impl<const N: u64> Q for [u8; N] {}
  |                          ^^^^^^^
  |
note: required for `[u8; N]` to implement `Q`
 --> src/lib.rs:5:20
  |
5 | impl<const N: u64> Q for [u8; N] {}
  |      ------------  ^     ^^^^^^^
  |      |
  |      unsatisfied trait bound introduced here

error: the constant `13` is not of type `u64`
 --> src/lib.rs:7:25
  |
7 | pub fn q_user() -> [u8; <[u8; 13] as Q>::ASSOC] {}
  |                         ^^^^^^^^^^^^^^^^^^^^^^
  |
note: required for `[u8; 13]` to implement `Q`
 --> src/lib.rs:5:20
  |
5 | impl<const N: u64> Q for [u8; N] {}
  |      ------------  ^     ^^^^^^^
  |      |
  |      unsatisfied trait bound introduced here
