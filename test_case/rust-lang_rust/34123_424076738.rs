
rror[E0658]: intrinsics are subject to change
 --> src/main.rs:3:17
  |
3 |     let assign: unsafe extern "rust-intrinsic" fn(*const i32, *mut i32, usize) = copy;
  |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error

For more information about this error, try `rustc --explain E0658
