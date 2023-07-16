
[01:04:26] ---- [ui] ui/consts/static_mut_containing_mut_ref2.rs stdout ----
[01:04:26] diff of stderr:
[01:04:26] 
[01:04:26] 4 LL | pub static mut STDERR_BUFFER: () = unsafe { *(&mut STDERR_BUFFER_SPACE) = 42; };
[01:04:26] 5    |                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^ statics require immutable values
[01:04:26] - error: aborting due to previous error
[01:04:26] - error: aborting due to previous error
[01:04:26] + error[E0019]: static contains unimplemented expression type
[01:04:26] +   --> $DIR/static_mut_containing_mut_ref2.rs:5:45
[01:04:26] +    |
[01:04:26] + LL | pub static mut STDERR_BUFFER: () = unsafe { *(&mut STDERR_BUFFER_SPACE) = 42; };
[01:04:26] 8 
[01:04:26] - For more information about this error, try `rustc --explain E0017`.
[01:04:26] + error: aborting due to 2 previous errors
[01:04:26] + 
[01:04:26] + 
[01:04:26] + Some errors occurred: E0017, E0019.
[01:04:26] + For more information about an error, try `rustc --explain E0017`.
