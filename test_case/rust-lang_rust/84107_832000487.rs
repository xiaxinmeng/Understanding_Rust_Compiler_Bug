plain
   Compiling compiler_builtins v0.1.42
   Compiling libc v0.2.93
The following warnings were emitted during compilation:

warning: <unknown>:0: error: this directive must appear between .cfi_startproc and .cfi_endproc directives
error: failed to run custom build command for `compiler_builtins v0.1.42`

Caused by:
Caused by:
  process didn't exit successfully: `D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage1-std\release\build\compiler_builtins-e092a17c00e598c0\build-script-build` (exit code: 1)
  --- stdout
  cargo:rerun-if-changed=build.rs
  cargo:compiler-rt=C:\Users\runneradmin\.cargo\registry\src\github.com-1ecc6299db9ec823\compiler_builtins-0.1.42\compiler-rt
  cargo:rustc-cfg=feature="unstable"
  cargo:rerun-if-changed=D:\a\rust\rust\src/llvm-project/compiler-rt\lib/builtins\aarch64/lse.S
  TARGET = Some("aarch64-pc-windows-msvc")
  OPT_LEVEL = Some("3")
  HOST = Some("x86_64-pc-windows-msvc")
  CC_aarch64-pc-windows-msvc = None
  CC_aarch64_pc_windows_msvc = None
  TARGET_CC = None
  CC = Some("D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe")
  CFLAGS_aarch64-pc-windows-msvc = None
  CFLAGS_aarch64_pc_windows_msvc = None
  TARGET_CFLAGS = None
  CRATE_CC_NO_DEFAULTS = None
  CRATE_CC_NO_DEFAULTS = None
  CARGO_CFG_TARGET_FEATURE = Some("crt-static,fp,neon")
  DEBUG = Some("true")
  CC_aarch64-pc-windows-msvc = None
  CC_aarch64_pc_windows_msvc = None
  TARGET_CC = None
  CC = Some("D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe")
  CFLAGS_aarch64-pc-windows-msvc = None
  CFLAGS_aarch64_pc_windows_msvc = None
  TARGET_CFLAGS = None
  CRATE_CC_NO_DEFAULTS = None
  CRATE_CC_NO_DEFAULTS = None
  CARGO_CFG_TARGET_FEATURE = Some("crt-static,fp,neon")
  running: "D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe" "-nologo" "-MT" "-O2" "-Z7" "-Brepro" "--target=aarch64-pc-windows-msvc" "-I" "D:\\a\\rust\\rust\\src/llvm-project/compiler-rt\\lib/builtins" "/Zl" "-D__func__=__FUNCTION__" "-DL_cas" "-DSIZE=1" "-DMODEL=1" "-FoD:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-std\\aarch64-pc-windows-msvc\\release\\build\\compiler_builtins-a42a19e3623e0aeb\\out\\lse.o" "-c" "D:\\a\\rust\\rust\\src/llvm-project/compiler-rt\\lib/builtins\\aarch64/lse.S"
  cargo:warning=<unknown>:0: error: this directive must appear between .cfi_startproc and .cfi_endproc directives

  --- stderr



  error occurred: Command "D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe" "-nologo" "-MT" "-O2" "-Z7" "-Brepro" "--target=aarch64-pc-windows-msvc" "-I" "D:\\a\\rust\\rust\\src/llvm-project/compiler-rt\\lib/builtins" "/Zl" "-D__func__=__FUNCTION__" "-DL_cas" "-DSIZE=1" "-DMODEL=1" "-FoD:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-std\\aarch64-pc-windows-msvc\\release\\build\\compiler_builtins-a42a19e3623e0aeb\\out\\lse.o" "-c" "D:\\a\\rust\\rust\\src/llvm-project/compiler-rt\\lib/builtins\\aarch64/lse.S" with args "clang-cl.exe" did not execute successfully (status code exit code: 1).

warning: build failed, waiting for other jobs to finish...
[RUSTC-TIMING] core test:false 22.221
error: build failed
error: build failed
command did not execute successfully: "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "build" "--target" "aarch64-pc-windows-msvc" "-Zbinary-dep-depinfo" "-j" "8" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace profiler compiler-builtins-c" "--manifest-path" "D:\\a\\rust\\rust\\library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: D:\a\rust\rust\build\bootstrap\debug\bootstrap dist
Build completed unsuccessfully in 0:29:07
