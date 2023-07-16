plain
      System Firmware Version: VMW71.00V.13989454.B64.1906190538
      OS Loader Version: 540.120.3~22
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VMJ0E/ZESkmD
      Provisioning UDID: 4203018E-580F-C1B5-9525-B745CECA79EB

hw.ncpu: 12
hw.byteorder: 1234
---
test [rustdoc] tests/rustdoc/wrapping.rs ... ok

failures:

---- [rustdoc] tests/rustdoc/primitive/no_std.rs stdout ----
Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-apple-darwin target=x86_64-apple-darwin
thread '[rustdoc] tests/rustdoc/primitive/no_std.rs' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 22, kind: InvalidInput, message: "Invalid argument" }', src/tools/compiletest/src/runtest.rs:130:43


failures:
    [rustdoc] tests/rustdoc/primitive/no_std.rs
