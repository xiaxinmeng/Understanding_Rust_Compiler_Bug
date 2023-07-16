 rust
src/libstd/sys/unix/mutex.rs:58:60: 58:86 error: unresolved name `libc::PTHREAD_MUTEX_NORMAL` [E0425]
src/libstd/sys/unix/mutex.rs:58         let r = libc::pthread_mutexattr_settype(&mut attr, libc::PTHREAD_MUTEX_NORMAL);
                                                                                           ^~~~~~~~~~~~~~~~~~~~~~~~~~
