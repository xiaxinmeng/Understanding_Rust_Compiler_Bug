plain
      Memory: 14 GB
      Boot ROM Version: VMW71.00V.13989454.B64.1906190538
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VMuHVPZqhT/q

hw.ncpu: 3
hw.byteorder: 1234
hw.memsize: 15032385536
---

error: failed to run custom build command for `libz-sys v1.1.2`

Caused by:
  process didn't exit successfully: `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage1-tools/release/build/libz-sys-6a8089f3ba64501c/build-script-build` (exit status: 1)
  --- stdout
  cargo:rerun-if-env-changed=LIBZ_SYS_STATIC
  cargo:rerun-if-changed=build.rs
  TARGET = Some("x86_64-apple-darwin")
  OPT_LEVEL = Some("3")
  HOST = Some("x86_64-apple-darwin")
  CC_x86_64-apple-darwin = Some("sccache /Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang")
  CFLAGS_x86_64-apple-darwin = Some("-ffunction-sections -fdata-sections -fPIC --target=x86_64-apple-darwin -stdlib=libc++ -fdebug-prefix-map=/Users/runner/work/rust/rust=/rustc/5b60db2f4cb8a914e0c1041fc66e6a4096377b75")
  CRATE_CC_NO_DEFAULTS = None
  DEBUG = Some("false")
  running: "sccache" "/Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-apple-darwin" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-apple-darwin" "-stdlib=libc++" "-fdebug-prefix-map=/Users/runner/work/rust/rust=/rustc/5b60db2f4cb8a914e0c1041fc66e6a4096377b75" "-I" "src/zlib" "-fvisibility=hidden" "-DSTDC" "-D_LARGEFILE64_SOURCE" "-D_POSIX_SOURCE" "-D_C99_SOURCE" "-o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage1-tools/x86_64-apple-darwin/release/build/libz-sys-9be647c09c509bd3/out/build/src/zlib/adler32.o" "-c" "src/zlib/adler32.c"
  cargo:warning=error: Connection to server timed out
  exit status: 2
  running: "sccache" "/Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-apple-darwin" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-apple-darwin" "-stdlib=libc++" "-fdebug-prefix-map=/Users/runner/work/rust/rust=/rustc/5b60db2f4cb8a914e0c1041fc66e6a4096377b75" "-I" "src/zlib" "-fvisibility=hidden" "-DSTDC" "-D_LARGEFILE64_SOURCE" "-D_POSIX_SOURCE" "-D_C99_SOURCE" "-o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage1-tools/x86_64-apple-darwin/release/build/libz-sys-9be647c09c509bd3/out/build/src/zlib/compress.o" "-c" "src/zlib/compress.c"

  --- stderr



  error occurred: Command "sccache" "/Users/runner/work/rust/rust/clang+llvm-12.0.0-x86_64-apple-darwin/bin/clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-apple-darwin" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-apple-darwin" "-stdlib=libc++" "-fdebug-prefix-map=/Users/runner/work/rust/rust=/rustc/5b60db2f4cb8a914e0c1041fc66e6a4096377b75" "-I" "src/zlib" "-fvisibility=hidden" "-DSTDC" "-D_LARGEFILE64_SOURCE" "-D_POSIX_SOURCE" "-D_C99_SOURCE" "-o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage1-tools/x86_64-apple-darwin/release/build/libz-sys-9be647c09c509bd3/out/build/src/zlib/adler32.o" "-c" "src/zlib/adler32.c" with args "clang" did not execute successfully (status code exit status: 2).

warning: build failed, waiting for other jobs to finish...
[RUSTC-TIMING] syn test:false 85.643
[RUSTC-TIMING] serde test:false 8.814
