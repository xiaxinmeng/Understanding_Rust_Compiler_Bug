plain
   Compiling miniz_oxide v0.4.0
   Compiling std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
   Compiling hashbrown v0.11.0
   Compiling addr2line v0.16.0
error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
    |
    |
272 |                 return Ok(NonZeroUsize::new_unchecked(count));
    |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
    |
    = note: consult the function's documentation for information on how to avoid undefined behavior
For more information about this error, try `rustc --explain E0133`.
error: could not compile `std` due to previous error
Build completed unsuccessfully in 0:00:23
