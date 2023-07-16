rust
error[E0080]: evaluation of constant value failed
  --> src/main.rs:11:19
   |
11 |     0 => unsafe { ptr_offset_from(dest as *const u8, origin as *const u8) as usize },
   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                   |
   |                   ptr_offset_from cannot compute offset of pointers into different allocations.
