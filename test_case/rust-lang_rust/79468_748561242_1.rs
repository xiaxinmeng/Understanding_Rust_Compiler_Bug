
error[E0080]: values of the type `[u8; 2305843009213693951]` are too big for the current architecture
 --> big.rs:3:1
  |
3 | static MY_TOO_BIG_ARRAY_2: [u8; HUGE_SIZE] = [0x00; HUGE_SIZE];
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
