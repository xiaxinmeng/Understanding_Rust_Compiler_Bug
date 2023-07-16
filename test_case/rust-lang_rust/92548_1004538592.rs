plain
    Checking rustc-main v0.0.0 (/checkout/compiler/rustc)
error[E0432]: unresolved import `tikv_jemallocator`
  --> compiler/rustc/src/main.rs:29:5
   |
29 | use tikv_jemallocator::Jemalloc;
   |     ^^^^^^^^^^^^^^^^^ use of undeclared crate or module `tikv_jemallocator`
error: unused import: `GlobalAlloc`
  --> compiler/rustc/src/main.rs:28:18
   |
28 | use std::alloc::{GlobalAlloc, Layout};
28 | use std::alloc::{GlobalAlloc, Layout};
   |                  ^^^^^^^^^^^
   |
   = note: `-D unused-imports` implied by `-D warnings`
For more information about this error, try `rustc --explain E0432`.
error: could not compile `rustc-main` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed
