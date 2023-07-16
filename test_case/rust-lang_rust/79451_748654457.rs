plain
      Memory: 14 GB
      Boot ROM Version: VMW71.00V.13989454.B64.1906190538
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VMiWaqwTYYq7

hw.ncpu: 3
hw.byteorder: 1234
hw.memsize: 15032385536
---
[RUSTC-TIMING] build_script_build test:false 0.473
[RUSTC-TIMING] build_script_build test:false 0.578
The following warnings were emitted during compilation:

warning: ar: no archive members specified
warning: usage:  ar -d [-TLsv] archive file ...
warning:  ar -m [-TLsv] archive file ...
warning:  ar -m [-abiTLsv] position archive file ...
warning:  ar -p [-TLsv] archive [file ...]
warning:  ar -q [-cTLsv] archive file ...
warning:  ar -r [-cuTLsv] archive file ...
warning:  ar -r [-abciuTLsv] position archive file ...
warning:  ar -t [-TLsv] archive [file ...]
warning:  ar -x [-ouTLsv] archive [file ...]
error: failed to run custom build command for `profiler_builtins v0.0.0 (/Users/runner/work/rust/rust/library/profiler_builtins)`

Caused by:
Caused by:
  process didn't exit successfully: `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-std/release/build/profiler_builtins-1ff4284c8fc0d194/build-script-build` (exit code: 1)
  --- stdout
  TARGET = Some("x86_64-apple-darwin")
  HOST = Some("x86_64-apple-darwin")
  AR_x86_64-apple-darwin = Some("ar")
  running: "ar" "crs" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-std/x86_64-apple-darwin/release/build/profiler_builtins-5ec280d6e6f2145b/out/libprofiler-rt.a"
  cargo:warning=ar: no archive members specified
  cargo:warning=usage:  ar -d [-TLsv] archive file ...
  cargo:warning= ar -m [-TLsv] archive file ...
  cargo:warning= ar -m [-abiTLsv] position archive file ...
  cargo:warning= ar -p [-TLsv] archive [file ...]
  cargo:warning= ar -q [-cTLsv] archive file ...
  cargo:warning= ar -r [-cuTLsv] archive file ...
  cargo:warning= ar -r [-abciuTLsv] position archive file ...
  cargo:warning= ar -t [-TLsv] archive [file ...]
  cargo:warning= ar -x [-ouTLsv] archive [file ...]

  --- stderr



  error occurred: Command "ar" "crs" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-std/x86_64-apple-darwin/release/build/profiler_builtins-5ec280d6e6f2145b/out/libprofiler-rt.a" with args "ar" did not execute successfully (status code exit code: 1).

warning: build failed, waiting for other jobs to finish...
[RUSTC-TIMING] core test:false 68.505
error: build failed
