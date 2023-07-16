
error: constant expression depends on a generic parameter
 --> src/lib.rs:6:37
  |
6 | pub struct BitArray<const N: usize>(ConstBytes<{(N + 7) / 8}>);
  |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: this may fail depending on what value the parameter takes

error: constant expression depends on a generic parameter
 --> src/lib.rs:9:19
  |
9 |     fn new(bytes: ConstBytes<{(N + 7) / 8}>) -> Self {
  |                   ^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: this may fail depending on what value the parameter takes

error: aborting due to 2 previous errors; 1 warning emitted
