plain
[RUSTC-TIMING] build_script_build test:false 0.473
[RUSTC-TIMING] build_script_build test:false 1.462
The following warnings were emitted during compilation:

warning: error: /Applications/Xcode_13.1.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/ranlib: can't open file: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-std/x86_64-apple-darwin/release/build/profiler_builtins-e02f38cb938c3431/out/libprofiler-rt.a (No such file or directory)
warning: /Applications/Xcode_13.1.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/ar: internal ranlib command failed
error: failed to run custom build command for `profiler_builtins v0.0.0 (/Users/runner/work/rust/rust/library/profiler_builtins)`

Caused by:
  process didn't exit successfully: `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-std/release/build/profiler_builtins-47d7f900160d7f14/build-script-build` (exit status: 1)
  process didn't exit successfully: `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-std/release/build/profiler_builtins-47d7f900160d7f14/build-script-build` (exit status: 1)
  --- stdout
  TARGET = Some("x86_64-apple-darwin")
  HOST = Some("x86_64-apple-darwin")
  AR_x86_64-apple-darwin = Some("ar")
  running: "ar" "s" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-std/x86_64-apple-darwin/release/build/profiler_builtins-e02f38cb938c3431/out/libprofiler-rt.a"
  cargo:warning=error: /Applications/Xcode_13.1.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/ranlib: can't open file: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-std/x86_64-apple-darwin/release/build/profiler_builtins-e02f38cb938c3431/out/libprofiler-rt.a (No such file or directory)
  cargo:warning=/Applications/Xcode_13.1.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/ar: internal ranlib command failed

  --- stderr



  error occurred: Command "ar" "s" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-std/x86_64-apple-darwin/release/build/profiler_builtins-e02f38cb938c3431/out/libprofiler-rt.a" with args "ar" did not execute successfully (status code exit status: 1).

warning: build failed, waiting for other jobs to finish...
[RUSTC-TIMING] core test:false 33.033
error: build failed
