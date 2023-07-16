plain
   Compiling hashbrown v0.9.0
   Compiling object v0.22.0
   Compiling miniz_oxide v0.4.0
   Compiling addr2line v0.14.0
error[E0425]: cannot find value `piffd` in this scope
   --> library/std/src/sys/unix/process/process_unix.rs:527:22
    |
527 |         let pidfd = (piffd >= 0).then(|| PidFd::from_inner(sys::fd::FileDesc::new(pidfd)));
    |                      ^^^^^ help: a local variable with a similar name exists: `pidfd`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0425`.
error: could not compile `std`
