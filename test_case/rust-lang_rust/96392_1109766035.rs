
# project: https://github.com/BLAKE3-team/BLAKE3.git
$ cargo +stage2 build -Z build-std --release --target x86_64-apple-ios-macabi

   Compiling rustc-std-workspace-core v1.99.0 (/Volumes/SharedVol/rust-projects/rust/build/aarch64-apple-darwin/stage2/lib/rustlib/src/rust/library/rustc-std-workspace-core)
   Compiling blake3 v1.3.1 (/Volumes/SharedVol/rust-projects/BLAKE3)
The following warnings were emitted during compilation:

warning: The C compiler "clang" does not support -mavx512f and -mavx512vl.
warning: clang-13: error: invalid version number in '--target=x86_64-apple-ios13.0-macabi'

error: failed to run custom build command for `blake3 v1.3.1 (/Volumes/SharedVol/rust-projects/BLAKE3)`

Caused by:
  process didn't exit successfully: `/Volumes/SharedVol/rust-projects/BLAKE3/target/release/build/blake3-ac5c6dee5cd36567/build-script-build` (exit status: 1)
  --- stdout
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
  OPT_LEVEL = Some("3")
  CC_x86_64-apple-ios-macabi = None
  CC_x86_64_apple_ios_macabi = None
  TARGET_CC = None
  CC = None
  CFLAGS_x86_64-apple-ios-macabi = None
  CFLAGS_x86_64_apple_ios_macabi = None
  TARGET_CFLAGS = None
  CFLAGS = None
  CRATE_CC_NO_DEFAULTS = None
  DEBUG = Some("false")
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
  OPT_LEVEL = Some("3")
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
  DEBUG = Some("false")
  Detecting iOS SDK path for macosx
  running: "xcrun" "--show-sdk-path" "--sdk" "macosx"
  exit status: 0
  running: "clang" "-O3" "-fPIC" "--target=x86_64-apple-ios13.0-macabi" "-isysroot" "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX12.3.sdk" "-fembed-bitcode" "-Wall" "-Wextra" "-std=c11" "-o" "/Volumes/SharedVol/rust-projects/BLAKE3/target/x86_64-apple-ios-macabi/release/build/blake3-4be9cc78c36efcbd/out/c/blake3_sse2_x86-64_unix.o" "-c" "c/blake3_sse2_x86-64_unix.S"
  cargo:warning=clang-13: error: invalid version number in '--target=x86_64-apple-ios13.0-macabi'
  exit status: 1

  --- stderr


  error occurred: Command "clang" "-O3" "-fPIC" "--target=x86_64-apple-ios13.0-macabi" "-isysroot" "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX12.3.sdk" "-fembed-bitcode" "-Wall" "-Wextra" "-std=c11" "-o" "/Volumes/SharedVol/rust-projects/BLAKE3/target/x86_64-apple-ios-macabi/release/build/blake3-4be9cc78c36efcbd/out/c/blake3_sse2_x86-64_unix.o" "-c" "c/blake3_sse2_x86-64_unix.S" with args "clang" did not execute successfully (status code exit status: 1).

