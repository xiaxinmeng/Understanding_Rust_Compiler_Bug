
rust git:(master) ✗ RUST_BACKTRACE=1 cargo test src/test/ui/parser
   Compiling rustc_codegen_llvm v0.0.0 (/Users/robinboehm/development/rust/rust/src/librustc_codegen_llvm)
   Compiling libc v0.2.54
   Compiling compiler_builtins v0.1.15
   Compiling profiler_builtins v0.0.0 (/Users/robinboehm/development/rust/rust/src/libprofiler_builtins)
   Compiling strip-ansi-escapes v0.1.0
error: failed to run custom build command for `profiler_builtins v0.0.0 (/Users/robinboehm/development/rust/rust/src/libprofiler_builtins)`

Caused by:
  process didn't exit successfully: `/Users/robinboehm/development/rust/rust/target/debug/build/profiler_builtins-48a5ce79ce969ba5/build-script-build` (exit code: 101)
--- stderr
thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', src/libcore/option.rs:347:21
stack backtrace:
   0: <unknown>
   1: <unknown>
   2: <unknown>
   3: <unknown>
   4: <unknown>
   5: <unknown>
   6: <unknown>
   7: <unknown>
   8: <unknown>
   9: <unknown>
  10: <unknown>
  11: <unknown>
  12: <unknown>
  13: <unknown>
  14: <unknown>

warning: build failed, waiting for other jobs to finish...
error: build failed
