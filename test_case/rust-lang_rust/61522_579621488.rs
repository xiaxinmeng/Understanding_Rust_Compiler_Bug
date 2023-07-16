
error: constant expression depends on a generic parameter
 --> src/main.rs:3:40
  |
3 | pub struct MyArray<const COUNT: usize>([u8; COUNT + 1]);
  |                                        ^^^^^^^^^^^^^^^
  |
  = note: this may fail depending on what value the parameter takes

error: constant expression depends on a generic parameter
 --> src/main.rs:6:5
  |
6 | /     fn inner(&self) -> &[u8; COUNT + 1] {
7 | |         &self.0
8 | |     }
  | |_____^
  |
  = note: this may fail depending on what value the parameter takes
