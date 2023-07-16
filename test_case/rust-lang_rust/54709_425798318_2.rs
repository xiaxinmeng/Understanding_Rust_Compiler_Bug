console
error[E0658]: casting pointers to integers in statics is unstable (see issue #51910)
 --> src/lib.rs:7:29
  |
7 | pub static ADDRESS: usize = &VALUE as *const usize as usize;
  |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = help: add #![feature(const_raw_ptr_to_usize_cast)] to the crate attributes to enable
