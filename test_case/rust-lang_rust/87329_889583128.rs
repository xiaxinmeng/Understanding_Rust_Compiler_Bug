plain
[RUSTC-TIMING] getrandom test:false 0.463
[RUSTC-TIMING] getopts test:false 3.916
[RUSTC-TIMING] typenum test:false 1.236
[RUSTC-TIMING] ryu test:false 0.698
error: failed to run custom build command for `psm v0.1.11`
Caused by:
Caused by:
  process didn't exit successfully: `D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage1-rustc\release\build\psm-3899b44976ac068b\build-script-build` (exit code: 1)
  --- stdout
  cargo:rustc-cfg=asm
  OPT_LEVEL = Some("3")
  TARGET = Some("x86_64-pc-windows-msvc")
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
  running: "C:\\Program Files (x86)\\Microsoft Visual Studio\\2019\\Enterprise\\VC\\Tools\\MSVC\\14.29.30037\\bin\\HostX64\\x64\\ml64.exe" "-nologo" "-FoD:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-rustc\\x86_64-pc-windows-msvc\\release\\build\\psm-ae7318fab6fe448f\\out\\src/arch/x86_64_msvc.o" "-c" "src/arch/x86_64_msvc.asm"
   Assembling: src/arch/x86_64_msvc.asm
  --- stderr



  error occurred: Command "C:\\Program Files (x86)\\Microsoft Visual Studio\\2019\\Enterprise\\VC\\Tools\\MSVC\\14.29.30037\\bin\\HostX64\\x64\\lib.exe" "-out:D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-rustc\\x86_64-pc-windows-msvc\\release\\build\\psm-ae7318fab6fe448f\\out\\libpsm_s.a" "-nologo" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-rustc\\x86_64-pc-windows-msvc\\release\\build\\psm-ae7318fab6fe448f\\out\\src/arch/x86_64_msvc.o" with args "lib.exe" failed to start.

warning: build failed, waiting for other jobs to finish...
[RUSTC-TIMING] crc32fast test:false 0.543
[RUSTC-TIMING] snap test:false 2.210
