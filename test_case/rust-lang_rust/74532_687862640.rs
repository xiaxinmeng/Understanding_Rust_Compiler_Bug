plain
      Memory: 12 GB
      Boot ROM Version: VMW71.00V.13989454.B64.1906190538
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VMqeK5Gvke5K

hw.ncpu: 4
hw.byteorder: 1234
hw.memsize: 12884901888
---
   Compiling profiler_builtins v0.0.0 (/Users/runner/work/rust/rust/library/profiler_builtins)
[RUSTC-TIMING] build_script_build test:false 0.377
[RUSTC-TIMING] build_script_build test:false 0.413
[RUSTC-TIMING] build_script_build test:false 0.675
error[E0527]: pattern requires 0 elements but array has 8
     |
1160 | / macro_rules! atomic_int {
1160 | / macro_rules! atomic_int {
1161 | |     ($cfg_cas:meta,
1162 | |      $stable:meta,
1163 | |      $stable_cxchg:meta,
...    |
1311 | |                     let [] = [(); align_of::<Self>() - align_of::<$int_type>()];
     | |                         ^^ expected 8 elements
1957 | |     }
1958 | | }
1958 | | }
     | |_- in this expansion of `atomic_int!`
...
2107 | / atomic_int! {
2108 | |     cfg(target_has_atomic = "128"),
2109 | |     unstable(feature = "integer_atomics", issue = "32976"),
2110 | |     unstable(feature = "integer_atomics", issue = "32976"),
...    |
2122 | |     i128 AtomicI128 ATOMIC_I128_INIT
     | |_- in this macro invocation


error[E0527]: pattern requires 0 elements but array has 8
     |
1160 | / macro_rules! atomic_int {
1160 | / macro_rules! atomic_int {
1161 | |     ($cfg_cas:meta,
1162 | |      $stable:meta,
1163 | |      $stable_cxchg:meta,
...    |
1311 | |                     let [] = [(); align_of::<Self>() - align_of::<$int_type>()];
     | |                         ^^ expected 8 elements
1957 | |     }
1958 | | }
1958 | | }
     | |_- in this expansion of `atomic_int!`
...
2125 | / atomic_int! {
2126 | |     cfg(target_has_atomic = "128"),
2127 | |     unstable(feature = "integer_atomics", issue = "32976"),
2128 | |     unstable(feature = "integer_atomics", issue = "32976"),
...    |
2140 | |     u128 AtomicU128 ATOMIC_U128_INIT
     | |_- in this macro invocation

error: aborting due to 2 previous errors


For more information about this error, try `rustc --explain E0527`.
[RUSTC-TIMING] core test:false 16.078
error: could not compile `core`.

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0/bin/cargo" "build" "--target" "x86_64-apple-darwin" "-Zbinary-dep-depinfo" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace profiler compiler-builtins-c" "--manifest-path" "/Users/runner/work/rust/rust/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /Users/runner/work/rust/rust/build/bootstrap/debug/bootstrap --stage 2 test
Build completed unsuccessfully in 0:02:20
== clock drift check ==
  local time: Sun Sep  6 18:25:03 UTC 2020
  local time: Sun Sep  6 18:25:03 UTC 2020
  network time: Sun, 06 Sep 2020 18:25:03 GMT
== end clock drift check ==
##[error]Process completed with exit code 1.
Terminate orphan process: pid (2734) (sccache)
Terminate orphan process: pid (1827) (Python)
