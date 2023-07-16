plain
   Compiling addr2line v0.16.0
error[E0423]: expected value, found built-in attribute `path`
    --> library/std/src/sys/unix/fs.rs:1426:21
     |
1426 |     let path = cstr(path)?;

error[E0308]: mismatched types
   --> library/std/src/os/./unix/fs.rs:972:21
    |
    |
972 |     sys::fs::fchown(fd.as_fd(), uid.unwrap_or(u32::MAX), gid.unwrap_or(u32::MAX))
    |                     ^^^^^^^^^^ expected `i32`, found struct `owned::BorrowedFd`
Some errors have detailed explanations: E0308, E0423.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `std` due to 2 previous errors
Build completed unsuccessfully in 0:00:24
