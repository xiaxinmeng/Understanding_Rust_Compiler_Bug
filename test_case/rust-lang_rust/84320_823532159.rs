plain
The following warnings were emitted during compilation:

warning: error: Connection to server timed out

error: failed to run custom build command for `libnghttp2-sys v0.1.4+1.41.0`
Caused by:
Caused by:
  process didn't exit successfully: `/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage1-tools/release/build/libnghttp2-sys-3cd71264f3ae175a/build-script-build` (exit code: 1)
  --- stdout
  TARGET = Some("x86_64-apple-darwin")
  OPT_LEVEL = Some("3")
  HOST = Some("x86_64-apple-darwin")
  CC_x86_64-apple-darwin = Some("sccache /Users/runner/work/rust/rust/clang+llvm-10.0.0-x86_64-apple-darwin/bin/clang")
  CFLAGS_x86_64-apple-darwin = Some("-ffunction-sections -fdata-sections -fPIC --target=x86_64-apple-darwin -stdlib=libc++ -fdebug-prefix-map=/Users/runner/work/rust/rust=/rustc/6627380fa4763ecfcf00b2d9d853196239184a3b")
  CRATE_CC_NO_DEFAULTS = None
  DEBUG = Some("false")
  running: "sccache" "/Users/runner/work/rust/rust/clang+llvm-10.0.0-x86_64-apple-darwin/bin/clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-apple-darwin" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-apple-darwin" "-stdlib=libc++" "-fdebug-prefix-map=/Users/runner/work/rust/rust=/rustc/6627380fa4763ecfcf00b2d9d853196239184a3b" "-I" "nghttp2/lib/includes" "-I" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage1-tools/x86_64-apple-darwin/release/build/libnghttp2-sys-fd806f3caba039ff/out/i/include" "-DNGHTTP2_STATICLIB" "-DHAVE_NETINET_IN" "-DHAVE_ARPA_INET_H" "-o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage1-tools/x86_64-apple-darwin/release/build/libnghttp2-sys-fd806f3caba039ff/out/i/lib/nghttp2/lib/nghttp2_buf.o" "-c" "nghttp2/lib/nghttp2_buf.c"
  cargo:warning=error: Connection to server timed out
  exit status: 2
  running: "sccache" "/Users/runner/work/rust/rust/clang+llvm-10.0.0-x86_64-apple-darwin/bin/clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-apple-darwin" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-apple-darwin" "-stdlib=libc++" "-fdebug-prefix-map=/Users/runner/work/rust/rust=/rustc/6627380fa4763ecfcf00b2d9d853196239184a3b" "-I" "nghttp2/lib/includes" "-I" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage1-tools/x86_64-apple-darwin/release/build/libnghttp2-sys-fd806f3caba039ff/out/i/include" "-DNGHTTP2_STATICLIB" "-DHAVE_NETINET_IN" "-DHAVE_ARPA_INET_H" "-o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage1-tools/x86_64-apple-darwin/release/build/libnghttp2-sys-fd806f3caba039ff/out/i/lib/nghttp2/lib/nghttp2_callbacks.o" "-c" "nghttp2/lib/nghttp2_callbacks.c"

  --- stderr



  error occurred: Command "sccache" "/Users/runner/work/rust/rust/clang+llvm-10.0.0-x86_64-apple-darwin/bin/clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-apple-darwin" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-apple-darwin" "-stdlib=libc++" "-fdebug-prefix-map=/Users/runner/work/rust/rust=/rustc/6627380fa4763ecfcf00b2d9d853196239184a3b" "-I" "nghttp2/lib/includes" "-I" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage1-tools/x86_64-apple-darwin/release/build/libnghttp2-sys-fd806f3caba039ff/out/i/include" "-DNGHTTP2_STATICLIB" "-DHAVE_NETINET_IN" "-DHAVE_ARPA_INET_H" "-o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage1-tools/x86_64-apple-darwin/release/build/libnghttp2-sys-fd806f3caba039ff/out/i/lib/nghttp2/lib/nghttp2_buf.o" "-c" "nghttp2/lib/nghttp2_buf.c" with args "clang" did not execute successfully (status code exit status: 2).

warning: build failed, waiting for other jobs to finish...
[RUSTC-TIMING] flate2 test:false 2.544
error: build failed
