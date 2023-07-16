
➜  BLAKE3 git:(master) ✗ cargo +stage2 build -Z build-std --target x86_64-apple-ios-macabi
   Compiling rustc-std-workspace-core v1.99.0 (/Volumes/SharedVol/rust-projects/rust/build/aarch64-apple-darwin/stage2/lib/rustlib/src/rust/library/rustc-std-workspace-core)
   Compiling blake3 v1.3.1 (/Volumes/SharedVol/rust-projects/BLAKE3)
   Compiling compiler_builtins v0.1.71
   Compiling libc v0.2.121
The following warnings were emitted during compilation:

warning: The C compiler "clang" does not support -mavx512f and -mavx512vl.
warning: clang-13: error: invalid version number in '--target=x86_64-apple-ios13.0-macabi'

error: failed to run custom build command for `blake3 v1.3.1 (/Volumes/SharedVol/rust-projects/BLAKE3)`

Caused by:
  process didn't exit successfully: `/Volumes/SharedVol/rust-projects/BLAKE3/target/debug/build/blake3-b58207ab56af61ae/build-script-build` (exit status: 1)
  --- stdout
  cargo:rustc-env=TARGET=x86_64-apple-ios14.0-macabi
  cargo:rustc-env=IPHONEOS_DEPLOYMENT_TARGET=14.0
  cargo:rerun-if-env-changed=CARGO_FEATURE_PURE
  cargo:rerun-if-env-changed=CARGO_FEATURE_NO_NEON
  TARGET = Some("x86_64-apple-ios-macabi")
  HOST = Some("aarch64-apple-darwin")
  CC_x86_64-apple-ios-macabi = None
  CC_x86_64_apple_ios_macabi = None
  TARGET_CC = None
  CC = None
  CFLAGS_x86_64-apple-ios-macabi = None
  CFLAGS_x86_64_apple_ios_macabi = None
  TARGET_CFLAGS = None
  CFLAGS = None
  CRATE_CC_NO_DEFAULTS = None
  Detecting iOS SDK path for macosx
  running: "xcrun" "--show-sdk-path" "--sdk" "macosx"
  exit status: 0
  OPT_LEVEL = Some("0")
  CC_x86_64-apple-ios-macabi = None
  CC_x86_64_apple_ios_macabi = None
  TARGET_CC = None
  CC = None
  CFLAGS_x86_64-apple-ios-macabi = None
  CFLAGS_x86_64_apple_ios_macabi = None
  TARGET_CFLAGS = None
  CFLAGS = None
  CRATE_CC_NO_DEFAULTS = None
  DEBUG = Some("true")
  Detecting iOS SDK path for macosx
  running: "xcrun" "--show-sdk-path" "--sdk" "macosx"
  exit status: 0
  cargo:warning=The C compiler "clang" does not support -mavx512f and -mavx512vl.
  cargo:rerun-if-env-changed=BLAKE3_CI
  cargo:rerun-if-env-changed=CARGO_FEATURE_PREFER_INTRINSICS
  cargo:rerun-if-env-changed=CARGO_FEATURE_PURE
  cargo:rustc-cfg=blake3_sse2_ffi
  cargo:rustc-cfg=blake3_sse41_ffi
  cargo:rustc-cfg=blake3_avx2_ffi
  TARGET = Some("x86_64-apple-ios-macabi")
  OPT_LEVEL = Some("0")
  HOST = Some("aarch64-apple-darwin")
  CC_x86_64-apple-ios-macabi = None
  CC_x86_64_apple_ios_macabi = None
  TARGET_CC = None
  CC = None
  CFLAGS_x86_64-apple-ios-macabi = None
  CFLAGS_x86_64_apple_ios_macabi = None
  TARGET_CFLAGS = None
  CFLAGS = None
  CRATE_CC_NO_DEFAULTS = None
  DEBUG = Some("true")
  Detecting iOS SDK path for macosx
  running: "xcrun" "--show-sdk-path" "--sdk" "macosx"
  exit status: 0
  running: "clang" "-O0" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-apple-ios13.0-macabi" "-isysroot" "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX12.3.sdk" "-fembed-bitcode" "-Wall" "-Wextra" "-std=c11" "-o" "/Volumes/SharedVol/rust-projects/BLAKE3/target/x86_64-apple-ios-macabi/debug/build/blake3-5c91a7166ed285cc/out/c/blake3_sse2_x86-64_unix.o" "-c" "c/blake3_sse2_x86-64_unix.S"
  cargo:warning=clang-13: error: invalid version number in '--target=x86_64-apple-ios13.0-macabi'
  exit status: 1

  --- stderr


  error occurred: Command "clang" "-O0" "-fPIC" "-g" "-fno-omit-frame-pointer" "--target=x86_64-apple-ios13.0-macabi" "-isysroot" "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX12.3.sdk" "-fembed-bitcode" "-Wall" "-Wextra" "-std=c11" "-o" "/Volumes/SharedVol/rust-projects/BLAKE3/target/x86_64-apple-ios-macabi/debug/build/blake3-5c91a7166ed285cc/out/c/blake3_sse2_x86-64_unix.o" "-c" "c/blake3_sse2_x86-64_unix.S" with args "clang" did not execute successfully (status code exit status: 1).


warning: build failed, waiting for other jobs to finish...
