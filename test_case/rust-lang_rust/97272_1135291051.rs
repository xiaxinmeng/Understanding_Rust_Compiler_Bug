plain
      Memory: 14 GB
      System Firmware Version: VMW71.00V.13989454.B64.1906190538
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VMzEydMr2vHT
      Provisioning UDID: 4203018E-580F-C1B5-9525-B745CECA79EB

hw.ncpu: 3
hw.byteorder: 1234
---
failures:

---- [ui] src/test/ui/abi/abi-sysv64-register-usage.rs stdout ----

error: test compilation failed although it shouldn't!
status: signal: 9
command: "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/bin/rustc" "/Users/runner/work/rust/rust/src/test/ui/abi/abi-sysv64-register-usage.rs" "-Zthreads=1" "--target=x86_64-apple-darwin" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/ui/abi/abi-sysv64-register-usage/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/Users/runner/work/rust/rust/build/x86_64-apple-darwin/native/rust-test-helpers" "-L" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/ui/abi/abi-sysv64-register-usage/auxiliary"
stdout: none
stderr: none


failures:
    [ui] src/test/ui/abi/abi-sysv64-register-usage.rs
