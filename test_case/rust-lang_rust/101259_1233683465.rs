plain
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.79
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: `ptr::metadata::from_raw_parts` is not yet stable as a const fn
   |
48 |         from_raw_parts(self as *const (), ())
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   |
   = help: const-stable functions can only call other const-stable functions

error: `ptr::metadata::from_raw_parts_mut` is not yet stable as a const fn
  --> library/core/src/ptr/mut_ptr.rs:47:9
   |
47 |         from_raw_parts_mut(self as *mut (), ())
   |
   = help: const-stable functions can only call other const-stable functions

error: could not compile `core` due to 2 previous errors
