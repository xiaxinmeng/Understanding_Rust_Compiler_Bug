
error[E0606]: casting `&mut [i32; 2]` as `*mut i32` is invalid
  --> test.rs:13:32
   |
13 |         let my_num: *mut i32 = my_num as *mut i32;
   |                                ^^^^^^^^^^^^^^^^^^

error: aborting due to previous error
