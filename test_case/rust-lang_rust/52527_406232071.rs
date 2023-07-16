
[00:50:32] error[E0396]: raw pointers cannot be dereferenced in statics
[00:50:32]   --> /checkout/src/test/run-pass/const-block.rs:42:57
[00:50:32]    |
[00:50:32] 42 | static BLOCK_UNSAFE_SAFE_PTR: &'static isize = unsafe { &*(0xdeadbeef as *const isize) };
[00:50:32]    |                                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ dereference of raw pointer in constant
[00:50:32] 
[00:50:32] error[E0396]: raw pointers cannot be dereferenced in statics
[00:50:32]   --> /checkout/src/test/run-pass/const-block.rs:45:5
[00:50:32]    |
[00:50:32] 45 |     &*X
[00:50:32]    |     ^^^ dereference of raw pointer in constant
[00:50:32] 
[00:50:32] error: aborting due to 2 previous errors
[00:50:32] 
[00:50:32] For more information about this error, try `rustc --explain E0396`.
