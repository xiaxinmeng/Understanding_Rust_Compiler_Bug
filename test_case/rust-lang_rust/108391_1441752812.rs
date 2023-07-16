plain
    Checking addr2line v0.17.0
error: unused import: `Align8`
 --> library/std/src/sys/windows/io.rs:5:21
  |
5 | use crate::sys::{c, Align8};
  |
  |
  = note: `-D unused-imports` implied by `-D warnings`
error: unused import: `core`
 --> library/std/src/sys/windows/io.rs:6:5
  |
6 | use core;
---
    |
545 | pub struct FILE_NAME_INFO {
    |            ^^^^^^^^^^^^^^
    |
    = note: `-D dead-code` implied by `-D warnings`
error: could not compile `std` due to 3 previous errors
Build completed unsuccessfully in 0:00:24
