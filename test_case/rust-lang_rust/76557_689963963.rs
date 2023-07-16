
error[E0308]: mismatched types
  --> library/std/src/sys/unix/ext/ucred.rs:65:17
   |
65 |                 &mut ucred_size,
   |                 ^^^^^^^^^^^^^^^ expected `i32`, found `u32`
   |
   = note:    expected raw pointer `*mut i32`
           found mutable reference `&mut u32`
