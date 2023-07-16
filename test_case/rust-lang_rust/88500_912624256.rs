plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.49
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: unused imports: `RefCell`, `UnsafeCell`, `future::Future`, `marker::PhantomPinned`, `mem`, `ptr`
    |
    |
729 |     cell::{RefCell, UnsafeCell},
730 |     future::Future,
    |     ^^^^^^^^^^^^^^
731 |     marker::PhantomPinned,
    |     ^^^^^^^^^^^^^^^^^^^^^
    |     ^^^^^^^^^^^^^^^^^^^^^
732 |     mem, ptr,
    |     ^^^  ^^^
    |
    = note: `-D unused-imports` implied by `-D warnings`
error: could not compile `core` due to previous error
Build completed unsuccessfully in 0:00:08
