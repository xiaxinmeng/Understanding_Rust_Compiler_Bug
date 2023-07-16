plain
      Memory: 14 GB
      Boot ROM Version: VMW71.00V.13989454.B64.1906190538
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VMIohYHp8cZ9

hw.ncpu: 3
hw.byteorder: 1234
hw.memsize: 15032385536
---
   Compiling addr2line v0.14.0
[RUSTC-TIMING] addr2line test:false 1.035
[RUSTC-TIMING] gimli test:false 13.204
[RUSTC-TIMING] object test:false 22.330
error: unused imports: `AtomicBool`, `Ordering`
 --> library/std/src/sys/unix/process/process_unix.rs:6:27
6 | use crate::sync::atomic::{AtomicBool, Ordering};
  |                           ^^^^^^^^^^  ^^^^^^^^
  |
  |
  = note: `-D unused-imports` implied by `-D warnings`
error: unused import: `crate::sys_common::FromInner`
  --> library/std/src/sys/unix/process/process_unix.rs:10:5
   |
10 | use crate::sys_common::FromInner;
10 | use crate::sys_common::FromInner;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: unused import: `c_long`
  --> library/std/src/sys/unix/process/process_unix.rs:19:19
   |
19 | use libc::{c_int, c_long, gid_t, pid_t, uid_t};

error: aborting due to 3 previous errors

[RUSTC-TIMING] std test:false 4.802
