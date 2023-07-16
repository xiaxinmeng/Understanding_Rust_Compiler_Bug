
error[E0080]: evaluation of constant value failed
 --> src/lib.rs:3:30
  |
3 |     let u16_slice = unsafe { core::slice::from_raw_parts(u8_slice.as_ptr().cast::<u16>(), 2) };
  |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'pointer must be aligned', src/lib.rs:3:30
