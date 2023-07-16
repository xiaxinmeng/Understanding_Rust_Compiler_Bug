plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.71
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: call to unsafe function is unsafe and requires unsafe block (error E0133)
    |
197 |         let ptr = self.alloc(layout);
    |                   ^^^^^^^^^^^^^^^^^^ call to unsafe function
    |
    |
note: the lint level is defined here
   --> library/core/src/lib.rs:91:9
    |
91  | #![deny(unsafe_op_in_unsafe_fn)]
    |         ^^^^^^^^^^^^^^^^^^^^^^
    = note: consult the function's documentation for information on how to avoid undefined behavior

error: call to unsafe function is unsafe and requires unsafe block (error E0133)
    |
201 |             ptr::write_bytes(ptr, 0, size);
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
    |
    |
    = note: consult the function's documentation for information on how to avoid undefined behavior
error: could not compile `core` due to 2 previous errors
Build completed unsuccessfully in 0:00:11
