plain
........................................................................................  792/1132
.................................................................................iii....  880/1132
........................................................................................  968/1132
.............................................................................iiiiii..... 1056/1132
.......................F.F..................i...............................
failures:

---- src/thread/mod.rs - thread::Thread::from_raw (line 1346) stdout ----
error[E0658]: use of unstable library feature 'thread_pointer_conversion'
---
   |                                          ^^^^^^^^
   |
   = help: add `#![feature(thread_pointer_conversion)]` to the crate attributes to enable

error[E0277]: `*const ()` cannot be sent between threads safely
    |
    |
9   | let handle = thread::spawn(move || {
    |              ^^^^^^^^^^^^^ `*const ()` cannot be sent between threads safely
    |
    = help: within `(*const (), *const ())`, the trait `Send` is not implemented for `*const ()`
    = note: required because it appears within the type `(*const (), *const ())`
note: required by a bound in `spawn`
    |
680 |     T: Send + 'static,
    |        ^^^^ required by this bound in `spawn`

---
   |                        ^^^^^^^^^^^^^^^^
   |
   = help: add `#![feature(thread_pointer_conversion)]` to the crate attributes to enable

error[E0369]: binary operation `==` cannot be applied to type `Thread`
   |
   |
20 |     assert_ne!(main_thread, child_thread);
   |     |
   |     Thread
   |     Thread
   |
   |
   = note: this error originates in the macro `assert_ne` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0369]: binary operation `==` cannot be applied to type `Thread`
   |
21 |     assert_eq!(main_thread, thread::current());
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |     |
---
    src/thread/mod.rs - thread::Thread::into_raw (line 1316)

test result: FAILED. 1112 passed; 2 failed; 18 ignored; 0 measured; 0 filtered out; finished in 11.61s

error: doctest failed, to rerun pass `-p std --doc`
