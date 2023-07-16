plain
   Compiling addr2line v0.16.0
error: unused import: `read`
   --> library/std/src/sys/unix/thread.rs:381:21
    |
381 |     use crate::fs::{read, try_exists, File};
    |
    |
    = note: `-D unused-imports` implied by `-D warnings`
error: could not compile `std` due to previous error
Build completed unsuccessfully in 0:00:18
