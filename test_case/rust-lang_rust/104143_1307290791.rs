plain
[RUSTC-TIMING] annotate_snippets test:false 1.025
   Compiling psm v0.1.16
The following warnings were emitted during compilation:

warning: The MSVC ARM assemblers do not support -D flags
error: failed to run custom build command for `psm v0.1.16`

Caused by:
  process didn't exit successfully: `D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage1-rustc\release\build\psm-c0ca29d29f3e8391\build-script-build` (exit code: 1)
  process didn't exit successfully: `D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage1-rustc\release\build\psm-c0ca29d29f3e8391\build-script-build` (exit code: 1)
  --- stdout
  OPT_LEVEL = Some("3")
  TARGET = Some("aarch64-pc-windows-msvc")
  HOST = Some("x86_64-pc-windows-msvc")
  cargo:rerun-if-env-changed=CC_aarch64-pc-windows-msvc
  CC_aarch64-pc-windows-msvc = None
  cargo:rerun-if-env-changed=CC_aarch64_pc_windows_msvc
  CC_aarch64_pc_windows_msvc = None
  cargo:rerun-if-env-changed=TARGET_CC
  TARGET_CC = None
  cargo:rerun-if-env-changed=CC
  CC = Some("D:/a/rust/rust/citools/clang-rust/bin/clang-cl.exe")
  cargo:rerun-if-env-changed=CFLAGS_aarch64-pc-windows-msvc
  CFLAGS_aarch64-pc-windows-msvc = None
  cargo:rerun-if-env-changed=CFLAGS_aarch64_pc_windows_msvc
  CFLAGS_aarch64_pc_windows_msvc = None
  cargo:rerun-if-env-changed=TARGET_CFLAGS
  TARGET_CFLAGS = None
  cargo:rerun-if-env-changed=CFLAGS
  CFLAGS = None
  cargo:rerun-if-env-changed=CRATE_CC_NO_DEFAULTS
  CARGO_CFG_TARGET_FEATURE = Some("crt-static,llvm14-builtins-abi,neon")
  DEBUG = Some("false")
  DEBUG = Some("false")
  cargo:rustc-cfg=asm
  cargo:warning=The MSVC ARM assemblers do not support -D flags
  running: "C:\\Program Files (x86)\\Microsoft Visual Studio\\2019\\Enterprise\\VC\\Tools\\MSVC\\14.29.30133\\bin\\HostX64\\arm64\\armasm64.exe" "-nologo" "-o" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-rustc\\aarch64-pc-windows-msvc\\release\\build\\psm-995d162fe6f0b1f5\\out\\src/arch/aarch64_armasm.o" "--" "src/arch/aarch64_armasm.asm"
  error A2029: unknown command-line argument or argument value --

   Usage:      armasm [<options>] sourcefile objectfile
               armasm [<options>] -o objectfile sourcefile
               armasm -h              for help
  exit code: 1

  --- stderr



  error occurred: Command "C:\\Program Files (x86)\\Microsoft Visual Studio\\2019\\Enterprise\\VC\\Tools\\MSVC\\14.29.30133\\bin\\HostX64\\arm64\\armasm64.exe" "-nologo" "-o" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage1-rustc\\aarch64-pc-windows-msvc\\release\\build\\psm-995d162fe6f0b1f5\\out\\src/arch/aarch64_armasm.o" "--" "src/arch/aarch64_armasm.asm" with args "armasm64.exe" did not execute successfully (status code exit code: 1).

warning: build failed, waiting for other jobs to finish...
[RUSTC-TIMING] indexmap test:false 0.819
[RUSTC-TIMING] ppv_lite86 test:false 0.519
