plain
   Compiling addr2line v0.14.0
error[E0433]: failed to resolve: use of undeclared crate or module `io`
   --> library/std/src/sys/unix/thread.rs:364:67
    |
364 |                 panic!("failed to allocate a guard page: {}", io::Error::last_os_error());
    |                                                                   ^^^^^ not found in `io`
help: consider importing one of these items
    |
231 |     use alloc::fmt::Error;
    |
---

error[E0433]: failed to resolve: use of undeclared crate or module `io`
   --> library/std/src/sys/unix/thread.rs:369:68
    |
369 |                 panic!("failed to protect the guard page: {}", io::Error::last_os_error());
    |                                                                    ^^^^^ not found in `io`
help: consider importing one of these items
    |
231 |     use alloc::fmt::Error;
    |
