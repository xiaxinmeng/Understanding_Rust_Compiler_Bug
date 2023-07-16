plain
      Memory: 14 GB
      System Firmware Version: VMW71.00V.13989454.B64.1906190538
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VMFWbEUgByxp
      Provisioning UDID: 4203018E-580F-C1B5-9525-B745CECA79EB

hw.ncpu: 3
hw.byteorder: 1234
---
hw.tbfrequency: 1000000000
hw.use_kernelmanagerd: 1
Building rustbuild
    Finished dev [unoptimized] target(s) in 0.56s
[TIMING] Mingw { host: aarch64-apple-darwin } -- 0.000
[TIMING] Assemble { target_compiler: Compiler { stage: 0, host: x86_64-apple-darwin } } -- 0.000
[TIMING] StartupObjects { compiler: Compiler { stage: 0, host: x86_64-apple-darwin }, target: x86_64-apple-darwin } -- 0.000
[TIMING] Sysroot { compiler: Compiler { stage: 0, host: x86_64-apple-darwin } } -- 0.001
[TIMING] Libdir { compiler: Compiler { stage: 0, host: x86_64-apple-darwin }, target: x86_64-apple-darwin } -- 0.000
---
   Compiling addr2line v0.16.0
[RUSTC-TIMING] addr2line test:false 1.090
[RUSTC-TIMING] gimli test:false 15.127
[RUSTC-TIMING] object test:false 12.579
error[E0412]: cannot find type `off64_t` in crate `libc`
   --> library/std/src/sys/unix/fd.rs:112:33
112 |                 offset as libc::off64_t,
    |                                 ^^^^^^^ help: a type alias with a similar name exists: `off_t`
    |
   ::: /Users/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/unix/bsd/mod.rs:1:1
   ::: /Users/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/unix/bsd/mod.rs:1:1
    |
1   | pub type off_t = i64;
    | --------------------- similarly named type alias `off_t` defined here

error[E0412]: cannot find type `off64_t` in crate `libc`
   --> library/std/src/sys/unix/fd.rs:179:33
179 |                 offset as libc::off64_t,
    |                                 ^^^^^^^ help: a type alias with a similar name exists: `off_t`
    |
   ::: /Users/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/unix/bsd/mod.rs:1:1
   ::: /Users/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/unix/bsd/mod.rs:1:1
    |
1   | pub type off_t = i64;
    | --------------------- similarly named type alias `off_t` defined here

error[E0412]: cannot find type `off64_t` in crate `libc`
    |
    |
969 |         let n = cvt(unsafe { lseek64(self.as_raw_fd(), pos as libc::off64_t, whence) })?;
    |                                                                     ^^^^^^^ help: a type alias with a similar name exists: `off_t`
   ::: /Users/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/unix/bsd/mod.rs:1:1
    |
1   | pub type off_t = i64;
1   | pub type off_t = i64;
    | --------------------- similarly named type alias `off_t` defined here
For more information about this error, try `rustc --explain E0412`.
[RUSTC-TIMING] std test:false 3.502
error: could not compile `std` due to 3 previous errors
Build completed unsuccessfully in 0:01:25
