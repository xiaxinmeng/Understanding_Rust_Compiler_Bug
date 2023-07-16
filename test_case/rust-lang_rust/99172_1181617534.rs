plain
      Memory: 14 GB
      System Firmware Version: VMW71.00V.13989454.B64.1906190538
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VMTt3exNRuKi
      Provisioning UDID: 4203018E-580F-C1B5-9525-B745CECA79EB

hw.ncpu: 3
hw.byteorder: 1234
---
failures:

---- [run-pass-valgrind] src/test/run-pass-valgrind/cast-enum-with-dtor.rs stdout ----

error: compilation failed!
status: signal: 9 (SIGKILL)
Some tests failed in compiletest suite=run-pass-valgrind mode=run-pass-valgrind host=x86_64-apple-darwin target=x86_64-apple-darwin
command: "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/bin/rustc" "/Users/runner/work/rust/rust/src/test/run-pass-valgrind/cast-enum-with-dtor.rs" "-Zthreads=1" "--target=x86_64-apple-darwin" "-O" "-C" "prefer-dynamic" "-o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-pass-valgrind/cast-enum-with-dtor/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/Users/runner/work/rust/rust/build/x86_64-apple-darwin/native/rust-test-helpers" "-L" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/run-pass-valgrind/cast-enum-with-dtor/auxiliary"
stdout: none
stderr: none


failures:
    [run-pass-valgrind] src/test/run-pass-valgrind/cast-enum-with-dtor.rs
