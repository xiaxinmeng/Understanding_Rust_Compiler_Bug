plain
[RUSTC-TIMING] build_script_build test:false 0.351
[RUSTC-TIMING] build_script_build test:false 0.410
The following warnings were emitted during compilation:

warning: /rustroot/bin/llvm-ar: error: unable to load '/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/build/profiler_builtins-90d74d33748e794c/out/libprofiler-rt.a': No such file or directory
error: failed to run custom build command for `profiler_builtins v0.0.0 (/checkout/library/profiler_builtins)`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/build/profiler_builtins-9cfe5cb79c620478/build-script-build` (exit status: 1)
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/build/profiler_builtins-9cfe5cb79c620478/build-script-build` (exit status: 1)
  --- stdout
  TARGET = Some("x86_64-unknown-linux-gnu")
  HOST = Some("x86_64-unknown-linux-gnu")
  AR_x86_64-unknown-linux-gnu = Some("/rustroot/bin/llvm-ar")
  running: "/rustroot/bin/llvm-ar" "s" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/build/profiler_builtins-90d74d33748e794c/out/libprofiler-rt.a"
  cargo:warning=/rustroot/bin/llvm-ar: error: unable to load '/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/build/profiler_builtins-90d74d33748e794c/out/libprofiler-rt.a': No such file or directory

  --- stderr



  error occurred: Command "/rustroot/bin/llvm-ar" "s" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/build/profiler_builtins-90d74d33748e794c/out/libprofiler-rt.a" with args "/rustroot/bin/llvm-ar" did not execute successfully (status code exit status: 1).

warning: build failed, waiting for other jobs to finish...
[RUSTC-TIMING] core test:false 20.478
error: build failed
