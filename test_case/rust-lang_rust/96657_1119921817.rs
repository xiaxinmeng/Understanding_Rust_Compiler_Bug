plain
      Memory: 14 GB
      System Firmware Version: VMW71.00V.13989454.B64.1906190538
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VMLCOZa9lFTM
      Provisioning UDID: 4203018E-580F-C1B5-9525-B745CECA79EB

hw.ncpu: 3
hw.byteorder: 1234
---
   Compiling addr2line v0.16.0
[RUSTC-TIMING] gimli test:false 15.666
[RUSTC-TIMING] addr2line test:false 0.978
[RUSTC-TIMING] object test:false 10.237
error[E0599]: no method named `to_timespec` found for struct `Timespec` in the current scope
   |
   |
83 |         now.checked_add_duration(&dur).and_then(|t| t.to_timespec()).unwrap_or(TIMESPEC_MAX);
   |                                                       ^^^^^^^^^^^ help: there is an associated function with a similar name: `sub_timespec`
  ::: library/std/src/sys/unix/time.rs:9:1
   |
   |
9  | pub(in crate::sys::unix) struct Timespec {
   | ---------------------------------------- method `to_timespec` not found for this
For more information about this error, try `rustc --explain E0599`.
[RUSTC-TIMING] std test:false 3.770
error: could not compile `std` due to previous error
Build completed unsuccessfully in 0:01:05
