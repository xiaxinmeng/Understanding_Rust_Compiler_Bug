
error: failed to run custom build command for `profiler_builtins v0.0.0 (/Users/adam/code/rust/library/profiler_builtins)`

Caused by:
  process didn't exit successfully: `/Users/adam/code/rust/build/aarch64-apple-darwin/stage1-std/release/build/profiler_builtins-91d370bd947d85af/build-script-build` (exit code: 1)
  --- stdout
  TARGET = Some("wasm32-unknown-unknown")
  OPT_LEVEL = Some("3")
  HOST = Some("aarch64-apple-darwin")
  CC_wasm32-unknown-unknown = Some("clang")
  CFLAGS_wasm32-unknown-unknown = Some("-ffunction-sections -fdata-sections -fPIC --target=wasm32-unknown-unknown")
  CRATE_CC_NO_DEFAULTS = None
  DEBUG = Some("false")
  running: "clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=wasm32-unknown-unknown" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=wasm32-unknown-unknown" "-I" "../../src/llvm-project/compiler-rt/include" "-fno-builtin" "-fomit-frame-pointer" "-fvisibility=hidden" "-DVISIBILITY_HIDDEN" "-DCOMPILER_RT_HAS_UNAME=1" "-DCOMPILER_RT_HAS_ATOMICS=1" "-o" "/Users/adam/code/rust/build/aarch64-apple-darwin/stage1-std/wasm32-unknown-unknown/release/build/profiler_builtins-614c077fb1787a99/out/../../src/llvm-project/compiler-rt/lib/profile/GCDAProfiling.o" "-c" "../../src/llvm-project/compiler-rt/lib/profile/GCDAProfiling.c"
  cargo:warning=../../src/llvm-project/compiler-rt/lib/profile/GCDAProfiling.c:24:10: fatal error: 'errno.h' file not found
  cargo:warning=#include <errno.h>
  cargo:warning=         ^~~~~~~~~
  cargo:warning=1 error generated.
  exit code: 1

  --- stderr


  error occurred: Command "clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=wasm32-unknown-unknown" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=wasm32-unknown-unknown" "-I" "../../src/llvm-project/compiler-rt/include" "-fno-builtin" "-fomit-frame-pointer" "-fvisibility=hidden" "-DVISIBILITY_HIDDEN" "-DCOMPILER_RT_HAS_UNAME=1" "-DCOMPILER_RT_HAS_ATOMICS=1" "-o" "/Users/adam/code/rust/build/aarch64-apple-darwin/stage1-std/wasm32-unknown-unknown/release/build/profiler_builtins-614c077fb1787a99/out/../../src/llvm-project/compiler-rt/lib/profile/GCDAProfiling.o" "-c" "../../src/llvm-project/compiler-rt/lib/profile/GCDAProfiling.c" with args "clang" did not execute successfully (status code exit code: 1).
