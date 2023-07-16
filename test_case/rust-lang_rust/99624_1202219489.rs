plain
      Memory: 14 GB
      System Firmware Version: VMW71.00V.13989454.B64.1906190538
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VM9S2kCLwKOc
      Provisioning UDID: 4203018E-580F-C1B5-9525-B745CECA79EB

hw.ncpu: 3
hw.byteorder: 1234
---
[RUSTC-TIMING] object test:false 10.984
error[E0433]: failed to resolve: use of undeclared crate or module `std`
   --> library/std/src/sys/unix/locks/pthread_condvar.rs:175:61
    |
175 |         assert_eq!(r, 0, "unexpected error: {:?}", std::io::Error::last_os_error());
    |                                                             ^^^^^ not found in `std::io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Error;
    |
    |
1   | use core::fmt::Error;
    |
1   | use crate::error::Error;
    |
1   | use crate::fmt::Error;
    |
      and 2 other candidates
help: if you import `Error`, refer to it directly
    |
175 -         assert_eq!(r, 0, "unexpected error: {:?}", std::io::Error::last_os_error());
175 +         assert_eq!(r, 0, "unexpected error: {:?}", Error::last_os_error());

For more information about this error, try `rustc --explain E0433`.
[RUSTC-TIMING] std test:false 3.628
error: could not compile `std` due to previous error
