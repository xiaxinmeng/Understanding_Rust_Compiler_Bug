plain
      L3 Cache: 12 MB
      Memory: 30 GB
      System Firmware Version: 1.00
      OS Loader Version: 540.120.3~22
      Serial Number (system): C02J5DDCG1HW
      Hardware UUID: 7D97AE97-2EA9-49FC-9EE3-C892277F44C7
      Provisioning UDID: 7D97AE97-2EA9-49FC-9EE3-C892277F44C7
hw.ncpu: 12
hw.byteorder: 1234
hw.memsize: 32212254720
hw.activecpu: 12
---
Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-apple-darwin target=x86_64-apple-darwin

failures:

---- [rustdoc] tests/rustdoc/primitive/no_std.rs stdout ----
thread '[rustdoc] tests/rustdoc/primitive/no_std.rs' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 22, kind: InvalidInput, message: "Invalid argument" }', src/tools/compiletest/src/runtest.rs:130:43


failures:
    [rustdoc] tests/rustdoc/primitive/no_std.rs
