console
error[E0018]: raw pointers cannot be cast to integers in statics
 --> src/lib.rs:7:29
  |
7 | pub static ADDRESS: usize = &VALUE as *const usize as usize;
  |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
