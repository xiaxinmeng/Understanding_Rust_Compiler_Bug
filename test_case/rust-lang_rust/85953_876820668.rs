plain
      Memory: 14 GB
      Boot ROM Version: VMW71.00V.13989454.B64.1906190538
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VMo2E9lMIjAT

hw.ncpu: 3
hw.byteorder: 1234
hw.memsize: 15032385536
---
[RUSTC-TIMING] gimli test:false 14.114
[RUSTC-TIMING] object test:false 18.326
warning: dropping unsupported crate type `dylib` for target `aarch64-apple-ios`

error: cannot find macro `syscall` in this scope
     |
1297 |     syscall! {
     |     ^^^^^^^
     |
     |
     = note: consider importing this macro:
             crate::sys::weak::syscall

error[E0425]: cannot find function `fclonefileat` in this scope
     |
     |
1313 |             cvt(unsafe { fclonefileat(reader.as_raw_fd(), libc::AT_FDCWD, to.as_ptr(), 0) });

error: aborting due to 2 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0425`.
