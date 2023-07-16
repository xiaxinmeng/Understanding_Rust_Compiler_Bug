plain

error: failed to run custom build command for `libz-sys v1.1.2`

Caused by:
  process didn't exit successfully: `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage1-tools/release/build/libz-sys-57710a7f3f3c39a1/build-script-build` (exit status: 1)
  --- stdout
  cargo:rerun-if-env-changed=LIBZ_SYS_STATIC
  cargo:rerun-if-changed=build.rs
  TARGET = Some("x86_64-apple-darwin")
  OPT_LEVEL = Some("0")
  HOST = Some("x86_64-apple-darwin")
  CC_x86_64-apple-darwin = Some("sccache /Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang")
  CFLAGS_x86_64-apple-darwin = Some("-ffunction-sections -fdata-sections -fPIC --target=x86_64-apple-darwin -stdlib=libc++ -fdebug-prefix-map=/Users/runner/work/rust/rust=/rustc/f86834050e98d3cd49834d98733b595b59646ca9")
  CRATE_CC_NO_DEFAULTS = None
  DEBUG = Some("false")
  running: "sccache" "/Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang" "-O0" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-apple-darwin" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-apple-darwin" "-stdlib=libc++" "-fdebug-prefix-map=/Users/runner/work/rust/rust=/rustc/f86834050e98d3cd49834d98733b595b59646ca9" "-I" "src/zlib" "-fvisibility=hidden" "-DSTDC" "-D_LARGEFILE64_SOURCE" "-D_POSIX_SOURCE" "-D_C99_SOURCE" "-o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage1-tools/release/build/libz-sys-25738435a1dfcb3d/out/build/src/zlib/adler32.o" "-c" "src/zlib/adler32.c"
  cargo:warning=error: Connection to server timed out
  exit status: 2
  running: "sccache" "/Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang" "-O0" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-apple-darwin" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-apple-darwin" "-stdlib=libc++" "-fdebug-prefix-map=/Users/runner/work/rust/rust=/rustc/f86834050e98d3cd49834d98733b595b59646ca9" "-I" "src/zlib" "-fvisibility=hidden" "-DSTDC" "-D_LARGEFILE64_SOURCE" "-D_POSIX_SOURCE" "-D_C99_SOURCE" "-o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage1-tools/release/build/libz-sys-25738435a1dfcb3d/out/build/src/zlib/compress.o" "-c" "src/zlib/compress.c"

  --- stderr



  error occurred: Command "sccache" "/Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang" "-O0" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-apple-darwin" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-apple-darwin" "-stdlib=libc++" "-fdebug-prefix-map=/Users/runner/work/rust/rust=/rustc/f86834050e98d3cd49834d98733b595b59646ca9" "-I" "src/zlib" "-fvisibility=hidden" "-DSTDC" "-D_LARGEFILE64_SOURCE" "-D_POSIX_SOURCE" "-D_C99_SOURCE" "-o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage1-tools/release/build/libz-sys-25738435a1dfcb3d/out/build/src/zlib/adler32.o" "-c" "src/zlib/adler32.c" with args "clang" did not execute successfully (status code exit status: 2).

warning: build failed, waiting for other jobs to finish...
The following warnings were emitted during compilation:

