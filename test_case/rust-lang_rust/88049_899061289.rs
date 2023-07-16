plain
   Compiling libc v0.2.99
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.49
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0606]: casting `*mut T` as `*mut U` is invalid
    |
    |
387 |         unsafe { NonNull::new_unchecked(self.as_ptr() as *mut U) }
    |
    |
    = note: vtable kinds may not match

error[E0606]: casting `*mut T` as `*mut U` is invalid
   --> library/core/src/ptr/unique.rs:139:40
139 |         unsafe { Unique::new_unchecked(self.as_ptr() as *mut U) }
    |                                        ^^^^^^^^^^^^^^^^^^^^^^^
    |
    |
    = note: vtable kinds may not match

error[E0606]: casting `*const T` as `*const U` is invalid
   |
48 |         self as _
   |         ^^^^^^^^^
   |
   |
   = note: vtable kinds may not match

error[E0606]: casting `*mut T` as `*mut U` is invalid
   |
47 |         self as _
   |         ^^^^^^^^^
   |
   |
   = note: vtable kinds may not match
For more information about this error, try `rustc --explain E0606`.
error: could not compile `core` due to 4 previous errors
Build completed unsuccessfully in 0:01:12
