plain
    Checking addr2line v0.16.0
error[E0412]: cannot find type `UnsafeCell` in this scope
  --> library/std/src/thread/scoped.rs:44:18
   |
44 |     main_thread: UnsafeCell<Thread>,
   |
help: consider importing one of these items
   |
1  | use core::cell::UnsafeCell;
---
    |                          |         |
    |                          |         cannot infer type for type parameter `U`
    |                          this method call resolves to `T`
    |
    = note: cannot satisfy `_: ~const core::convert::From<thread::Thread>`
    = note: required because of the requirements on the impl of `core::convert::Into<_>` for `thread::Thread`
Some errors have detailed explanations: E0283, E0412.
For more information about an error, try `rustc --explain E0283`.
error: could not compile `std` due to 2 previous errors
Build completed unsuccessfully in 0:01:58
