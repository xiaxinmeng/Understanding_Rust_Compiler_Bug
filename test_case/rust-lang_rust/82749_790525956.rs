plain
      Memory: 14 GB
      Boot ROM Version: VMW71.00V.13989454.B64.1906190538
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VMGEEw7V1nrE

hw.ncpu: 3
hw.byteorder: 1234
hw.memsize: 15032385536
---
test sync::mpsc::tests::stress_recv_timeout_two_threads ... ok

failures:

---- sys::unix::process::process_inner::tests::exitstatus_display_tests stdout ----
thread 'sys::unix::process::process_inner::tests::exitstatus_display_tests' panicked at 'assertion failed: `(left == right)`
  left: `"stopped (not terminated) by signal: 19"`,
 right: `"continued (WIFCONTINUED)"`', library/std/src/sys/unix/process/process_unix/tests.rs:8:20

failures:
    sys::unix::process::process_inner::tests::exitstatus_display_tests


test result: FAILED. 837 passed; 1 failed; 1 ignored; 0 measured; 0 filtered out; finished in 15.63s

error: test failed, to rerun pass '-p std --lib'


command did not execute successfully: "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0/bin/cargo" "test" "--target" "x86_64-apple-darwin" "-Zbinary-dep-depinfo" "-j" "3" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace profiler compiler-builtins-c" "--manifest-path" "/Users/runner/work/rust/rust/library/test/Cargo.toml" "-p" "std" "--"


failed to run: /Users/runner/work/rust/rust/build/bootstrap/debug/bootstrap --stage 2 test
Build completed unsuccessfully in 2:13:18
