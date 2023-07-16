plain
      Memory: 14 GB
      Boot ROM Version: VMW71.00V.13989454.B64.1906190538
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VMvpdEQX3D+/

hw.ncpu: 3
hw.byteorder: 1234
hw.memsize: 15032385536
---
   Compiling addr2line v0.14.0
[RUSTC-TIMING] hashbrown test:false 2.441
[RUSTC-TIMING] addr2line test:false 0.952
[RUSTC-TIMING] object test:false 25.655
error[E0609]: no field `create_pidfd` on type `&mut process_common::Command`
   --> library/std/src/sys/unix/process/process_unix.rs:390:21
    |
390 |             || self.create_pidfd

error: aborting due to previous error

For more information about this error, try `rustc --explain E0609`.
