
$ rustc +stage1 54376.rs
error[E0668]: malformed inline assembly
 --> 54376.rs:4:14
  |
4 |     unsafe { asm!("callq $0" : : "0"(foo)) };
  |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error

For more information about this error, try `rustc --explain E0668`.
