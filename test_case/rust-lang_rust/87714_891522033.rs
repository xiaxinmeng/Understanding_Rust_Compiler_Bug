plain
[RUSTC-TIMING] getrandom test:false 0.302
[RUSTC-TIMING] proc_macro2 test:false 1.149
[RUSTC-TIMING] getopts test:false 2.816
[RUSTC-TIMING] getrandom test:false 0.334
error: failed to run custom build command for `psm v0.1.14`
Caused by:
Caused by:
  process didn't exit successfully: `D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0-rustc\release\build\psm-acef9b08e5c009b2\build-script-build` (exit code: 1)
  OPT_LEVEL = Some("3")
  TARGET = Some("x86_64-pc-windows-msvc")
  HOST = Some("x86_64-pc-windows-msvc")
  HOST = Some("x86_64-pc-windows-msvc")
  CC_x86_64-pc-windows-msvc = None
  CC_x86_64_pc_windows_msvc = None
  HOST_CC = None
  CC = Some("D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe")
  CFLAGS_x86_64-pc-windows-msvc = None
  CFLAGS_x86_64_pc_windows_msvc = None
  HOST_CFLAGS = None
  CRATE_CC_NO_DEFAULTS = None
  CRATE_CC_NO_DEFAULTS = None
  CARGO_CFG_TARGET_FEATURE = Some("crt-static,fxsr,sse,sse2")
  DEBUG = Some("false")
  cargo:rustc-cfg=asm
  running: "clang" "-O3" "-ffunction-sections" "-fdata-sections" "--target=x86_64-pc-windows-msvc" "-Wall" "-Wextra" "-xassembler-with-cpp" "-DCFG_TARGET_OS_windows" "-DCFG_TARGET_ARCH_x86_64" "-DCFG_TARGET_ENV_msvc" "-o" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0-rustc\\x86_64-pc-windows-msvc\\release\\build\\psm-a238117c39c7b175\\out\\src/arch/x86_64_windows_gnu.o" "-c" "src/arch/x86_64_windows_gnu.s"
  --- stderr



  error occurred: Failed to find tool. Is `clang` installed? (see https://github.com/alexcrichton/cc-rs#compile-time-requirements for help)

warning: build failed, waiting for other jobs to finish...
[RUSTC-TIMING] ryu test:false 0.504
[RUSTC-TIMING] typenum test:false 1.066
