rust
error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
  --> ...\test.rs:50:58
   |
50 |     pub const FIRST: Self = unsafe { Self::new_unchecked(Ti::from_usize(0)) };
   |                                                          ^^^^^^^^^^^^^^^^^

error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
  --> ...\test.rs:51:57
   |
51 |     pub const LAST: Self = unsafe { Self::new_unchecked(Ti::from_usize(N - 1)) };
   |                                                         ^^^^^^^^^^^^^^^^^^^^^
